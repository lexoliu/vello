// Copyright 2025 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Tests for dynamic bump-buffer sizing: buffers sized far below a scene's
//! demand must grow via the `BumpAllocators` feedback, and the frame must
//! still render completely — never dropped or truncated.

use vello::{
    AaConfig, BumpBufferSizes, RenderParams, Renderer, RendererOptions, Scene,
    kurbo::{Affine, BezPath, Circle, Point, Stroke},
    peniko::color::palette,
    util::RenderContext,
    wgpu::{
        self, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d, MapMode,
        TexelCopyBufferInfo, TextureDescriptor, TextureUsages,
    },
};
use vello_research_tests::{TestParams, get_scene_image};

/// A scene with enough paths and segments to overflow a deliberately tiny
/// bump-buffer pool at this resolution.
fn dense_scene() -> Scene {
    let mut scene = Scene::new();
    let stroke = Stroke::new(1.5);
    // ~5,000 stroked circles spread over the target: tens of thousands of
    // path segments and heavy bin/tile coverage.
    for row in 0..50 {
        for col in 0..100 {
            let cx = (col * 8 + 4) as f64;
            let cy = (row * 8 + 4) as f64;
            let circle = Circle::new(Point::new(cx, cy), 3.5);
            scene.stroke(
                &stroke,
                Affine::IDENTITY,
                palette::css::REBECCA_PURPLE,
                None,
                &circle,
            );
        }
    }
    // A long single path to push the `lines`/curve buffers too.
    let mut path = BezPath::new();
    path.move_to(Point::new(0.0, 400.0));
    for i in 0..10_000 {
        let x = (i as f64) / 10_000.0 * 800.0;
        path.line_to(Point::new(x, 400.0 - (i % 97) as f64));
    }
    scene.stroke(
        &Stroke::new(1.0),
        Affine::IDENTITY,
        palette::css::LIME,
        None,
        &path,
    );
    scene
}

/// Minimum-size bump pool: far below `dense_scene`'s demand, so every
/// bump-allocated buffer must be grown through the feedback path.
fn tiny_buffers() -> BumpBufferSizes {
    BumpBufferSizes {
        lines: 64,
        bin_data: 128,
        tiles: 64,
        seg_counts: 64,
        segments: 64,
        blend_spill: 64,
        ptcl: 128,
    }
}

/// A frame whose initial bump buffers are too small must not come out
/// truncated: after growth it must equal the same scene rendered with the
/// default sizing.
#[test]
#[cfg_attr(skip_gpu_tests, ignore)]
fn overflow_renders_completely() {
    let scene = dense_scene();

    let mut grown_params = TestParams::new("buffer_growth_overflow", 800, 400);
    grown_params.buffer_sizes = Some(tiny_buffers());
    let grown = pollster::block_on(get_scene_image(&grown_params, &scene))
        .expect("render with undersized buffers");

    let default_params = TestParams::new("buffer_growth_overflow", 800, 400);
    let reference = pollster::block_on(get_scene_image(&default_params, &scene))
        .expect("render with default buffers");

    // The reference frame must contain content; otherwise the equality below
    // compares two empty frames.
    let drawn_pixels = reference
        .data
        .data()
        .as_chunks::<4>()
        .0
        .iter()
        .filter(|px| px[..3].iter().any(|c| *c > 16))
        .count();
    assert!(
        drawn_pixels > 10_000,
        "reference render is nearly blank ({drawn_pixels} drawn pixels)"
    );

    assert_eq!(
        grown.data.as_ref(),
        reference.data.as_ref(),
        "undersized bump buffers produced a truncated frame"
    );
}

/// A renderer plus its render target, kept alive across frames so buffer
/// growth — and the absence of further waits — is observable mid-run.
struct PersistentGpu {
    device: wgpu::Device,
    queue: wgpu::Queue,
    renderer: Renderer,
    target: wgpu::Texture,
    view: wgpu::TextureView,
    params: RenderParams,
}

impl PersistentGpu {
    fn new(width: u32, height: u32, buffer_sizes: Option<BumpBufferSizes>) -> Self {
        let mut context = RenderContext::new();
        let device_id =
            pollster::block_on(context.device(None)).expect("no compatible device found");
        let handle = &context.devices[device_id];
        let device = handle.device.clone();
        let queue = handle.queue.clone();
        let renderer = Renderer::new(
            &device,
            RendererOptions {
                use_cpu: false,
                num_init_threads: std::num::NonZeroUsize::new(1),
                antialiasing_support: std::iter::once(AaConfig::Area).collect(),
                pipeline_cache: None,
                buffer_sizes,
            },
        )
        .expect("failed to create renderer");
        let target = device.create_texture(&TextureDescriptor {
            label: Some("buffer_growth_mid_run target"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: TextureUsages::STORAGE_BINDING | TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = target.create_view(&wgpu::TextureViewDescriptor::default());
        Self {
            device,
            queue,
            renderer,
            target,
            view,
            params: RenderParams {
                base_color: palette::css::BLACK,
                width,
                height,
                antialiasing_method: AaConfig::Area,
            },
        }
    }

    /// Render `scene` and drain the deferred verification, re-rendering on
    /// overflow. Returns how many overflow rounds the frame needed — zero
    /// means the frame verified without any GPU-wait or re-render.
    fn render_and_verify(&mut self, scene: &Scene) -> u32 {
        let mut readbacks: Vec<vello::BumpReadback> = self
            .renderer
            .render_to_texture(&self.device, &self.queue, scene, &self.view, &self.params)
            .expect("render failed")
            .into_iter()
            .collect();
        for rounds in 0..16 {
            let overflowed = self
                .renderer
                .verify_bump_readbacks(&self.device, readbacks)
                .expect("verification failed");
            if overflowed.is_empty() {
                return rounds;
            }
            readbacks = Vec::new();
            for _index in overflowed {
                readbacks.extend(
                    self.renderer
                        .render_to_texture(
                            &self.device,
                            &self.queue,
                            scene,
                            &self.view,
                            &self.params,
                        )
                        .expect("re-render failed"),
                );
            }
        }
        panic!("bump buffers failed to converge after 16 growth rounds");
    }

    /// RGBA8 contents of the render target.
    fn read_target(&self) -> Vec<u8> {
        let (width, height) = (self.params.width, self.params.height);
        let padded_byte_width = (width * 4).next_multiple_of(256);
        let buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("buffer_growth_mid_run readback"),
            size: padded_byte_width as u64 * height as u64,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("buffer_growth_mid_run readback"),
            });
        encoder.copy_texture_to_buffer(
            self.target.as_image_copy(),
            TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded_byte_width),
                    rows_per_image: None,
                },
            },
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
        self.queue.submit(Some(encoder.finish()));
        let slice = buffer.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        slice.map_async(MapMode::Read, move |v| sender.send(v).unwrap());
        vello::util::block_on_wgpu(&self.device, receiver.receive());
        let mapped = slice.get_mapped_range().unwrap();
        mapped.to_vec()
    }
}

/// When the scene grows past the grown capacity mid-run, the grown frame must
/// still be rendered completely, and the frames after it must verify without
/// another overflow (i.e. no GPU wait, no re-render) — growth is absorbed
/// once, not paid per frame.
#[test]
#[cfg_attr(skip_gpu_tests, ignore)]
fn growth_mid_run_completes_then_settles() {
    let sparse = {
        let mut scene = Scene::new();
        scene.fill(
            vello::peniko::Fill::NonZero,
            Affine::IDENTITY,
            palette::css::REBECCA_PURPLE,
            None,
            &vello::kurbo::Rect::new(100.0, 100.0, 300.0, 200.0),
        );
        scene
    };
    let dense = dense_scene();

    let mut gpu = PersistentGpu::new(800, 400, Some(tiny_buffers()));
    // Frame 1: the sparse scene verifies without touching the grown path.
    gpu.render_and_verify(&sparse);
    // Frame 2: the scene grew past capacity mid-run — the overflow is caught,
    // buffers grow, and the frame is re-rendered before it could be sampled.
    let growth_rounds = gpu.render_and_verify(&dense);
    assert!(
        growth_rounds > 0,
        "dense scene over tiny buffers did not trigger a growth round"
    );
    let grown_frame = gpu.read_target();
    // Frames 3+: same dense scene verifies clean — the grown sizes held, so
    // verification resolves without a GPU wait and nothing is re-rendered.
    for frame in 0..2 {
        let rounds = gpu.render_and_verify(&dense);
        assert_eq!(
            rounds, 0,
            "frame {frame} after growth still triggered a growth round"
        );
    }

    let mut reference_gpu = PersistentGpu::new(800, 400, None);
    reference_gpu.render_and_verify(&dense);
    assert_eq!(
        grown_frame,
        reference_gpu.read_target(),
        "mid-run growth produced a truncated frame"
    );
}
