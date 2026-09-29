// Copyright 2025 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Tests for dynamic bump-buffer sizing: buffers sized far below a scene's
//! demand must grow via the `BumpAllocators` feedback, and the frame must
//! still render completely — never dropped or truncated.

use vello::{
    BumpBufferSizes, Scene,
    kurbo::{Affine, BezPath, Circle, Point, Stroke},
    peniko::color::palette,
};
use vello_tests::{TestParams, get_scene_image};

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
