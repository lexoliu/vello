// Copyright 2023 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::SegmentCount;

use super::{
    BinHeader, Clip, ClipBbox, ClipBic, ClipElement, DrawBbox, DrawMonoid, Layout, LineSoup, Path,
    PathBbox, PathMonoid, PathSegment, Tile,
};
use bytemuck::{Pod, Zeroable};

const TILE_WIDTH: u32 = 16;
const TILE_HEIGHT: u32 = 16;

// TODO: Obtain these from the vello_shaders crate
pub(crate) const PATH_REDUCE_WG: u32 = 256;
const PATH_BBOX_WG: u32 = 256;
const FLATTEN_WG: u32 = 256;
const CLIP_REDUCE_WG: u32 = 256;

/// Counters for tracking dynamic allocation on the GPU.
///
/// This must be kept in sync with the struct in `shader/shared/bump.wgsl`
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
#[repr(C)]
pub struct BumpAllocators {
    pub failed: u32,
    // Final needed dynamic size of the buffers. If any of these are larger
    // than the corresponding `_size` element reallocation needs to occur.
    pub binning: u32,
    pub ptcl: u32,
    pub tile: u32,
    pub seg_counts: u32,
    pub segments: u32,
    pub blend: u32,
    pub lines: u32,
}

#[derive(Default)]
pub struct BumpAllocatorMemory {
    pub total: u32,
    pub binning: BufferSize<u32>,
    pub ptcl: BufferSize<u32>,
    pub tile: BufferSize<Tile>,
    pub seg_counts: BufferSize<SegmentCount>,
    pub segments: BufferSize<PathSegment>,
    pub lines: BufferSize<LineSoup>,
}

impl BumpAllocators {
    /// `true` when at least one stage failed to bump-allocate this frame.
    pub fn failed(&self) -> bool {
        self.failed != 0
    }

    pub fn memory(&self) -> BumpAllocatorMemory {
        let binning = BufferSize::new(self.binning);
        let ptcl = BufferSize::new(self.ptcl);
        let tile = BufferSize::new(self.tile);
        let seg_counts = BufferSize::new(self.seg_counts);
        let segments = BufferSize::new(self.segments);
        let lines = BufferSize::new(self.lines);
        BumpAllocatorMemory {
            total: binning.size_in_bytes()
                + ptcl.size_in_bytes()
                + tile.size_in_bytes()
                + seg_counts.size_in_bytes()
                + segments.size_in_bytes()
                + lines.size_in_bytes(),
            binning,
            ptcl,
            tile,
            seg_counts,
            segments,
            lines,
        }
    }
}

impl std::fmt::Display for BumpAllocatorMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n \
                 \tTotal:\t\t\t{} bytes ({:.2} KB | {:.2} MB)\n\
                 \tBinning\t\t\t{} elements ({} bytes)\n\
                 \tPTCL\t\t\t{} elements ({} bytes)\n\
                 \tTile:\t\t\t{} elements ({} bytes)\n\
                 \tSegment Counts:\t\t{} elements ({} bytes)\n\
                 \tSegments:\t\t{} elements ({} bytes)\n\
                 \tLines:\t\t\t{} elements ({} bytes)",
            self.total,
            self.total as f32 / (1 << 10) as f32,
            self.total as f32 / (1 << 20) as f32,
            self.binning.len(),
            self.binning.size_in_bytes(),
            self.ptcl.len(),
            self.ptcl.size_in_bytes(),
            self.tile.len(),
            self.tile.size_in_bytes(),
            self.seg_counts.len(),
            self.seg_counts.size_in_bytes(),
            self.segments.len(),
            self.segments.size_in_bytes(),
            self.lines.len(),
            self.lines.size_in_bytes()
        )
    }
}

/// Storage of indirect dispatch size values.
///
/// The original plan was to reuse [`BumpAllocators`], but the WebGPU compatible
/// usage list rules forbid that being used as indirect counts while also
/// bound as writable.
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
#[repr(C)]
pub struct IndirectCount {
    pub count_x: u32,
    pub count_y: u32,
    pub count_z: u32,
    pub pad0: u32,
}

/// Uniform render configuration data used by all GPU stages.
///
/// This data structure must be kept in sync with the definition in
/// `shaders/shared/config.wgsl`.
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
#[repr(C)]
pub struct ConfigUniform {
    /// Width of the scene in tiles.
    pub width_in_tiles: u32,
    /// Height of the scene in tiles.
    pub height_in_tiles: u32,
    /// Width of the target in pixels.
    pub target_width: u32,
    /// Height of the target in pixels.
    pub target_height: u32,
    /// The base background color applied to the target before any blends.
    pub base_color: u32,
    /// Layout of packed scene data.
    pub layout: Layout,
    /// Size of line soup buffer allocation (in [`LineSoup`]s)
    pub lines_size: u32,
    /// Size of binning buffer allocation (in `u32`s).
    pub binning_size: u32,
    /// Size of tile buffer allocation (in [`Tile`]s).
    pub tiles_size: u32,
    /// Size of segment count buffer allocation (in [`SegmentCount`]s).
    pub seg_counts_size: u32,
    /// Size of segment buffer allocation (in [`PathSegment`]s).
    pub segments_size: u32,
    /// Size of blend spill buffer (in `u32` pixels).
    // TODO: Maybe store in TILE_WIDTH * TILE_HEIGHT blocks of pixels instead?
    pub blend_size: u32,
    /// Size of per-tile command list buffer allocation (in `u32`s).
    pub ptcl_size: u32,
}

/// Sizes, in elements, for the buffers that the pipeline bump-allocates on
/// the GPU.
///
/// These are only the *initial* allocation for a render: when a frame exceeds
/// a buffer, the shaders still count the full demand in [`BumpAllocators`] and
/// the renderer grows the affected buffers and re-renders, so under-sizing
/// costs a retried frame, never a truncated one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BumpBufferSizes {
    /// Flattened line soup, in [`LineSoup`] elements.
    pub lines: u32,
    /// Binning data, in `u32`s. Includes the `Layout::bin_data_start` prefix.
    pub bin_data: u32,
    /// Per-path-per-tile records, in [`Tile`] elements.
    pub tiles: u32,
    /// Per-path segment counts, in [`SegmentCount`] elements.
    pub seg_counts: u32,
    /// Flattened path segments, in [`PathSegment`] elements.
    pub segments: u32,
    /// Blend spill, in `u32` pixels (256 pixels per spilled blend tile).
    pub blend_spill: u32,
    /// Per-tile command lists, in `u32`s.
    pub ptcl: u32,
}

impl Default for BumpBufferSizes {
    /// The historical fixed sizes, hand-picked to accommodate the vello test
    /// scenes as well as paris-30k without re-rendering. This allocates ~170 MB
    /// of bump memory per frame even for trivial scenes; prefer
    /// [`BumpBufferSizes::for_target`].
    fn default() -> Self {
        Self {
            lines: 1 << 21,
            bin_data: 1 << 18,
            tiles: 1 << 21,
            seg_counts: 1 << 21,
            segments: 1 << 21,
            // 16 * 16 (1 << 8) pixels is one blend spill, so this allows for
            // 4096 spills.
            blend_spill: 1 << 20,
            ptcl: 1 << 23,
        }
    }
}

impl BumpBufferSizes {
    /// Initial sizes derived from the render target and the resolved scene
    /// layout.
    ///
    /// Each buffer is sized a small multiple of the structure it actually
    /// scales with — viewport tiles for the tile-domain buffers, path and
    /// draw-object counts for the path-domain buffers — with a floor so
    /// trivial scenes stay cheap, and capped at the historical constants so a
    /// derived estimate never exceeds the pre-sizing behaviour. Anything the
    /// estimate misses is caught and grown by the bump-allocator feedback, so
    /// these only need to be reasonable, not airtight.
    pub fn for_layout(layout: &Layout, width_in_tiles: u32, height_in_tiles: u32) -> Self {
        let n_tiles = width_in_tiles.max(1) * height_in_tiles.max(1);
        // Bins are 16x16 tiles; each active bin emits a handful of words.
        let n_bins = width_in_tiles.div_ceil(16).max(1) * height_in_tiles.div_ceil(16).max(1);
        let n_paths = layout.n_paths.max(1);
        let n_draw_objects = layout.n_draw_objects;
        let n_clips = layout.n_clips.max(1);
        // A rough count of path-tile intersections: every tile a path covers
        // gets a `seg_counts` entry and some `segments`/`tiles`. A path that
        // covers many tiles dominates these; assume a handful of intersections
        // per path plus base coverage.
        let intersections = n_paths * 4 + n_tiles / 4;
        let fallback = Self::default();
        Self {
            bin_data: (layout.bin_data_start + n_draw_objects * 8 + n_bins * 32)
                .clamp(1 << 14, fallback.bin_data),
            // One tile record per (path x covered tile); most tiles see few paths.
            tiles: (intersections * 2).clamp(1 << 12, fallback.tiles),
            seg_counts: intersections.clamp(1 << 12, fallback.seg_counts),
            // Flattened path segments: a few dozen per path on average, scaled
            // further by tile coverage for long curves.
            segments: (n_paths * 64 + intersections * 8).clamp(1 << 12, fallback.segments),
            lines: (n_paths * 16).clamp(1 << 12, fallback.lines),
            blend_spill: (n_clips * 512).clamp(1 << 14, fallback.blend_spill),
            // PTCL_INITIAL_ALLOC (64 words, shared/ptcl.wgsl) per tile is
            // reserved unconditionally by the coarse stage.
            ptcl: (n_tiles * 96).clamp(1 << 16, fallback.ptcl),
        }
    }

    /// Initial sizes derived from the render target alone, for callers that
    /// configure the renderer before a scene exists. Equivalent to
    /// [`for_layout`](Self::for_layout) with an empty layout; the
    /// path-dependent sizes fall back to their floors and grow on demand.
    pub fn for_target(width: u32, height: u32) -> Self {
        Self::for_layout(
            &Layout::default(),
            width.div_ceil(TILE_WIDTH),
            height.div_ceil(TILE_HEIGHT),
        )
    }

    /// Grow each size to cover the demand reported in `bump`, with headroom so
    /// the next frame does not immediately overflow again.
    ///
    /// `bump` counts allocation indices, not total buffer sizes, so the
    /// fixed offsets the shaders carve out come from the layout:
    /// `bin_data_start` words of `bin_data` hold the info/bin prefix
    /// (binning.wgsl compares `bump.binning` against `binning_size`), and
    /// `n_tiles * PTCL_INITIAL_ALLOC` (64 words per tile, shared/ptcl.wgsl)
    /// is reserved at the head of `ptcl` before `bump.ptcl` starts counting.
    pub fn satisfy(&mut self, bump: &BumpAllocators, bin_data_start: u32, n_tiles: u32) {
        // The counters keep counting past the allocation on overflow, so each
        // value is the exact demand for this frame. 25% + a small constant of
        // headroom absorbs growth within a stage family.
        fn grow(current: &mut u32, needed: u32) {
            *current = (*current).max(needed + needed / 4 + 64);
        }
        grow(&mut self.bin_data, bin_data_start + bump.binning);
        grow(&mut self.ptcl, n_tiles * 64 + bump.ptcl);
        grow(&mut self.tiles, bump.tile);
        grow(&mut self.seg_counts, bump.seg_counts);
        grow(&mut self.segments, bump.segments);
        grow(&mut self.blend_spill, bump.blend);
        grow(&mut self.lines, bump.lines);
    }

    /// Whether these sizes cover the demand reported in `bump`, given the
    /// same shader-side reserves as [`satisfy`](Self::satisfy).
    ///
    /// Some bump counters overflow silently: `bump.segments` is bumped in
    /// coarse.wgsl without a bounds check (the flag only fires when the ptcl
    /// command stream or blend stack overflows), so `failed` alone misses an
    /// exhausted segments buffer.
    pub fn covers(&self, bump: &BumpAllocators, bin_data_start: u32, n_tiles: u32) -> bool {
        self.bin_data >= bin_data_start + bump.binning
            && self.ptcl >= n_tiles * 64 + bump.ptcl
            && self.tiles >= bump.tile
            && self.seg_counts >= bump.seg_counts
            && self.segments >= bump.segments
            && self.blend_spill >= bump.blend
            && self.lines >= bump.lines
    }

    /// Set every field to the larger of `self` and `other`.
    pub fn merge_max(&mut self, other: &Self) {
        self.lines = self.lines.max(other.lines);
        self.bin_data = self.bin_data.max(other.bin_data);
        self.tiles = self.tiles.max(other.tiles);
        self.seg_counts = self.seg_counts.max(other.seg_counts);
        self.segments = self.segments.max(other.segments);
        self.blend_spill = self.blend_spill.max(other.blend_spill);
        self.ptcl = self.ptcl.max(other.ptcl);
    }
}

/// CPU side setup and configuration.
#[derive(Clone, Default)]
pub struct RenderConfig {
    /// GPU side configuration.
    pub gpu: ConfigUniform,
    /// Workgroup counts for all compute pipelines.
    pub workgroup_counts: WorkgroupCounts,
    /// Sizes of all buffer resources.
    pub buffer_sizes: BufferSizes,
}

impl RenderConfig {
    /// Creates the configuration with the historical fixed bump-buffer sizes.
    ///
    /// Prefer [`RenderConfig::with_buffer_sizes`], which sizes the
    /// bump-allocated buffers from the scene.
    pub fn new(layout: &Layout, width: u32, height: u32, base_color: &peniko::Color) -> Self {
        Self::with_buffer_sizes(
            layout,
            width,
            height,
            base_color,
            Some(&BumpBufferSizes::default()),
        )
    }

    /// Creates the configuration, sizing the bump-allocated buffers to `bump`;
    /// `None` derives them from the scene layout and the render target.
    pub fn with_buffer_sizes(
        layout: &Layout,
        width: u32,
        height: u32,
        base_color: &peniko::Color,
        bump: Option<&BumpBufferSizes>,
    ) -> Self {
        let new_width = width.next_multiple_of(TILE_WIDTH);
        let new_height = height.next_multiple_of(TILE_HEIGHT);
        let width_in_tiles = new_width / TILE_WIDTH;
        let height_in_tiles = new_height / TILE_HEIGHT;
        let n_path_tags = layout.path_tags_size();
        let workgroup_counts =
            WorkgroupCounts::new(layout, width_in_tiles, height_in_tiles, n_path_tags);
        let bump = bump.copied().unwrap_or_else(|| {
            BumpBufferSizes::for_layout(layout, width_in_tiles, height_in_tiles)
        });
        let buffer_sizes = BufferSizes::new(layout, &workgroup_counts, &bump);
        Self {
            gpu: ConfigUniform {
                width_in_tiles,
                height_in_tiles,
                target_width: width,
                target_height: height,
                base_color: base_color.premultiply().to_rgba8().to_u32(),
                lines_size: buffer_sizes.lines.len(),
                binning_size: buffer_sizes.bin_data.len() - layout.bin_data_start,
                tiles_size: buffer_sizes.tiles.len(),
                seg_counts_size: buffer_sizes.seg_counts.len(),
                segments_size: buffer_sizes.segments.len(),
                blend_size: buffer_sizes.blend_spill.len(),
                ptcl_size: buffer_sizes.ptcl.len(),
                layout: *layout,
            },
            workgroup_counts,
            buffer_sizes,
        }
    }
}

/// Type alias for a workgroup size.
pub type WorkgroupSize = (u32, u32, u32);

/// Computed sizes for all dispatches.
#[derive(Copy, Clone, Debug, Default)]
pub struct WorkgroupCounts {
    pub use_large_path_scan: bool,
    pub path_reduce: WorkgroupSize,
    pub path_reduce2: WorkgroupSize,
    pub path_scan1: WorkgroupSize,
    pub path_scan: WorkgroupSize,
    pub bbox_clear: WorkgroupSize,
    pub flatten: WorkgroupSize,
    pub draw_reduce: WorkgroupSize,
    pub draw_leaf: WorkgroupSize,
    pub clip_reduce: WorkgroupSize,
    pub clip_leaf: WorkgroupSize,
    pub binning: WorkgroupSize,
    pub tile_alloc: WorkgroupSize,
    pub path_count_setup: WorkgroupSize,
    // Note: `path_count` must use an indirect dispatch
    pub backdrop: WorkgroupSize,
    pub coarse: WorkgroupSize,
    pub path_tiling_setup: WorkgroupSize,
    // Note: `path_tiling` must use an indirect dispatch
    pub fine: WorkgroupSize,
}

impl WorkgroupCounts {
    pub fn new(
        layout: &Layout,
        width_in_tiles: u32,
        height_in_tiles: u32,
        n_path_tags: u32,
    ) -> Self {
        let n_paths = layout.n_paths;
        let n_draw_objects = layout.n_draw_objects;
        let n_clips = layout.n_clips;
        let path_tag_padded = align_up(n_path_tags, 4 * PATH_REDUCE_WG);
        let path_tag_wgs = path_tag_padded / (4 * PATH_REDUCE_WG);
        let use_large_path_scan = path_tag_wgs > PATH_REDUCE_WG;
        let reduced_size = if use_large_path_scan {
            align_up(path_tag_wgs, PATH_REDUCE_WG)
        } else {
            path_tag_wgs
        };
        let draw_object_wgs = n_draw_objects.div_ceil(PATH_BBOX_WG);
        let draw_monoid_wgs = draw_object_wgs.min(PATH_BBOX_WG);
        let flatten_wgs = n_path_tags.div_ceil(FLATTEN_WG);
        let clip_reduce_wgs = n_clips.saturating_sub(1) / CLIP_REDUCE_WG;
        let clip_wgs = n_clips.div_ceil(CLIP_REDUCE_WG);
        let path_wgs = n_paths.div_ceil(PATH_BBOX_WG);
        let width_in_bins = width_in_tiles.div_ceil(16);
        let height_in_bins = height_in_tiles.div_ceil(16);
        Self {
            use_large_path_scan,
            path_reduce: (path_tag_wgs, 1, 1),
            path_reduce2: (PATH_REDUCE_WG, 1, 1),
            path_scan1: (reduced_size / PATH_REDUCE_WG, 1, 1),
            path_scan: (path_tag_wgs, 1, 1),
            bbox_clear: (draw_object_wgs, 1, 1),
            flatten: (flatten_wgs, 1, 1),
            draw_reduce: (draw_monoid_wgs, 1, 1),
            draw_leaf: (draw_monoid_wgs, 1, 1),
            clip_reduce: (clip_reduce_wgs, 1, 1),
            clip_leaf: (clip_wgs, 1, 1),
            binning: (draw_object_wgs, 1, 1),
            tile_alloc: (path_wgs, 1, 1),
            path_count_setup: (1, 1, 1),
            backdrop: (path_wgs, 1, 1),
            coarse: (width_in_bins, height_in_bins, 1),
            path_tiling_setup: (1, 1, 1),
            fine: (width_in_tiles, height_in_tiles, 1),
        }
    }
}

/// Typed buffer size primitive.
#[derive(Copy, Clone, Eq, Default, Debug)]
pub struct BufferSize<T: Sized> {
    len: u32,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Sized> BufferSize<T> {
    /// Creates a new buffer size from number of elements.
    pub const fn new(len: u32) -> Self {
        Self {
            // Each buffer binding must be large enough to hold at least one element to avoid
            // triggering validation errors.
            //
            // Note: not using `Ord::max` here because it doesn't support const eval yet (except
            // in nightly)
            len: if len > 0 { len } else { 1 },
            _phantom: std::marker::PhantomData,
        }
    }

    /// Creates a new buffer size from size in bytes.
    pub const fn from_size_in_bytes(size: u32) -> Self {
        Self::new(size / size_of::<T>() as u32)
    }

    /// Returns the number of elements.
    #[expect(clippy::len_without_is_empty, reason = "The buffer can never be empty")]
    pub const fn len(self) -> u32 {
        self.len
    }

    /// Returns the size in bytes.
    pub const fn size_in_bytes(self) -> u32 {
        size_of::<T>() as u32 * self.len
    }

    /// Returns the size in bytes aligned up to the given value.
    pub const fn aligned_in_bytes(self, alignment: u32) -> u32 {
        align_up(self.size_in_bytes(), alignment)
    }
}

impl<T: Sized> PartialEq for BufferSize<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len
    }
}

impl<T: Sized> PartialOrd for BufferSize<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.len.partial_cmp(&other.len)
    }
}

/// Computed sizes for all buffers.
#[derive(Copy, Clone, Debug, Default)]
pub struct BufferSizes {
    // Known size buffers
    pub path_reduced: BufferSize<PathMonoid>,
    pub path_reduced2: BufferSize<PathMonoid>,
    pub path_reduced_scan: BufferSize<PathMonoid>,
    pub path_monoids: BufferSize<PathMonoid>,
    pub path_bboxes: BufferSize<PathBbox>,
    pub draw_reduced: BufferSize<DrawMonoid>,
    pub draw_monoids: BufferSize<DrawMonoid>,
    pub info: BufferSize<u32>,
    pub clip_inps: BufferSize<Clip>,
    pub clip_els: BufferSize<ClipElement>,
    pub clip_bics: BufferSize<ClipBic>,
    pub clip_bboxes: BufferSize<ClipBbox>,
    pub draw_bboxes: BufferSize<DrawBbox>,
    pub bump_alloc: BufferSize<BumpAllocators>,
    pub indirect_count: BufferSize<IndirectCount>,
    pub bin_headers: BufferSize<BinHeader>,
    pub paths: BufferSize<Path>,
    // Bump allocated buffers
    pub lines: BufferSize<LineSoup>,
    pub bin_data: BufferSize<u32>,
    pub tiles: BufferSize<Tile>,
    pub seg_counts: BufferSize<SegmentCount>,
    pub segments: BufferSize<PathSegment>,
    pub blend_spill: BufferSize<u32>,
    pub ptcl: BufferSize<u32>,
}

impl BufferSizes {
    /// The bump-allocated buffer sizes used for this configuration.
    pub fn bump_sizes(&self) -> BumpBufferSizes {
        BumpBufferSizes {
            lines: self.lines.len(),
            bin_data: self.bin_data.len(),
            tiles: self.tiles.len(),
            seg_counts: self.seg_counts.len(),
            segments: self.segments.len(),
            blend_spill: self.blend_spill.len(),
            ptcl: self.ptcl.len(),
        }
    }

    pub fn new(layout: &Layout, workgroups: &WorkgroupCounts, bump: &BumpBufferSizes) -> Self {
        let n_paths = layout.n_paths;
        let n_draw_objects = layout.n_draw_objects;
        let n_clips = layout.n_clips;
        let path_tag_wgs = workgroups.path_reduce.0;
        let reduced_size = if workgroups.use_large_path_scan {
            align_up(path_tag_wgs, PATH_REDUCE_WG)
        } else {
            path_tag_wgs
        };
        let path_reduced = BufferSize::new(reduced_size);
        let path_reduced2 = BufferSize::new(PATH_REDUCE_WG);
        let path_reduced_scan = BufferSize::new(reduced_size);
        let path_monoids = BufferSize::new(path_tag_wgs * PATH_REDUCE_WG);
        let path_bboxes = BufferSize::new(n_paths);
        let binning_wgs = workgroups.binning.0;
        let draw_monoid_wgs = workgroups.draw_reduce.0;
        let draw_reduced = BufferSize::new(draw_monoid_wgs);
        let draw_monoids = BufferSize::new(n_draw_objects);
        let info = BufferSize::new(layout.bin_data_start);
        let clip_inps = BufferSize::new(n_clips);
        let clip_els = BufferSize::new(n_clips);
        let clip_bics = BufferSize::new(n_clips / CLIP_REDUCE_WG);
        let clip_bboxes = BufferSize::new(n_clips);
        let draw_bboxes = BufferSize::new(n_paths);
        let bump_alloc = BufferSize::new(1);
        let indirect_count = BufferSize::new(1);
        let n_paths_aligned = align_up(n_paths, 256);
        let paths = BufferSize::new(n_paths_aligned);
        let width_in_bins = workgroups.coarse.0;
        let height_in_bins = workgroups.coarse.1;
        let n_bins = width_in_bins * height_in_bins;
        let aligned_n_bins = align_up(n_bins, 256);
        let bin_headers = BufferSize::new(binning_wgs * aligned_n_bins);

        // The first `bin_data_start` words of `bin_data` hold the info/bin
        // prefix, so the buffer can never be smaller than the layout's.
        let bin_data = BufferSize::new(bump.bin_data.max(layout.bin_data_start));
        let tiles = BufferSize::new(bump.tiles);
        let lines = BufferSize::new(bump.lines);
        let seg_counts = BufferSize::new(bump.seg_counts);
        let segments = BufferSize::new(bump.segments);
        let blend_spill = BufferSize::new(bump.blend_spill);
        let ptcl = BufferSize::new(bump.ptcl);
        Self {
            path_reduced,
            path_reduced2,
            path_reduced_scan,
            path_monoids,
            path_bboxes,
            draw_reduced,
            draw_monoids,
            info,
            clip_inps,
            clip_els,
            clip_bics,
            clip_bboxes,
            draw_bboxes,
            bump_alloc,
            indirect_count,
            lines,
            bin_headers,
            paths,
            bin_data,
            tiles,
            seg_counts,
            segments,
            blend_spill,
            ptcl,
        }
    }
}

const fn align_up(len: u32, alignment: u32) -> u32 {
    len + (len.wrapping_neg() & (alignment - 1))
}
