// Copyright 2025 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Generated code by `vello_sparse_shaders` - DO NOT EDIT
/// WGSL shader sources linked from WESL modules.
pub mod wgsl {
    /// Linked WGSL source for `blend.wesl`.
    pub const BLEND: &str = r###"const COMPOSE_CLEAR: u32 = 0u;

const COMPOSE_COPY: u32 = 1u;

const COMPOSE_DEST: u32 = 2u;

const COMPOSE_SRC_OVER: u32 = 3u;

const COMPOSE_DEST_OVER: u32 = 4u;

const COMPOSE_SRC_IN: u32 = 5u;

const COMPOSE_DEST_IN: u32 = 6u;

const COMPOSE_SRC_OUT: u32 = 7u;

const COMPOSE_DEST_OUT: u32 = 8u;

const COMPOSE_SRC_ATOP: u32 = 9u;

const COMPOSE_DEST_ATOP: u32 = 10u;

const COMPOSE_XOR: u32 = 11u;

const COMPOSE_PLUS: u32 = 12u;

const COMPOSE_PLUS_LIGHTER: u32 = 13u;

const MIX_NORMAL = 0u;

const MIX_MULTIPLY = 1u;

const MIX_SCREEN = 2u;

const MIX_OVERLAY = 3u;

const MIX_DARKEN = 4u;

const MIX_LIGHTEN = 5u;

const MIX_COLOR_DODGE = 6u;

const MIX_COLOR_BURN = 7u;

const MIX_HARD_LIGHT = 8u;

const MIX_SOFT_LIGHT = 9u;

const MIX_DIFFERENCE = 10u;

const MIX_EXCLUSION = 11u;

const MIX_HUE = 12u;

const MIX_SATURATION = 13u;

const MIX_COLOR = 14u;

const MIX_LUMINOSITY = 15u;

const BLEND_HAS_ALPHA = 1u << 26u;

struct BlendInstance {
    @location(0)
    geometry_origin: u32,
    @location(1)
    geometry_size: u32,
    @location(2)
    geometry_alpha_col_idx: u32,
    @location(3)
    parent_texture_size: u32,
    @location(4)
    child_texture_origin: u32,
    @location(5)
    child_parent_origin: u32,
    @location(6)
    child_rect_size: u32,
    @location(7)
    blend_config: u32
}

struct VertexOutput {
    @location(0)
    parent_texture_xy: vec2<f32>,
    @location(1)
    child_texture_xy: vec2<f32>,
    @location(2)
    child_local: vec2<f32>,
    @location(3) @interpolate(flat)
    child_rect_size: u32,
    @location(4) @interpolate(flat)
    blend_config: u32,
    @location(5)
    tex_coord: vec2<f32>,
    @builtin(position)
    position: vec4<f32>
}

@group(0) @binding(0)
var layer_texture_0: texture_2d<f32>;

@group(0) @binding(1)
var layer_texture_1: texture_2d<f32>;

@group(0) @binding(2)
var alphas_texture: texture_2d<u32>;

fn unpack_u16_pair(value: u32) -> vec2<u32> {
    return vec2<u32>(value & 65535u, value >> 16u);
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32, instance: BlendInstance) -> VertexOutput {
    let geometry_origin = unpack_u16_pair(instance.geometry_origin);
    let geometry_size = vec2<f32>(unpack_u16_pair(instance.geometry_size));
    let parent_texture_size = unpack_u16_pair(instance.parent_texture_size);
    let child_parent_origin = unpack_u16_pair(instance.child_parent_origin);
    let child_texture_origin = unpack_u16_pair(instance.child_texture_origin);
    let x = f32(vertex_index & 1u);
    let y = f32(vertex_index >> 1u);
    let parent_texture_xy = vec2<f32>(geometry_origin) + vec2<f32>(x, y) * geometry_size;
    let child_local = parent_texture_xy - vec2<f32>(child_parent_origin);
    var out: VertexOutput;
    out.parent_texture_xy = parent_texture_xy;
    out.child_texture_xy = vec2<f32>(child_texture_origin) + child_local;
    out.child_local = child_local;
    out.child_rect_size = instance.child_rect_size;
    out.blend_config = instance.blend_config;
    out.tex_coord = vec2<f32>(f32(instance.geometry_alpha_col_idx) + x * geometry_size.x, y * geometry_size.y);
    let ndc_x = parent_texture_xy.x * 2.0 / f32(parent_texture_size.x) - 1.0;
    let ndc_y = 1.0 - parent_texture_xy.y * 2.0 / f32(parent_texture_size.y);
    out.position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    return out;
}

fn load_layer(texture_index: u32, xy: vec2<i32>) -> vec4<f32> {
    if texture_index == 0u {
        return textureLoad(layer_texture_0, xy, 0);
    }
    else {
        return textureLoad(layer_texture_1, xy, 0);
    }
}

@fragment
fn fs_main(@location(0) parent_texture_xy: vec2<f32>, @location(1) child_texture_xy: vec2<f32>, @location(2) child_local: vec2<f32>, @location(3) @interpolate(flat) packed_child_rect_size: u32, @location(4) @interpolate(flat) blend_config: u32, @location(5) tex_coord: vec2<f32>) -> @location(0) vec4<f32> {
    let child_rect_size = unpack_u16_pair(packed_child_rect_size);
    let compose = blend_config & 255u;
    let mix_mode = (blend_config >> 8u) & 255u;
    let opacity = (blend_config >> 16u) & 255u;
    let parent_texture_index = (blend_config >> 24u) & 1u;
    let child_texture_index = (blend_config >> 25u) & 1u;
    let has_alpha = (blend_config >> 26u) & 1u;
    var clip_coverage = 1.0;
    if (has_alpha) != 0u {
        let alpha_index = u32(floor(tex_coord.x));
        let y = u32(floor(tex_coord.y));
        let texel_index = alpha_index / 4u;
        let channel_index = alpha_index % 4u;
        let texture_width = textureDimensions(alphas_texture).x;
        let packed = textureLoad(alphas_texture, vec2<u32>(texel_index % texture_width, texel_index / texture_width), 0);
        var column = packed.x;
        switch channel_index {
            case 1u {
                column = packed.y;
            }
            case 2u {
                column = packed.z;
            }
            case 3u {
                column = packed.w;
            }
            case default {
            
            }
        }
        clip_coverage = f32((column >> (y * 8u)) & 255u) * (1.0 / 255.0);
    }
    let backdrop = load_layer(parent_texture_index, vec2<i32>(parent_texture_xy));
    let child_opacity = f32(opacity) * (1.0 / 255.0);
    var child = vec4<f32>(0.0);
    if child_local.x >= 0.0 && child_local.y >= 0.0 && child_local.x < f32(child_rect_size.x) && child_local.y < f32(child_rect_size.y) {
        child = child_opacity * load_layer(child_texture_index, vec2<i32>(child_texture_xy));
    }
    let composed = blend_mix_compose(backdrop, child, compose, mix_mode);
    return mix(backdrop, composed, clip_coverage);
}

fn blend_mix_compose(backdrop: vec4<f32>, src: vec4<f32>, compose_mode: u32, mix_mode: u32) -> vec4<f32> {
    let BLEND_DEFAULT = ((MIX_NORMAL << 8u) | COMPOSE_SRC_OVER);
    let mode = ((mix_mode << 8u) | compose_mode);
    if mode == BLEND_DEFAULT {
        return backdrop * (1.0 - src.a) + src;
    }
    let EPSILON = 1e-15;
    let inv_src_a = 1.0 / max(src.a, EPSILON);
    var cs = src.rgb * inv_src_a;
    let inv_backdrop_a = 1.0 / max(backdrop.a, EPSILON);
    let cb = backdrop.rgb * inv_backdrop_a;
    let mixed = blend_mix(cb, cs, mix_mode);
    cs = mix(cs, mixed, backdrop.a);
    if compose_mode == COMPOSE_SRC_OVER {
        let co = mix(backdrop.rgb, cs, src.a);
        return vec4(co, src.a + backdrop.a * (1.0 - src.a));
    }
    else {
        return blend_compose_unpremul(cb, cs, backdrop.a, src.a, compose_mode);
    }
}

fn blend_compose_unpremul(cb: vec3<f32>, cs: vec3<f32>, ab: f32, as_: f32, mode: u32) -> vec4<f32> {
    var fa = 0.0;
    var fb = 0.0;
    switch mode {
        case COMPOSE_COPY {
            fa = 1.0;
            fb = 0.0;
        }
        case COMPOSE_DEST {
            fa = 0.0;
            fb = 1.0;
        }
        case COMPOSE_SRC_OVER {
            fa = 1.0;
            fb = 1.0 - as_;
        }
        case COMPOSE_DEST_OVER {
            fa = 1.0 - ab;
            fb = 1.0;
        }
        case COMPOSE_SRC_IN {
            fa = ab;
            fb = 0.0;
        }
        case COMPOSE_DEST_IN {
            fa = 0.0;
            fb = as_;
        }
        case COMPOSE_SRC_OUT {
            fa = 1.0 - ab;
            fb = 0.0;
        }
        case COMPOSE_DEST_OUT {
            fa = 0.0;
            fb = 1.0 - as_;
        }
        case COMPOSE_SRC_ATOP {
            fa = ab;
            fb = 1.0 - as_;
        }
        case COMPOSE_DEST_ATOP {
            fa = 1.0 - ab;
            fb = as_;
        }
        case COMPOSE_XOR {
            fa = 1.0 - ab;
            fb = 1.0 - as_;
        }
        case COMPOSE_PLUS {
            fa = 1.0;
            fb = 1.0;
        }
        case COMPOSE_PLUS_LIGHTER {
            return min(vec4(1.0), vec4(as_ * cs + ab * cb, as_ + ab));
        }
        case default {
        
        }
    }
    let as_fa = as_ * fa;
    let ab_fb = ab * fb;
    let co = as_fa * cs + ab_fb * cb;
    return vec4(co, min(as_fa + ab_fb, 1.0));
}

fn screen(cb: vec3<f32>, cs: vec3<f32>) -> vec3<f32> {
    return cb + cs - (cb * cs);
}

fn color_dodge(cb: f32, cs: f32) -> f32 {
    if cb == 0.0 {
        return 0.0;
    }
    else if cs == 1.0 {
        return 1.0;
    }
    else {
        return min(1.0, cb / (1.0 - cs));
    }
}

fn color_burn(cb: f32, cs: f32) -> f32 {
    if cb == 1.0 {
        return 1.0;
    }
    else if cs == 0.0 {
        return 0.0;
    }
    else {
        return 1.0 - min(1.0, (1.0 - cb) / cs);
    }
}

fn hard_light(cb: vec3<f32>, cs: vec3<f32>) -> vec3<f32> {
    return select(screen(cb, 2.0 * cs - 1.0), cb * 2.0 * cs, cs <= vec3(0.5));
}

fn soft_light(cb: vec3<f32>, cs: vec3<f32>) -> vec3<f32> {
    let d = select(sqrt(cb), ((16.0 * cb - 12.0) * cb + 4.0) * cb, cb <= vec3(0.25));
    return select(cb + (2.0 * cs - 1.0) * (d - cb), cb - (1.0 - 2.0 * cs) * cb * (1.0 - cb), cs <= vec3(0.5));
}

fn sat(c: vec3<f32>) -> f32 {
    return max(c.x, max(c.y, c.z)) - min(c.x, min(c.y, c.z));
}

fn lum(c: vec3<f32>) -> f32 {
    let f = vec3(0.3, 0.59, 0.11);
    return dot(c, f);
}

fn clip_color(c_in: vec3<f32>) -> vec3<f32> {
    var c = c_in;
    let l = lum(c);
    let n = min(c.x, min(c.y, c.z));
    let x = max(c.x, max(c.y, c.z));
    if n < 0.0 {
        c = l + (((c - l) * l) / (l - n));
    }
    if x > 1.0 {
        c = l + (((c - l) * (1.0 - l)) / (x - l));
    }
    return c;
}

fn set_lum(c: vec3<f32>, l: f32) -> vec3<f32> {
    return clip_color(c + (l - lum(c)));
}

fn set_sat_inner(cmin: ptr<function, f32>, cmid: ptr<function, f32>, cmax: ptr<function, f32>, s: f32) {
    if *cmax > *cmin {
        *cmid = ((*cmid - *cmin) * s) / (*cmax - *cmin);
        *cmax = s;
    }
    else {
        *cmid = 0.0;
        *cmax = 0.0;
    }
    *cmin = 0.0;
}

fn set_sat(c: vec3<f32>, s: f32) -> vec3<f32> {
    var r = c.r;
    var g = c.g;
    var b = c.b;
    if r <= g {
        if g <= b {
            set_sat_inner(&r, &g, &b, s);
        }
        else {
            if r <= b {
                set_sat_inner(&r, &b, &g, s);
            }
            else {
                set_sat_inner(&b, &r, &g, s);
            }
        }
    }
    else {
        if r <= b {
            set_sat_inner(&g, &r, &b, s);
        }
        else {
            if g <= b {
                set_sat_inner(&g, &b, &r, s);
            }
            else {
                set_sat_inner(&b, &g, &r, s);
            }
        }
    }
    return vec3(r, g, b);
}

fn blend_mix(cb: vec3<f32>, cs: vec3<f32>, mode: u32) -> vec3<f32> {
    var b = vec3(0.0);
    switch mode {
        case MIX_MULTIPLY {
            b = cb * cs;
        }
        case MIX_SCREEN {
            b = screen(cb, cs);
        }
        case MIX_OVERLAY {
            b = hard_light(cs, cb);
        }
        case MIX_DARKEN {
            b = min(cb, cs);
        }
        case MIX_LIGHTEN {
            b = max(cb, cs);
        }
        case MIX_COLOR_DODGE {
            b = vec3(color_dodge(cb.x, cs.x), color_dodge(cb.y, cs.y), color_dodge(cb.z, cs.z));
        }
        case MIX_COLOR_BURN {
            b = vec3(color_burn(cb.x, cs.x), color_burn(cb.y, cs.y), color_burn(cb.z, cs.z));
        }
        case MIX_HARD_LIGHT {
            b = hard_light(cb, cs);
        }
        case MIX_SOFT_LIGHT {
            b = soft_light(cb, cs);
        }
        case MIX_DIFFERENCE {
            b = abs(cb - cs);
        }
        case MIX_EXCLUSION {
            b = cb + cs - 2.0 * cb * cs;
        }
        case MIX_HUE {
            b = set_lum(set_sat(cs, sat(cb)), lum(cb));
        }
        case MIX_SATURATION {
            b = set_lum(set_sat(cb, sat(cs)), lum(cb));
        }
        case MIX_COLOR {
            b = set_lum(cs, lum(cb));
        }
        case MIX_LUMINOSITY {
            b = set_lum(cb, lum(cs));
        }
        case default {
            b = cs;
        }
    }
    return b;
}
"###;
    /// Linked WGSL source for `clear.wesl`.
    pub const CLEAR: &str = r###"@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32, @location(0) origin: vec2<u32>, @location(1) size: vec2<u32>, @location(2) target_size: vec2<u32>) -> @builtin(position) vec4<f32> {
    let x = f32(vertex_index & 1u);
    let y = f32(vertex_index >> 1u);
    let pix_x = f32(origin.x) + x * f32(size.x);
    let pix_y = f32(origin.y) + y * f32(size.y);
    let ndc_x = pix_x * 2.0 / f32(target_size.x) - 1.0;
    let ndc_y = 1.0 - pix_y * 2.0 / f32(target_size.y);
    return vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
}

@vertex
fn vs_main_fullscreen(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32((vertex_index & 1u) * 2u) - 1.0;
    let y = f32((vertex_index & 2u)) - 1.0;
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
}
"###;
    /// Linked WGSL source for `copy.wesl`.
    pub const COPY: &str = r###"struct CopyInstance {
    @location(0)
    dest_texture_origin: u32,
    @location(1)
    source_texture_origin: u32,
    @location(2)
    copy_rect_size: u32,
    @location(3)
    dest_texture_size: u32
}

struct VertexOutput {
    @location(0)
    source_texture_xy: vec2<f32>,
    @builtin(position)
    position: vec4<f32>
}

@group(0) @binding(0)
var source_texture: texture_2d<f32>;

fn unpack_u16_pair(value: u32) -> vec2<u32> {
    return vec2<u32>(value & 65535u, value >> 16u);
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32, instance: CopyInstance) -> VertexOutput {
    let dest_texture_origin = unpack_u16_pair(instance.dest_texture_origin);
    let source_texture_origin = unpack_u16_pair(instance.source_texture_origin);
    let copy_rect_size = unpack_u16_pair(instance.copy_rect_size);
    let dest_texture_size = unpack_u16_pair(instance.dest_texture_size);
    let x = f32(vertex_index & 1u);
    let y = f32(vertex_index >> 1u);
    let local = vec2<f32>(x, y) * vec2<f32>(copy_rect_size);
    let dest_texture_xy = vec2<f32>(dest_texture_origin) + local;
    var out: VertexOutput;
    out.source_texture_xy = vec2<f32>(source_texture_origin) + local;
    let ndc_x = dest_texture_xy.x * 2.0 / f32(dest_texture_size.x) - 1.0;
    let ndc_y = 1.0 - dest_texture_xy.y * 2.0 / f32(dest_texture_size.y);
    out.position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(@location(0) source_texture_xy: vec2<f32>) -> @location(0) vec4<f32> {
    return textureLoad(source_texture, vec2<i32>(source_texture_xy), 0);
}
"###;
    /// Linked WGSL source for `filter.wesl`.
    pub const FILTER: &str = r###"@group(0) @binding(0)
var filter_data: texture_2d<u32>;

@group(1) @binding(0)
var source_texture: texture_2d<f32>;

@group(1) @binding(1)
var linear_sampler: sampler;

@group(2) @binding(0)
var original_texture: texture_2d<f32>;

const FILTER_SIZE_BYTES: u32 = 48;

const FILTER_SIZE_U32: u32 = FILTER_SIZE_BYTES / 4;

const TEXELS_PER_FILTER: u32 = FILTER_SIZE_U32 / 4u;

const FILTER_TYPE_OFFSET: u32 = 0u;

const FILTER_TYPE_FLOOD: u32 = 1u;

const FILTER_TYPE_GAUSSIAN_BLUR: u32 = 2u;

const FILTER_TYPE_DROP_SHADOW: u32 = 3u;

const PASS_COPY: u32 = 0u;

const PASS_FLOOD: u32 = 1u;

const PASS_OFFSET: u32 = 2u;

const PASS_DOWNSCALE: u32 = 3u;

const PASS_BLUR_H: u32 = 4u;

const PASS_BLUR_V: u32 = 5u;

const PASS_UPSCALE: u32 = 6u;

const PASS_COMPOSITE_DROP_SHADOW: u32 = 7u;

const PASS_COLORIZE: u32 = 8u;

const FILTER_ATLAS_PADDING: u32 = 6u;

const MAX_TAPS_PER_SIDE: u32 = 3u;

fn load_filter_texel(texel_offset: u32, texel_index: u32) -> vec4<u32> {
    let w = textureDimensions(filter_data).x;
    let flat_index = texel_offset + texel_index;
    return textureLoad(filter_data, vec2(flat_index % w, flat_index / w), 0);
}

fn get_filter_type(texel0: vec4<u32>) -> u32 {
    return texel0.x & 31u;
}

fn get_filter_header_n_linear_taps(texel0: vec4<u32>) -> u32 {
    return (texel0.x >> 11u) & 3u;
}

fn get_offset_dx(texel0: vec4<u32>) -> f32 {
    return bitcast<f32>(texel0.y);
}

fn get_offset_dy(texel0: vec4<u32>) -> f32 {
    return bitcast<f32>(texel0.z);
}

fn get_flood_color(texel0: vec4<u32>) -> u32 {
    return texel0.y;
}

fn get_blur_center_weight(texel0: vec4<u32>) -> f32 {
    return bitcast<f32>(texel0.y);
}

fn get_blur_linear_weights(texel0: vec4<u32>, texel1: vec4<u32>) -> vec3<f32> {
    return vec3<f32>(bitcast<f32>(texel0.z), bitcast<f32>(texel0.w), bitcast<f32>(texel1.x));
}

fn get_blur_linear_offsets(texel1: vec4<u32>) -> vec3<f32> {
    return vec3<f32>(bitcast<f32>(texel1.y), bitcast<f32>(texel1.z), bitcast<f32>(texel1.w));
}

fn get_drop_shadow_dx(texel2: vec4<u32>) -> f32 {
    return bitcast<f32>(texel2.x);
}

fn get_drop_shadow_dy(texel2: vec4<u32>) -> f32 {
    return bitcast<f32>(texel2.y);
}

fn get_drop_shadow_color(texel2: vec4<u32>) -> u32 {
    return texel2.z;
}

struct FilterInstanceData {
    @location(0)
    source_origin: u32,
    @location(1)
    source_size: u32,
    @location(2)
    dest_origin: u32,
    @location(3)
    dest_size: u32,
    @location(4)
    dest_texture_size: u32,
    @location(5)
    filter_data_offset: u32,
    @location(6)
    original_origin: u32,
    @location(7)
    original_size: u32,
    @location(8)
    filter_pass_kind: u32
}

struct FilterVertexOutput {
    @builtin(position)
    position: vec4<f32>,
    @location(0) @interpolate(flat)
    filter_data_offset: u32,
    @location(1) @interpolate(flat)
    source_origin: vec2<u32>,
    @location(2) @interpolate(flat)
    source_size: vec2<u32>,
    @location(3) @interpolate(flat)
    dest_origin: vec2<u32>,
    @location(4) @interpolate(flat)
    dest_size: vec2<u32>,
    @location(5) @interpolate(flat)
    original_origin: vec2<u32>,
    @location(6) @interpolate(flat)
    filter_pass_kind: u32
}

fn unpack_u16_pair(value: u32) -> vec2<u32> {
    return vec2<u32>(value & 65535u, value >> 16u);
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32, instance: FilterInstanceData) -> FilterVertexOutput {
    let quad_vertex = vertex_index % 4u;
    let x = f32((quad_vertex & 1u));
    let y = f32((quad_vertex >> 1u));
    let source_origin = unpack_u16_pair(instance.source_origin);
    let source_size = unpack_u16_pair(instance.source_size);
    let dest_origin = unpack_u16_pair(instance.dest_origin);
    let dest_size = unpack_u16_pair(instance.dest_size);
    let dest_texture_size = vec2<f32>(unpack_u16_pair(instance.dest_texture_size));
    let original_origin = unpack_u16_pair(instance.original_origin);
    let original_size = unpack_u16_pair(instance.original_size);
    let render_size = min(original_size, dest_size + vec2(FILTER_ATLAS_PADDING));
    let pix_x = f32(dest_origin.x) + x * f32(render_size.x);
    let pix_y = f32(dest_origin.y) + y * f32(render_size.y);
    let ndc_x = pix_x * 2.0 / dest_texture_size.x - 1.0;
    let ndc_y = 1.0 - pix_y * 2.0 / dest_texture_size.y;
    var out: FilterVertexOutput;
    out.position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    out.filter_data_offset = instance.filter_data_offset;
    out.source_origin = source_origin;
    out.source_size = source_size;
    out.dest_origin = dest_origin;
    out.dest_size = dest_size;
    out.original_origin = original_origin;
    out.filter_pass_kind = instance.filter_pass_kind;
    return out;
}

fn sample_original(original_origin: vec2<u32>, rel_coord: vec2<f32>) -> vec4<f32> {
    let source_coord = vec2<u32>(vec2<i32>(original_origin) + vec2<i32>(rel_coord));
    return textureLoad(original_texture, source_coord, 0);
}

fn sample_source(source_origin: vec2<u32>, rel_coord: vec2<f32>) -> vec4<f32> {
    let source_coord = vec2<u32>(vec2<i32>(source_origin) + vec2<i32>(rel_coord));
    return textureLoad(source_texture, source_coord, 0);
}

fn sample_source_checked(source_origin: vec2<u32>, source_size: vec2<u32>, rel_coord: vec2<f32>) -> vec4<f32> {
    if rel_coord.x < 0.0 || rel_coord.x >= f32(source_size.x) || rel_coord.y < 0.0 || rel_coord.y >= f32(source_size.y) {
        return vec4<f32>(0.0);
    }
    return sample_source(source_origin, rel_coord);
}

fn downscale(position: vec4<f32>, source_origin: vec2<u32>, dest_origin: vec2<u32>) -> vec4<f32> {
    let frag_coord = vec2<u32>(position.xy);
    let rel = vec2<i32>(frag_coord - dest_origin);
    let source_rel = vec2<f32>(rel * 2);
    let source_texel = vec2<f32>(source_origin) + source_rel;
    let source_texture_size = vec2<f32>(textureDimensions(source_texture));
    let lo = vec2<f32>(-0.25);
    let hi = vec2<f32>(1.25);
    let s00 = textureSampleLevel(source_texture, linear_sampler, (source_texel + vec2(lo.x, lo.y) + 0.5) / source_texture_size, 0.0);
    let s01 = textureSampleLevel(source_texture, linear_sampler, (source_texel + vec2(lo.x, hi.y) + 0.5) / source_texture_size, 0.0);
    let s10 = textureSampleLevel(source_texture, linear_sampler, (source_texel + vec2(hi.x, lo.y) + 0.5) / source_texture_size, 0.0);
    let s11 = textureSampleLevel(source_texture, linear_sampler, (source_texel + vec2(hi.x, hi.y) + 0.5) / source_texture_size, 0.0);
    return (s00 + s01 + s10 + s11) * 0.25;
}

fn upscale(position: vec4<f32>, source_origin: vec2<u32>, dest_origin: vec2<u32>) -> vec4<f32> {
    let frag_coord = vec2<u32>(position.xy);
    let rel = vec2<i32>(frag_coord - dest_origin);
    let source_base = vec2<f32>(rel / 2);
    let phase = vec2<f32>(rel % 2);
    let source_texture_size = vec2<f32>(textureDimensions(source_texture));
    let sample_offset = select(vec2(-0.25), vec2(0.25), phase == vec2(1.0));
    let source_texel = vec2<f32>(source_origin) + source_base + sample_offset;
    return textureSampleLevel(source_texture, linear_sampler, (source_texel + 0.5) / source_texture_size, 0.0);
}

fn convolve(source_origin: vec2<u32>, source_rel: vec2<f32>, dir: vec2<f32>, n_linear_taps: u32, center_weight: f32, weights: vec3<f32>, offsets: vec3<f32>) -> vec4<f32> {
    let source_texel = vec2<f32>(source_origin) + source_rel;
    let source_texture_size = vec2<f32>(textureDimensions(source_texture));
    var color = textureSampleLevel(source_texture, linear_sampler, (source_texel + 0.5) / source_texture_size, 0.0) * center_weight;
    var weights_arr: array<f32, 3>;
    weights_arr[0] = weights.x;
    weights_arr[1] = weights.y;
    weights_arr[2] = weights.z;
    var offsets_arr: array<f32, 3>;
    offsets_arr[0] = offsets.x;
    offsets_arr[1] = offsets.y;
    offsets_arr[2] = offsets.z;
    for (var i = 0u; i < n_linear_taps; i++) {
        let w = weights_arr[i];
        let d = dir * offsets_arr[i];
        color += textureSampleLevel(source_texture, linear_sampler, (source_texel + d + 0.5) / source_texture_size, 0.0) * w;
        color += textureSampleLevel(source_texture, linear_sampler, (source_texel - d + 0.5) / source_texture_size, 0.0) * w;
    }
    return color;
}

const HORIZONTAL: vec2<f32> = vec2<f32>(1.0, 0.0);

const VERTICAL: vec2<f32> = vec2<f32>(0.0, 1.0);

@fragment
fn fs_main(@location(0) @interpolate(flat) filter_data_offset: u32, @location(1) @interpolate(flat) source_origin: vec2<u32>, @location(2) @interpolate(flat) source_size: vec2<u32>, @location(3) @interpolate(flat) dest_origin: vec2<u32>, @location(4) @interpolate(flat) dest_size: vec2<u32>, @location(5) @interpolate(flat) original_origin: vec2<u32>, @location(6) @interpolate(flat) filter_pass_kind: u32, @builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let frag_coord = vec2<u32>(position.xy);
    let rel_coord = vec2<f32>(frag_coord - dest_origin);
    if rel_coord.x >= f32(dest_size.x) || rel_coord.y >= f32(dest_size.y) {
        return vec4<f32>(0.0);
    }
    switch filter_pass_kind {
        case PASS_COPY {
            return sample_source(source_origin, rel_coord);
        }
        case PASS_FLOOD {
            let filter_texel0 = load_filter_texel(filter_data_offset, 0u);
            return unpack4x8unorm(get_flood_color(filter_texel0));
        }
        case PASS_OFFSET {
            let filter_texel0 = load_filter_texel(filter_data_offset, 0u);
            var dxdy: vec2<f32>;
            if get_filter_type(filter_texel0) == FILTER_TYPE_DROP_SHADOW {
                let filter_texel2 = load_filter_texel(filter_data_offset, 2u);
                dxdy = vec2<f32>(get_drop_shadow_dx(filter_texel2), get_drop_shadow_dy(filter_texel2));
            }
            else {
                dxdy = vec2<f32>(get_offset_dx(filter_texel0), get_offset_dy(filter_texel0));
            }
            return sample_source_checked(source_origin, source_size, rel_coord - floor(dxdy + 0.5));
        }
        case PASS_DOWNSCALE {
            return downscale(position, source_origin, dest_origin);
        }
        case PASS_BLUR_H {
            let filter_texel0 = load_filter_texel(filter_data_offset, 0u);
            let filter_texel1 = load_filter_texel(filter_data_offset, 1u);
            return convolve(source_origin, rel_coord, HORIZONTAL, get_filter_header_n_linear_taps(filter_texel0), get_blur_center_weight(filter_texel0), get_blur_linear_weights(filter_texel0, filter_texel1), get_blur_linear_offsets(filter_texel1));
        }
        case PASS_BLUR_V {
            let filter_texel0 = load_filter_texel(filter_data_offset, 0u);
            let filter_texel1 = load_filter_texel(filter_data_offset, 1u);
            return convolve(source_origin, rel_coord, VERTICAL, get_filter_header_n_linear_taps(filter_texel0), get_blur_center_weight(filter_texel0), get_blur_linear_weights(filter_texel0, filter_texel1), get_blur_linear_offsets(filter_texel1));
        }
        case PASS_UPSCALE {
            return upscale(position, source_origin, dest_origin);
        }
        case PASS_COMPOSITE_DROP_SHADOW {
            let filter_texel2 = load_filter_texel(filter_data_offset, 2u);
            let blurred = sample_source(source_origin, rel_coord);
            let shadow_color = unpack4x8unorm(get_drop_shadow_color(filter_texel2));
            let shadow_result = shadow_color * blurred.a;
            let original = sample_original(original_origin, rel_coord);
            return original + shadow_result * (1.0 - original.a);
        }
        case PASS_COLORIZE {
            let filter_texel2 = load_filter_texel(filter_data_offset, 2u);
            let blurred = sample_source(source_origin, rel_coord);
            let shadow_color = unpack4x8unorm(get_drop_shadow_color(filter_texel2));
            return shadow_color * blurred.a;
        }
        case default {
            return vec4<f32>(0.0);
        }
    }
}
"###;
    /// Linked WGSL source for `render.wesl`.
    pub const RENDER: &str = r###"const COLOR_SOURCE_PAYLOAD: u32 = 0u;

const COLOR_SOURCE_LAYER: u32 = 1u;

const PAINT_TYPE_SOLID: u32 = 0u;

const PAINT_TYPE_IMAGE: u32 = 1u;

const PAINT_TYPE_LINEAR_GRADIENT: u32 = 2u;

const PAINT_TYPE_RADIAL_GRADIENT: u32 = 3u;

const PAINT_TYPE_SWEEP_GRADIENT: u32 = 4u;

const PAINT_TYPE_BLURRED_ROUNDED_RECT: u32 = 5u;

const PAINT_TEXTURE_INDEX_MASK: u32 = 67108863u;

const RECT_STRIP_FLAG: u32 = 2147483648u;

const IMAGE_QUALITY_LOW = 0u;

const IMAGE_QUALITY_MEDIUM = 1u;

const IMAGE_QUALITY_HIGH = 2u;

const IMAGE_SOURCE_ATLAS = 0u;

const IMAGE_SOURCE_EXTERNAL = 1u;

const GRADIENT_TYPE_LINEAR: u32 = 0u;

const GRADIENT_TYPE_RADIAL: u32 = 1u;

const GRADIENT_TYPE_SWEEP: u32 = 2u;

const RADIAL_GRADIENT_TYPE_STANDARD: u32 = 0u;

const RADIAL_GRADIENT_TYPE_STRIP: u32 = 1u;

const RADIAL_GRADIENT_TYPE_FOCAL: u32 = 2u;

const PI: f32 = 3.141592653589793;

const TWO_PI: f32 = 2.0 * PI;

const NEARLY_ZERO_TOLERANCE: f32 = 1.0 / 4096.0;

struct Config {
    width: u32,
    height: u32,
    strip_height: u32,
    alphas_tex_width_bits: u32,
    encoded_paints_tex_width_bits: u32,
    strip_offset_x: i32,
    strip_offset_y: i32,
    ndc_y_negate: u32
}

struct StripInstance {
    @location(0)
    xy: u32,
    @location(1)
    widths_or_rect_height: u32,
    @location(2)
    col_idx_or_rect_frac: u32,
    @location(3)
    payload: u32,
    @location(4)
    paint_and_rect_flag: u32,
    @location(5)
    depth_index: u32
}

struct VertexOutput {
    @location(0) @interpolate(flat)
    paint_and_rect_flag: u32,
    @location(1)
    tex_coord: vec2<f32>,
    @location(2)
    sample_xy: vec2<f32>,
    @location(3) @interpolate(flat)
    dense_end_or_rect_size: u32,
    @location(4) @interpolate(flat)
    payload: u32,
    @location(5) @interpolate(flat)
    rect_frac: u32,
    @builtin(position)
    position: vec4<f32>
}

@group(0) @binding(1)
var<uniform> config: Config;

@group(1) @binding(0)
var atlas_texture_array: texture_2d_array<f32>;

@group(1) @binding(1)
var external_texture: texture_2d<f32>;

@group(2) @binding(0)
var encoded_paints_texture: texture_2d<u32>;

@group(3) @binding(0)
var gradient_texture: texture_2d<f32>;

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32, instance: StripInstance) -> VertexOutput {
    var out: VertexOutput;
    out.sample_xy = vec2(0.0);
    let x = f32(in_vertex_index & 1u);
    let y = f32(in_vertex_index >> 1u);
    let x0 = instance.xy & 65535u;
    let y0 = instance.xy >> 16u;
    let width = instance.widths_or_rect_height & 65535u;
    let dense_width = instance.widths_or_rect_height >> 16u;
    let is_rect = (instance.paint_and_rect_flag & RECT_STRIP_FLAG) != 0u;
    var height = config.strip_height;
    if is_rect {
        height = dense_width;
        out.dense_end_or_rect_size = width | (dense_width << 16u);
        out.rect_frac = instance.col_idx_or_rect_frac;
    }
    else {
        out.dense_end_or_rect_size = instance.col_idx_or_rect_frac + dense_width;
        out.rect_frac = 0u;
    }
    let pix_x = f32(i32(x0) + config.strip_offset_x) + x * f32(width);
    let pix_y = f32(i32(y0) + config.strip_offset_y) + y * f32(height);
    let ndc_x = pix_x * 2.0 / f32(config.width) - 1.0;
    let ndc_y = 1.0 - pix_y * 2.0 / f32(config.height);
    let color_source = (instance.paint_and_rect_flag >> 29u) & 3u;
    if color_source == COLOR_SOURCE_PAYLOAD {
        let paint_type = (instance.paint_and_rect_flag >> 26u) & 7u;
        let scene_strip_x = instance.payload & 65535u;
        let scene_strip_y = instance.payload >> 16u;
        if paint_type == PAINT_TYPE_IMAGE {
            let paint_tex_idx = instance.paint_and_rect_flag & PAINT_TEXTURE_INDEX_MASK;
            let image_texel0 = load_encoded_paint_texel(paint_tex_idx, 0u);
            let image_texel1 = load_encoded_paint_texel(paint_tex_idx, 1u);
            let image_texel2 = load_encoded_paint_texel(paint_tex_idx, 2u);
            let pos = vec2<f32>(f32(scene_strip_x) + x * f32(width), f32(scene_strip_y) + y * f32(height));
            out.sample_xy = get_image_translate(image_texel1, image_texel2) + get_image_offset(image_texel0) + get_image_transform(image_texel0, image_texel1) * pos;
        }
        else if paint_type == PAINT_TYPE_LINEAR_GRADIENT || paint_type == PAINT_TYPE_RADIAL_GRADIENT || paint_type == PAINT_TYPE_SWEEP_GRADIENT || paint_type == PAINT_TYPE_BLURRED_ROUNDED_RECT {
            out.sample_xy = vec2<f32>(f32(scene_strip_x) + x * f32(width), f32(scene_strip_y) + y * f32(height));
        }
    }
    else if color_source == COLOR_SOURCE_LAYER {
        let src_x = instance.payload & 65535u;
        let src_y = instance.payload >> 16u;
        out.sample_xy = vec2<f32>(f32(src_x) + x * f32(width), f32(src_y) + y * f32(height));
    }
    let col_offset = select(f32(instance.col_idx_or_rect_frac), 0.0, is_rect);
    out.tex_coord = vec2<f32>(col_offset + x * f32(width), y * f32(height));
    let z = 1.0 - f32(instance.depth_index) / f32(1u << 24u);
    let final_ndc_y = select(ndc_y, -ndc_y, config.ndc_y_negate != 0u);
    out.position = vec4<f32>(ndc_x, final_ndc_y, z, 1.0);
    out.payload = instance.payload;
    out.paint_and_rect_flag = instance.paint_and_rect_flag;
    return out;
}

@group(0) @binding(0)
var alphas_texture: texture_2d<u32>;

@group(0) @binding(2)
var layer_input_texture: texture_2d<f32>;

@fragment
fn fs_main(@location(0) @interpolate(flat) paint_and_rect_flag: u32, @location(1) tex_coord: vec2<f32>, @location(2) sample_xy: vec2<f32>, @location(3) @interpolate(flat) dense_end_or_rect_size: u32, @location(4) @interpolate(flat) payload: u32, @location(5) @interpolate(flat) rect_frac: u32, @builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    var alpha = 1.0;
    let is_rect = (paint_and_rect_flag & RECT_STRIP_FLAG) != 0u;
    if is_rect && rect_frac != 0u {
        let frac = unpack4x8unorm(rect_frac);
        let rect_size = vec2<f32>(f32(dense_end_or_rect_size & 65535u), f32(dense_end_or_rect_size >> 16u));
        let tc = tex_coord;
        let bottom_and_right = min(tc + 0.5, rect_size - frac.zw);
        let top_and_left = max(tc - 0.5, frac.xy);
        let a = clamp(bottom_and_right - top_and_left, vec2(0.0), vec2(1.0));
        alpha = a.x * a.y;
    }
    else if !is_rect && dense_end_or_rect_size != 0u {
        let x = u32(floor(tex_coord.x));
        let y = u32(floor(tex_coord.y));
        let alphas_index = x;
        let tex_dimensions = textureDimensions(alphas_texture);
        let alphas_tex_width = tex_dimensions.x;
        let texel_index = alphas_index / 4u;
        let channel_index = alphas_index % 4u;
        let tex_x = texel_index & (alphas_tex_width - 1u);
        let tex_y = texel_index >> config.alphas_tex_width_bits;
        let rgba_values = textureLoad(alphas_texture, vec2<u32>(tex_x, tex_y), 0);
        let alphas_u32 = unpack_alphas_from_channel(rgba_values, channel_index);
        alpha = f32((alphas_u32 >> (y * 8u)) & 255u) * (1.0 / 255.0);
    }
    let color_source = (paint_and_rect_flag >> 29u) & 3u;
    var final_color: vec4<f32>;
    if color_source == COLOR_SOURCE_PAYLOAD {
        let paint_type = (paint_and_rect_flag >> 26u) & 7u;
        if paint_type == PAINT_TYPE_SOLID {
            final_color = alpha * unpack4x8unorm(payload);
        }
        else if paint_type == PAINT_TYPE_IMAGE {
            let paint_tex_idx = paint_and_rect_flag & PAINT_TEXTURE_INDEX_MASK;
            let image_texel0 = load_encoded_paint_texel(paint_tex_idx, 0u);
            let image_texel1 = load_encoded_paint_texel(paint_tex_idx, 1u);
            let image_texel2 = load_encoded_paint_texel(paint_tex_idx, 2u);
            let image_offset = get_image_offset(image_texel0);
            let image_size = get_image_size(image_texel0);
            let image_extend_modes = get_image_extend_modes(image_texel0);
            let image_atlas_index = get_image_atlas_index(image_texel0);
            let image_quality = get_image_quality(image_texel0);
            let image_source_kind = get_image_source_kind(image_texel0);
            let image_padding = get_image_padding(image_texel2);
            let packed_tint = image_texel2.y;
            let image_tint = unpack4x8unorm(packed_tint);
            let is_multiply = image_texel2.z == TINT_MODE_MULTIPLY;
            let local_xy = sample_xy - image_offset;
            let offset = 1e-5;
            let extended_xy = vec2<f32>(extend_mode(local_xy.x + offset, image_extend_modes.x, image_size.x), extend_mode(local_xy.y + offset, image_extend_modes.y, image_size.y));
            var sample_color: vec4<f32>;
            if image_source_kind == IMAGE_SOURCE_EXTERNAL {
                let final_xy = image_offset + extended_xy;
                sample_color = sample_external_image(image_quality, final_xy, image_offset, image_size);
            }
            else if image_quality == IMAGE_QUALITY_HIGH {
                let final_xy = image_offset + extended_xy;
                sample_color = bicubic_sample(atlas_texture_array, final_xy, i32(image_atlas_index), image_offset, image_size, image_extend_modes, image_padding);
            }
            else if image_quality == IMAGE_QUALITY_MEDIUM {
                let final_xy = image_offset + extended_xy - vec2(0.5);
                sample_color = bilinear_sample(atlas_texture_array, final_xy, i32(image_atlas_index), image_offset, image_size, image_extend_modes, image_padding);
            }
            else {
                let final_xy = image_offset + extended_xy;
                sample_color = textureLoad(atlas_texture_array, vec2<u32>(final_xy), i32(image_atlas_index), 0);
            }
            final_color = alpha * select(image_tint * sample_color.a, sample_color * image_tint, is_multiply);
        }
        else if paint_type == PAINT_TYPE_LINEAR_GRADIENT {
            let paint_tex_idx = paint_and_rect_flag & PAINT_TEXTURE_INDEX_MASK;
            let gradient_texel0 = load_encoded_paint_texel(paint_tex_idx, 0u);
            let gradient_texel1 = load_encoded_paint_texel(paint_tex_idx, 1u);
            let fragment_pos = sample_xy;
            let grad_pos = apply_gradient_transform(gradient_texel0, gradient_texel1, fragment_pos);
            let t_value = grad_pos.x + 1e-5;
            let gradient_color = sample_gradient_lut(t_value, get_gradient_extend_mode(gradient_texel0), get_gradient_start(gradient_texel0), get_gradient_texture_width(gradient_texel0));
            final_color = alpha * gradient_color;
        }
        else if paint_type == PAINT_TYPE_RADIAL_GRADIENT {
            let paint_tex_idx = paint_and_rect_flag & PAINT_TEXTURE_INDEX_MASK;
            let gradient_texel0 = load_encoded_paint_texel(paint_tex_idx, 0u);
            let gradient_texel1 = load_encoded_paint_texel(paint_tex_idx, 1u);
            let gradient_texel2 = load_encoded_paint_texel(paint_tex_idx, 2u);
            let gradient_texel3 = load_encoded_paint_texel(paint_tex_idx, 3u);
            let fragment_pos = sample_xy;
            let grad_pos = apply_gradient_transform(gradient_texel0, gradient_texel1, fragment_pos);
            let gradient_result = calculate_radial_gradient(grad_pos, gradient_texel2, gradient_texel3);
            let gradient_color = sample_gradient_lut(gradient_result.x, get_gradient_extend_mode(gradient_texel0), get_gradient_start(gradient_texel0), get_gradient_texture_width(gradient_texel0));
            final_color = select(vec4<f32>(0.0, 0.0, 0.0, 0.0), alpha * gradient_color, gradient_result.y != 0.0);
        }
        else if paint_type == PAINT_TYPE_SWEEP_GRADIENT {
            let paint_tex_idx = paint_and_rect_flag & PAINT_TEXTURE_INDEX_MASK;
            let gradient_texel0 = load_encoded_paint_texel(paint_tex_idx, 0u);
            let gradient_texel1 = load_encoded_paint_texel(paint_tex_idx, 1u);
            let gradient_texel2 = load_encoded_paint_texel(paint_tex_idx, 2u);
            let fragment_pos = sample_xy;
            var grad_pos = apply_gradient_transform(gradient_texel0, gradient_texel1, fragment_pos);
            grad_pos = select(grad_pos, vec2(0.0), abs(grad_pos) < vec2(NEARLY_ZERO_TOLERANCE));
            let unit_angle = xy_to_unit_angle(grad_pos.x, grad_pos.y);
            let angle = unit_angle * TWO_PI;
            let t_value = (angle - get_sweep_start_angle(gradient_texel2)) * get_sweep_inv_angle_delta(gradient_texel2);
            let gradient_color = sample_gradient_lut(t_value, get_gradient_extend_mode(gradient_texel0), get_gradient_start(gradient_texel0), get_gradient_texture_width(gradient_texel0));
            final_color = alpha * gradient_color;
        }
        else if paint_type == PAINT_TYPE_BLURRED_ROUNDED_RECT {
            let paint_tex_idx = paint_and_rect_flag & PAINT_TEXTURE_INDEX_MASK;
            let blurred_texel0 = load_encoded_paint_texel(paint_tex_idx, 0u);
            let blurred_texel1 = load_encoded_paint_texel(paint_tex_idx, 1u);
            let blurred_texel2 = load_encoded_paint_texel(paint_tex_idx, 2u);
            let blurred_texel3 = load_encoded_paint_texel(paint_tex_idx, 3u);
            let blurred_texel4 = load_encoded_paint_texel(paint_tex_idx, 4u);
            final_color = alpha * calculate_blurred_rounded_rect(sample_xy, blurred_texel0, blurred_texel1, blurred_texel2, blurred_texel3, blurred_texel4);
        }
    }
    else if color_source == COLOR_SOURCE_LAYER {
        let layer_opacity = f32(paint_and_rect_flag & 255u) * (1.0 / 255.0);
        final_color = alpha * layer_opacity * textureLoad(layer_input_texture, vec2<i32>(sample_xy), 0);
    }
    else {
        final_color = vec4<f32>(0.0);
    }
    return final_color;
}

const TINT_MODE_ALPHA_MASK: u32 = 0u;

const TINT_MODE_MULTIPLY: u32 = 1u;

fn encoded_paint_coord(flat_idx: u32) -> vec2<u32> {
    return vec2<u32>(flat_idx & ((1u << config.encoded_paints_tex_width_bits) - 1u), flat_idx >> config.encoded_paints_tex_width_bits);
}

fn load_encoded_paint_texel(paint_tex_idx: u32, texel_offset: u32) -> vec4<u32> {
    return textureLoad(encoded_paints_texture, encoded_paint_coord(paint_tex_idx + texel_offset), 0);
}

fn get_image_quality(texel0: vec4<u32>) -> u32 {
    return texel0.x & 3u;
}

fn get_image_extend_modes(texel0: vec4<u32>) -> vec2<u32> {
    return vec2<u32>((texel0.x >> 2u) & 3u, (texel0.x >> 4u) & 3u);
}

fn get_image_size(texel0: vec4<u32>) -> vec2<f32> {
    return vec2<f32>(f32(texel0.y >> 16u), f32(texel0.y & 65535u));
}

fn get_image_offset(texel0: vec4<u32>) -> vec2<f32> {
    return vec2<f32>(f32(texel0.z >> 16u), f32(texel0.z & 65535u));
}

fn get_image_atlas_index(texel0: vec4<u32>) -> u32 {
    return (texel0.x >> 6u) & 255u;
}

fn get_image_source_kind(texel0: vec4<u32>) -> u32 {
    return (texel0.x >> 14u) & 1u;
}

fn get_image_transform(texel0: vec4<u32>, texel1: vec4<u32>) -> mat2x2<f32> {
    return mat2x2<f32>(vec2<f32>(bitcast<f32>(texel0.w), bitcast<f32>(texel1.x)), vec2<f32>(bitcast<f32>(texel1.y), bitcast<f32>(texel1.z)));
}

fn get_image_translate(texel1: vec4<u32>, texel2: vec4<u32>) -> vec2<f32> {
    return vec2<f32>(bitcast<f32>(texel1.w), bitcast<f32>(texel2.x));
}

fn get_image_padding(texel2: vec4<u32>) -> f32 {
    return f32(texel2.w);
}

fn unpack_alphas_from_channel(rgba: vec4<u32>, channel_index: u32) -> u32 {
    switch channel_index {
        case 0u {
            return rgba.x;
        }
        case 1u {
            return rgba.y;
        }
        case 2u {
            return rgba.z;
        }
        case 3u {
            return rgba.w;
        }
        case default {
            return rgba.x;
        }
    }
}

const EXTEND_PAD: u32 = 0u;

const EXTEND_REPEAT: u32 = 1u;

const EXTEND_REFLECT: u32 = 2u;

fn extend_mode(t: f32, mode: u32, max: f32) -> f32 {
    switch mode {
        case EXTEND_PAD {
            return clamp(t, 0.5, max - 0.5);
        }
        case EXTEND_REPEAT {
            return extend_mode_normalized(t / max, mode) * max;
        }
        case EXTEND_REFLECT, default {
            return extend_mode_normalized(t / max, mode) * max;
        }
    }
}

fn extend_mode_normalized(t: f32, mode: u32) -> f32 {
    switch mode {
        case EXTEND_PAD {
            return clamp(t, 0.0, 1.0);
        }
        case EXTEND_REPEAT {
            return fract(t);
        }
        case EXTEND_REFLECT, default {
            return abs(t - 2.0 * round(0.5 * t));
        }
    }
}

fn bilinear_sample(tex: texture_2d_array<f32>, coords: vec2<f32>, atlas_idx: i32, image_offset: vec2<f32>, image_size: vec2<f32>, _extend_modes: vec2<u32>, _image_padding: f32) -> vec4<f32> {
    let atlas_max = image_offset + image_size - vec2(1.0);
    let atlas_uv_clamped = clamp(coords, image_offset, atlas_max);
    let uv_quad = vec4(floor(atlas_uv_clamped), ceil(atlas_uv_clamped));
    let uv_frac = fract(coords);
    let a = textureLoad(tex, vec2<i32>(uv_quad.xy), atlas_idx, 0);
    let b = textureLoad(tex, vec2<i32>(uv_quad.xw), atlas_idx, 0);
    let c = textureLoad(tex, vec2<i32>(uv_quad.zy), atlas_idx, 0);
    let d = textureLoad(tex, vec2<i32>(uv_quad.zw), atlas_idx, 0);
    return mix(mix(a, b, uv_frac.y), mix(c, d, uv_frac.y), uv_frac.x);
}

fn external_bilinear_sample(coords: vec2<f32>, image_offset: vec2<f32>, image_size: vec2<f32>) -> vec4<f32> {
    let atlas_max = image_offset + image_size - vec2(1.0);
    let atlas_uv_clamped = clamp(coords, image_offset, atlas_max);
    let uv_quad = vec4(floor(atlas_uv_clamped), ceil(atlas_uv_clamped));
    let uv_frac = fract(coords);
    let a = textureLoad(external_texture, vec2<i32>(uv_quad.xy), 0);
    let b = textureLoad(external_texture, vec2<i32>(uv_quad.xw), 0);
    let c = textureLoad(external_texture, vec2<i32>(uv_quad.zy), 0);
    let d = textureLoad(external_texture, vec2<i32>(uv_quad.zw), 0);
    return mix(mix(a, b, uv_frac.y), mix(c, d, uv_frac.y), uv_frac.x);
}

fn bicubic_sample(tex: texture_2d_array<f32>, coords: vec2<f32>, atlas_idx: i32, image_offset: vec2<f32>, image_size: vec2<f32>, _extend_modes: vec2<u32>, _image_padding: f32) -> vec4<f32> {
    let atlas_max = image_offset + image_size - vec2(1.0);
    let frac_coords = fract(coords + 0.5);
    let cx = cubic_weights(frac_coords.x);
    let cy = cubic_weights(frac_coords.y);
    let s00 = textureLoad(tex, vec2<i32>(clamp(coords + vec2(-1.5, -1.5), image_offset, atlas_max)), atlas_idx, 0);
    let s10 = textureLoad(tex, vec2<i32>(clamp(coords + vec2(-0.5, -1.5), image_offset, atlas_max)), atlas_idx, 0);
    let s20 = textureLoad(tex, vec2<i32>(clamp(coords + vec2(0.5, -1.5), image_offset, atlas_max)), atlas_idx, 0);
    let s30 = textureLoad(tex, vec2<i32>(clamp(coords + vec2(1.5, -1.5), image_offset, atlas_max)), atlas_idx, 0);
    let s01 = textureLoad(tex, vec2<i32>(clamp(coords + vec2(-1.5, -0.5), image_offset, atlas_max)), atlas_idx, 0);
    let s11 = textureLoad(tex, vec2<i32>(clamp(coords + vec2(-0.5, -0.5), image_offset, atlas_max)), atlas_idx, 0);
    let s21 = textureLoad(tex, vec2<i32>(clamp(coords + vec2(0.5, -0.5), image_offset, atlas_max)), atlas_idx, 0);
    let s31 = textureLoad(tex, vec2<i32>(clamp(coords + vec2(1.5, -0.5), image_offset, atlas_max)), atlas_idx, 0);
    let s02 = textureLoad(tex, vec2<i32>(clamp(coords + vec2(-1.5, 0.5), image_offset, atlas_max)), atlas_idx, 0);
    let s12 = textureLoad(tex, vec2<i32>(clamp(coords + vec2(-0.5, 0.5), image_offset, atlas_max)), atlas_idx, 0);
    let s22 = textureLoad(tex, vec2<i32>(clamp(coords + vec2(0.5, 0.5), image_offset, atlas_max)), atlas_idx, 0);
    let s32 = textureLoad(tex, vec2<i32>(clamp(coords + vec2(1.5, 0.5), image_offset, atlas_max)), atlas_idx, 0);
    let s03 = textureLoad(tex, vec2<i32>(clamp(coords + vec2(-1.5, 1.5), image_offset, atlas_max)), atlas_idx, 0);
    let s13 = textureLoad(tex, vec2<i32>(clamp(coords + vec2(-0.5, 1.5), image_offset, atlas_max)), atlas_idx, 0);
    let s23 = textureLoad(tex, vec2<i32>(clamp(coords + vec2(0.5, 1.5), image_offset, atlas_max)), atlas_idx, 0);
    let s33 = textureLoad(tex, vec2<i32>(clamp(coords + vec2(1.5, 1.5), image_offset, atlas_max)), atlas_idx, 0);
    let row0 = cx.x * s00 + cx.y * s10 + cx.z * s20 + cx.w * s30;
    let row1 = cx.x * s01 + cx.y * s11 + cx.z * s21 + cx.w * s31;
    let row2 = cx.x * s02 + cx.y * s12 + cx.z * s22 + cx.w * s32;
    let row3 = cx.x * s03 + cx.y * s13 + cx.z * s23 + cx.w * s33;
    let result = cy.x * row0 + cy.y * row1 + cy.z * row2 + cy.w * row3;
    let a = clamp(result.a, 0.0, 1.0);
    return vec4<f32>(clamp(result.rgb, vec3(0.0), vec3(a)), a);
}

fn external_bicubic_sample(coords: vec2<f32>, image_offset: vec2<f32>, image_size: vec2<f32>) -> vec4<f32> {
    let atlas_max = image_offset + image_size - vec2(1.0);
    let frac_coords = fract(coords + 0.5);
    let cx = cubic_weights(frac_coords.x);
    let cy = cubic_weights(frac_coords.y);
    let s00 = textureLoad(external_texture, vec2<i32>(clamp(coords + vec2(-1.5, -1.5), image_offset, atlas_max)), 0);
    let s10 = textureLoad(external_texture, vec2<i32>(clamp(coords + vec2(-0.5, -1.5), image_offset, atlas_max)), 0);
    let s20 = textureLoad(external_texture, vec2<i32>(clamp(coords + vec2(0.5, -1.5), image_offset, atlas_max)), 0);
    let s30 = textureLoad(external_texture, vec2<i32>(clamp(coords + vec2(1.5, -1.5), image_offset, atlas_max)), 0);
    let s01 = textureLoad(external_texture, vec2<i32>(clamp(coords + vec2(-1.5, -0.5), image_offset, atlas_max)), 0);
    let s11 = textureLoad(external_texture, vec2<i32>(clamp(coords + vec2(-0.5, -0.5), image_offset, atlas_max)), 0);
    let s21 = textureLoad(external_texture, vec2<i32>(clamp(coords + vec2(0.5, -0.5), image_offset, atlas_max)), 0);
    let s31 = textureLoad(external_texture, vec2<i32>(clamp(coords + vec2(1.5, -0.5), image_offset, atlas_max)), 0);
    let s02 = textureLoad(external_texture, vec2<i32>(clamp(coords + vec2(-1.5, 0.5), image_offset, atlas_max)), 0);
    let s12 = textureLoad(external_texture, vec2<i32>(clamp(coords + vec2(-0.5, 0.5), image_offset, atlas_max)), 0);
    let s22 = textureLoad(external_texture, vec2<i32>(clamp(coords + vec2(0.5, 0.5), image_offset, atlas_max)), 0);
    let s32 = textureLoad(external_texture, vec2<i32>(clamp(coords + vec2(1.5, 0.5), image_offset, atlas_max)), 0);
    let s03 = textureLoad(external_texture, vec2<i32>(clamp(coords + vec2(-1.5, 1.5), image_offset, atlas_max)), 0);
    let s13 = textureLoad(external_texture, vec2<i32>(clamp(coords + vec2(-0.5, 1.5), image_offset, atlas_max)), 0);
    let s23 = textureLoad(external_texture, vec2<i32>(clamp(coords + vec2(0.5, 1.5), image_offset, atlas_max)), 0);
    let s33 = textureLoad(external_texture, vec2<i32>(clamp(coords + vec2(1.5, 1.5), image_offset, atlas_max)), 0);
    let row0 = cx.x * s00 + cx.y * s10 + cx.z * s20 + cx.w * s30;
    let row1 = cx.x * s01 + cx.y * s11 + cx.z * s21 + cx.w * s31;
    let row2 = cx.x * s02 + cx.y * s12 + cx.z * s22 + cx.w * s32;
    let row3 = cx.x * s03 + cx.y * s13 + cx.z * s23 + cx.w * s33;
    let result = cy.x * row0 + cy.y * row1 + cy.z * row2 + cy.w * row3;
    let a = clamp(result.a, 0.0, 1.0);
    return vec4<f32>(clamp(result.rgb, vec3(0.0), vec3(a)), a);
}

fn sample_external_image(quality: u32, coords: vec2<f32>, image_offset: vec2<f32>, image_size: vec2<f32>) -> vec4<f32> {
    if quality == IMAGE_QUALITY_HIGH {
        return external_bicubic_sample(coords, image_offset, image_size);
    }
    if quality == IMAGE_QUALITY_MEDIUM {
        return external_bilinear_sample(coords - vec2(0.5), image_offset, image_size);
    }
    return textureLoad(external_texture, vec2<u32>(coords), 0);
}

const MF: array<vec4<f32>, 4> = array<vec4<f32>, 4>(vec4<f32>((1.0 / 6.0) / 3.0, -(3.0 / 6.0) / 3.0 - 1.0 / 3.0, (3.0 / 6.0) / 3.0 + 2.0 * 1.0 / 3.0, -(1.0 / 6.0) / 3.0 - 1.0 / 3.0), vec4<f32>(1.0 - (2.0 / 6.0) / 3.0, 0.0, -3.0 + (12.0 / 6.0) / 3.0 + 1.0 / 3.0, 2.0 - (9.0 / 6.0) / 3.0 - 1.0 / 3.0), vec4<f32>((1.0 / 6.0) / 3.0, (3.0 / 6.0) / 3.0 + 1.0 / 3.0, 3.0 - (15.0 / 6.0) / 3.0 - 2.0 * 1.0 / 3.0, -2.0 + (9.0 / 6.0) / 3.0 + 1.0 / 3.0), vec4<f32>(0.0, 0.0, -1.0 / 3.0, (1.0 / 6.0) / 3.0 + 1.0 / 3.0));

fn cubic_weights(fract: f32) -> vec4<f32> {
    return vec4<f32>(single_weight(fract, MF[0][0], MF[0][1], MF[0][2], MF[0][3]), single_weight(fract, MF[1][0], MF[1][1], MF[1][2], MF[1][3]), single_weight(fract, MF[2][0], MF[2][1], MF[2][2], MF[2][3]), single_weight(fract, MF[3][0], MF[3][1], MF[3][2], MF[3][3]));
}

fn single_weight(t: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
    return t * (t * (t * d + c) + b) + a;
}

fn xy_to_unit_angle(x: f32, y: f32) -> f32 {
    let xabs = abs(x);
    let yabs = abs(y);
    let slope = min(xabs, yabs) / max(xabs, yabs);
    let s = slope * slope;
    var phi = slope * (0.15912117063999176 + s * (-0.0518539696931839 + s * (0.0247610192745924 + s * (-0.007054738234728575))));
    phi = select(phi, 0.25 - phi, xabs < yabs);
    phi = select(phi, 0.5 - phi, x < 0.0);
    phi = select(phi, 1.0 - phi, y < 0.0);
    phi = select(phi, 0.0, phi != phi);
    return phi;
}

fn sample_gradient_lut(t_value: f32, extend_mode: u32, gradient_start: u32, texture_width: u32) -> vec4<f32> {
    let clamped_t = extend_mode_normalized(t_value, extend_mode);
    let t_offset = u32(clamped_t * f32(texture_width - 1u));
    let flat_coord = gradient_start + t_offset;
    let gradient_tex_width = textureDimensions(gradient_texture).x;
    let tex_x = flat_coord % gradient_tex_width;
    let tex_y = flat_coord / gradient_tex_width;
    let gradient_color = textureLoad(gradient_texture, vec2<u32>(tex_x, tex_y), 0);
    return gradient_color;
}

fn get_gradient_texture_width(texel0: vec4<u32>) -> u32 {
    return texel0.x & 268435455u;
}

fn get_gradient_extend_mode(texel0: vec4<u32>) -> u32 {
    return (texel0.x >> 30u) & 3u;
}

fn get_gradient_start(texel0: vec4<u32>) -> u32 {
    return texel0.y;
}

fn get_gradient_transform(texel0: vec4<u32>, texel1: vec4<u32>) -> mat2x2<f32> {
    return mat2x2<f32>(vec2<f32>(bitcast<f32>(texel0.z), bitcast<f32>(texel0.w)), vec2<f32>(bitcast<f32>(texel1.x), bitcast<f32>(texel1.y)));
}

fn get_gradient_translate(texel1: vec4<u32>) -> vec2<f32> {
    return vec2<f32>(bitcast<f32>(texel1.z), bitcast<f32>(texel1.w));
}

fn apply_gradient_transform(texel0: vec4<u32>, texel1: vec4<u32>, fragment_pos: vec2<f32>) -> vec2<f32> {
    return get_gradient_transform(texel0, texel1) * fragment_pos + get_gradient_translate(texel1);
}

fn get_radial_kind(texel2: vec4<u32>) -> u32 {
    return texel2.x & 3u;
}

fn get_radial_f_is_swapped(texel2: vec4<u32>) -> u32 {
    return (texel2.x >> 2u) & 1u;
}

fn get_radial_bias(texel2: vec4<u32>) -> f32 {
    return bitcast<f32>(texel2.y);
}

fn get_radial_scale(texel2: vec4<u32>) -> f32 {
    return bitcast<f32>(texel2.z);
}

fn get_radial_fp0(texel2: vec4<u32>) -> f32 {
    return bitcast<f32>(texel2.w);
}

fn get_radial_fp1(texel3: vec4<u32>) -> f32 {
    return bitcast<f32>(texel3.x);
}

fn get_radial_fr1(texel3: vec4<u32>) -> f32 {
    return bitcast<f32>(texel3.y);
}

fn get_radial_f_focal_x(texel3: vec4<u32>) -> f32 {
    return bitcast<f32>(texel3.z);
}

fn get_radial_scaled_r0_squared(texel3: vec4<u32>) -> f32 {
    return bitcast<f32>(texel3.w);
}

fn get_sweep_start_angle(texel2: vec4<u32>) -> f32 {
    return bitcast<f32>(texel2.x);
}

fn get_sweep_inv_angle_delta(texel2: vec4<u32>) -> f32 {
    return bitcast<f32>(texel2.y);
}

fn get_blurred_rounded_rect_transform(texel0: vec4<u32>) -> mat2x2<f32> {
    return mat2x2<f32>(vec2<f32>(bitcast<f32>(texel0.x), bitcast<f32>(texel0.y)), vec2<f32>(bitcast<f32>(texel0.z), bitcast<f32>(texel0.w)));
}

fn get_blurred_rounded_rect_translate(texel1: vec4<u32>) -> vec2<f32> {
    return vec2<f32>(bitcast<f32>(texel1.x), bitcast<f32>(texel1.y));
}

fn get_blurred_rounded_rect_color(texel1: vec4<u32>) -> vec4<f32> {
    return unpack4x8unorm(texel1.z);
}

fn get_blurred_rounded_rect_invert(texel1: vec4<u32>) -> u32 {
    return texel1.w;
}

fn get_blurred_rounded_rect_exponent(texel2: vec4<u32>) -> f32 {
    return bitcast<f32>(texel2.x);
}

fn get_blurred_rounded_rect_recip_exponent(texel2: vec4<u32>) -> f32 {
    return bitcast<f32>(texel2.y);
}

fn get_blurred_rounded_rect_scale(texel2: vec4<u32>) -> f32 {
    return bitcast<f32>(texel2.z);
}

fn get_blurred_rounded_rect_std_dev_inv(texel2: vec4<u32>) -> f32 {
    return bitcast<f32>(texel2.w);
}

fn get_blurred_rounded_rect_min_edge(texel3: vec4<u32>) -> f32 {
    return bitcast<f32>(texel3.x);
}

fn get_blurred_rounded_rect_w(texel3: vec4<u32>) -> f32 {
    return bitcast<f32>(texel3.y);
}

fn get_blurred_rounded_rect_h(texel3: vec4<u32>) -> f32 {
    return bitcast<f32>(texel3.z);
}

fn get_blurred_rounded_rect_r1(texel3: vec4<u32>) -> f32 {
    return bitcast<f32>(texel3.w);
}

fn get_blurred_rounded_rect_width(texel4: vec4<u32>) -> f32 {
    return bitcast<f32>(texel4.x);
}

fn get_blurred_rounded_rect_height(texel4: vec4<u32>) -> f32 {
    return bitcast<f32>(texel4.y);
}

fn calculate_radial_gradient(grad_pos: vec2<f32>, texel2: vec4<u32>, texel3: vec4<u32>) -> vec2<f32> {
    let x_pos = grad_pos.x;
    let y_pos = grad_pos.y;
    var t_value: f32;
    var is_valid: bool;
    let kind = get_radial_kind(texel2);
    switch kind {
        case RADIAL_GRADIENT_TYPE_STANDARD {
            let radius = sqrt(x_pos * x_pos + y_pos * y_pos);
            t_value = get_radial_bias(texel2) + get_radial_scale(texel2) * radius;
            is_valid = true;
        }
        case RADIAL_GRADIENT_TYPE_STRIP {
            let p1 = get_radial_scaled_r0_squared(texel3) - y_pos * y_pos;
            is_valid = p1 >= 0.0;
            if is_valid {
                t_value = x_pos + sqrt(p1);
            }
            else {
                t_value = 0.0;
            }
        }
        case RADIAL_GRADIENT_TYPE_FOCAL, default {
            var t = 0.0;
            let fp0 = get_radial_fp0(texel2);
            let fp1 = get_radial_fp1(texel3);
            let fr1 = get_radial_fr1(texel3);
            let f_focal_x = get_radial_f_focal_x(texel3);
            let is_swapped = get_radial_f_is_swapped(texel2);
            let is_focal_on_circle = abs(1.0 - fr1) <= NEARLY_ZERO_TOLERANCE;
            let is_well_behaved = !is_focal_on_circle && fr1 > 1.0;
            let is_natively_focal = abs(f_focal_x) <= NEARLY_ZERO_TOLERANCE;
            is_valid = true;
            if is_focal_on_circle {
                t = x_pos + y_pos * y_pos / x_pos;
                is_valid = t >= 0.0 && x_pos != 0.0;
            }
            else if is_well_behaved {
                t = sqrt(x_pos * x_pos + y_pos * y_pos) - x_pos * fp0;
            }
            else {
                let xx = x_pos * x_pos;
                let yy = y_pos * y_pos;
                let discriminant = xx - yy;
                if is_swapped != 0u || (1.0 - f_focal_x < 0.0) {
                    t = -sqrt(discriminant) - x_pos * fp0;
                }
                else {
                    t = sqrt(discriminant) - x_pos * fp0;
                }
                is_valid = discriminant >= 0.0 && t >= 0.0;
            }
            if is_valid {
                if 1.0 - f_focal_x < 0.0 {
                    t = -t;
                }
                if !is_natively_focal {
                    t = t + fp1;
                }
                if is_swapped != 0u {
                    t = 1.0 - t;
                }
            }
            t_value = t;
        }
    }
    return vec2<f32>(t_value, select(0.0, 1.0, is_valid));
}

fn erf7(x: f32) -> f32 {
    let y = clamp(x * 1.1283791671, -100.0, 100.0);
    let yy = y * y;
    let z = y + (0.24295 + (0.03395 + 0.0104 * yy) * yy) * (y * yy);
    return z / sqrt(1.0 + z * z);
}

fn calculate_blurred_rounded_rect(fragment_pos: vec2<f32>, texel0: vec4<u32>, texel1: vec4<u32>, texel2: vec4<u32>, texel3: vec4<u32>, texel4: vec4<u32>) -> vec4<f32> {
    let transform = get_blurred_rounded_rect_transform(texel0);
    let translate = get_blurred_rounded_rect_translate(texel1);
    let color = get_blurred_rounded_rect_color(texel1);
    let invert = get_blurred_rounded_rect_invert(texel1);
    let exponent = get_blurred_rounded_rect_exponent(texel2);
    let recip_exponent = get_blurred_rounded_rect_recip_exponent(texel2);
    let scale = get_blurred_rounded_rect_scale(texel2);
    let std_dev_inv = get_blurred_rounded_rect_std_dev_inv(texel2);
    let min_edge = get_blurred_rounded_rect_min_edge(texel3);
    let w = get_blurred_rounded_rect_w(texel3);
    let h = get_blurred_rounded_rect_h(texel3);
    let r1 = get_blurred_rounded_rect_r1(texel3);
    let width = get_blurred_rounded_rect_width(texel4);
    let height = get_blurred_rounded_rect_height(texel4);
    let local_xy = transform * fragment_pos + translate;
    let y = local_xy.y - 0.5 * height;
    let y0 = r1 + abs(y) - 0.5 * h;
    let y1 = max(y0, 0.0);
    let x = local_xy.x - 0.5 * width;
    let x0 = r1 + abs(x) - 0.5 * w;
    let x1 = max(x0, 0.0);
    let d_pos = pow(pow(x1, exponent) + pow(y1, exponent), recip_exponent);
    let d_neg = min(max(x0, y0), 0.0);
    let d = d_pos + d_neg - r1;
    let blur_coverage = scale * (erf7(std_dev_inv * (min_edge + d)) - erf7(std_dev_inv * d));
    let blur_alpha = select(blur_coverage, 1.0 - blur_coverage, invert != 0u);
    return color * blur_alpha;
}
"###;
    /// All linked WGSL shader sources, keyed by WESL root module name.
    pub const ALL: &[(&str, &str)] = &[
        ("blend", BLEND),
        ("clear", CLEAR),
        ("copy", COPY),
        ("filter", FILTER),
        ("render", RENDER),
    ];
}
/// Build-time GLSL shaders derived from linked WESL modules.
#[cfg(feature = "glsl")]
/// Compiled GLSL for `blend.wesl`.
pub mod blend {
    #![allow(
        missing_docs,
        reason = "No metadata to generate precise documentation for generated code."
    )]

    pub const VERTEX_SOURCE: &str = r###"#version 300 es

precision highp float;
precision highp int;

struct BlendInstance {
    uint geometry_origin;
    uint geometry_size;
    uint geometry_alpha_col_idx;
    uint parent_texture_size;
    uint child_texture_origin;
    uint child_parent_origin;
    uint child_rect_size;
    uint blend_config;
};
struct VertexOutput {
    vec2 parent_texture_xy;
    vec2 child_texture_xy;
    vec2 child_local;
    uint child_rect_size;
    uint blend_config;
    vec2 tex_coord;
    vec4 position;
};
const uint COMPOSE_CLEAR = 0u;
const uint COMPOSE_COPY = 1u;
const uint COMPOSE_DEST = 2u;
const uint COMPOSE_SRC_OVER = 3u;
const uint COMPOSE_DEST_OVER = 4u;
const uint COMPOSE_SRC_IN = 5u;
const uint COMPOSE_DEST_IN = 6u;
const uint COMPOSE_SRC_OUT = 7u;
const uint COMPOSE_DEST_OUT = 8u;
const uint COMPOSE_SRC_ATOP = 9u;
const uint COMPOSE_DEST_ATOP = 10u;
const uint COMPOSE_XOR = 11u;
const uint COMPOSE_PLUS = 12u;
const uint COMPOSE_PLUS_LIGHTER = 13u;
const uint MIX_NORMAL = 0u;
const uint MIX_MULTIPLY = 1u;
const uint MIX_SCREEN = 2u;
const uint MIX_OVERLAY = 3u;
const uint MIX_DARKEN = 4u;
const uint MIX_LIGHTEN = 5u;
const uint MIX_COLOR_DODGE = 6u;
const uint MIX_COLOR_BURN = 7u;
const uint MIX_HARD_LIGHT = 8u;
const uint MIX_SOFT_LIGHT = 9u;
const uint MIX_DIFFERENCE = 10u;
const uint MIX_EXCLUSION = 11u;
const uint MIX_HUE = 12u;
const uint MIX_SATURATION = 13u;
const uint MIX_COLOR = 14u;
const uint MIX_LUMINOSITY = 15u;
const uint BLEND_HAS_ALPHA = 67108864u;

layout(location = 0) in uint _p2vs_location0;
layout(location = 1) in uint _p2vs_location1;
layout(location = 2) in uint _p2vs_location2;
layout(location = 3) in uint _p2vs_location3;
layout(location = 4) in uint _p2vs_location4;
layout(location = 5) in uint _p2vs_location5;
layout(location = 6) in uint _p2vs_location6;
layout(location = 7) in uint _p2vs_location7;
smooth out vec2 _vs2fs_location0;
smooth out vec2 _vs2fs_location1;
smooth out vec2 _vs2fs_location2;
flat out uint _vs2fs_location3;
flat out uint _vs2fs_location4;
smooth out vec2 _vs2fs_location5;

uvec2 unpack_u16_pair(uint value) {
    return uvec2((value & 65535u), (value >> 16u));
}

vec3 screen(vec3 cb, vec3 cs) {
    return ((cb + cs) - (cb * cs));
}

vec3 hard_light(vec3 cb_1, vec3 cs_1) {
    vec3 _e7 = screen(cb_1, ((2.0 * cs_1) - vec3(1.0)));
    return mix(_e7, ((cb_1 * 2.0) * cs_1), lessThanEqual(cs_1, vec3(0.5)));
}

float color_dodge(float cb_2, float cs_2) {
    if ((cb_2 == 0.0)) {
        return 0.0;
    } else {
        if ((cs_2 == 1.0)) {
            return 1.0;
        } else {
            return min(1.0, (cb_2 / (1.0 - cs_2)));
        }
    }
}

float color_burn(float cb_3, float cs_3) {
    if ((cb_3 == 1.0)) {
        return 1.0;
    } else {
        if ((cs_3 == 0.0)) {
            return 0.0;
        } else {
            return (1.0 - min(1.0, ((1.0 - cb_3) / cs_3)));
        }
    }
}

vec3 soft_light(vec3 cb_4, vec3 cs_4) {
    vec3 d = mix(sqrt(cb_4), (((((16.0 * cb_4) - vec3(12.0)) * cb_4) + vec3(4.0)) * cb_4), lessThanEqual(cb_4, vec3(0.25)));
    return mix((cb_4 + (((2.0 * cs_4) - vec3(1.0)) * (d - cb_4))), (cb_4 - (((vec3(1.0) - (2.0 * cs_4)) * cb_4) * (vec3(1.0) - cb_4))), lessThanEqual(cs_4, vec3(0.5)));
}

float lum(vec3 c) {
    vec3 f = vec3(0.3, 0.59, 0.11);
    return dot(c, f);
}

vec3 clip_color(vec3 c_in) {
    vec3 c_1 = vec3(0.0);
    c_1 = c_in;
    vec3 _e2 = c_1;
    float _e3 = lum(_e2);
    float _e5 = c_1.x;
    float _e7 = c_1.y;
    float _e9 = c_1.z;
    float n = min(_e5, min(_e7, _e9));
    float _e13 = c_1.x;
    float _e15 = c_1.y;
    float _e17 = c_1.z;
    float x = max(_e13, max(_e15, _e17));
    if ((n < 0.0)) {
        vec3 _e22 = c_1;
        c_1 = (vec3(_e3) + (((_e22 - vec3(_e3)) * _e3) / vec3((_e3 - n))));
    }
    if ((x > 1.0)) {
        vec3 _e33 = c_1;
        c_1 = (vec3(_e3) + (((_e33 - vec3(_e3)) * (1.0 - _e3)) / vec3((x - _e3))));
    }
    vec3 _e44 = c_1;
    return _e44;
}

vec3 set_lum(vec3 c_2, float l) {
    float _e2 = lum(c_2);
    vec3 _e6 = clip_color((c_2 + vec3((l - _e2))));
    return _e6;
}

void set_sat_inner(inout float cmin, inout float cmid, inout float cmax, float s) {
    float _e4 = cmax;
    float _e5 = cmin;
    if ((_e4 > _e5)) {
        float _e7 = cmid;
        float _e8 = cmin;
        float _e11 = cmax;
        float _e12 = cmin;
        cmid = (((_e7 - _e8) * s) / (_e11 - _e12));
        cmax = s;
    } else {
        cmid = 0.0;
        cmax = 0.0;
    }
    cmin = 0.0;
    return;
}

vec3 set_sat(vec3 c_3, float s_1) {
    float r = 0.0;
    float g = 0.0;
    float b = 0.0;
    r = c_3.x;
    g = c_3.y;
    b = c_3.z;
    float _e8 = r;
    float _e9 = g;
    if ((_e8 <= _e9)) {
        float _e11 = g;
        float _e12 = b;
        if ((_e11 <= _e12)) {
            set_sat_inner(r, g, b, s_1);
        } else {
            float _e14 = r;
            float _e15 = b;
            if ((_e14 <= _e15)) {
                set_sat_inner(r, b, g, s_1);
            } else {
                set_sat_inner(b, r, g, s_1);
            }
        }
    } else {
        float _e17 = r;
        float _e18 = b;
        if ((_e17 <= _e18)) {
            set_sat_inner(g, r, b, s_1);
        } else {
            float _e20 = g;
            float _e21 = b;
            if ((_e20 <= _e21)) {
                set_sat_inner(g, b, r, s_1);
            } else {
                set_sat_inner(b, g, r, s_1);
            }
        }
    }
    float _e23 = r;
    float _e24 = g;
    float _e25 = b;
    return vec3(_e23, _e24, _e25);
}

float sat(vec3 c_4) {
    return (max(c_4.x, max(c_4.y, c_4.z)) - min(c_4.x, min(c_4.y, c_4.z)));
}

vec3 blend_mix(vec3 cb_5, vec3 cs_5, uint mode) {
    vec3 b_1 = vec3(0.0);
    switch(mode) {
        case 1u: {
            b_1 = (cb_5 * cs_5);
            break;
        }
        case 2u: {
            vec3 _e7 = screen(cb_5, cs_5);
            b_1 = _e7;
            break;
        }
        case 3u: {
            vec3 _e8 = hard_light(cs_5, cb_5);
            b_1 = _e8;
            break;
        }
        case 4u: {
            b_1 = min(cb_5, cs_5);
            break;
        }
        case 5u: {
            b_1 = max(cb_5, cs_5);
            break;
        }
        case 6u: {
            float _e13 = color_dodge(cb_5.x, cs_5.x);
            float _e16 = color_dodge(cb_5.y, cs_5.y);
            float _e19 = color_dodge(cb_5.z, cs_5.z);
            b_1 = vec3(_e13, _e16, _e19);
            break;
        }
        case 7u: {
            float _e23 = color_burn(cb_5.x, cs_5.x);
            float _e26 = color_burn(cb_5.y, cs_5.y);
            float _e29 = color_burn(cb_5.z, cs_5.z);
            b_1 = vec3(_e23, _e26, _e29);
            break;
        }
        case 8u: {
            vec3 _e31 = hard_light(cb_5, cs_5);
            b_1 = _e31;
            break;
        }
        case 9u: {
            vec3 _e32 = soft_light(cb_5, cs_5);
            b_1 = _e32;
            break;
        }
        case 10u: {
            b_1 = abs((cb_5 - cs_5));
            break;
        }
        case 11u: {
            b_1 = ((cb_5 + cs_5) - ((2.0 * cb_5) * cs_5));
            break;
        }
        case 12u: {
            float _e40 = sat(cb_5);
            vec3 _e41 = set_sat(cs_5, _e40);
            float _e42 = lum(cb_5);
            vec3 _e43 = set_lum(_e41, _e42);
            b_1 = _e43;
            break;
        }
        case 13u: {
            float _e44 = sat(cs_5);
            vec3 _e45 = set_sat(cb_5, _e44);
            float _e46 = lum(cb_5);
            vec3 _e47 = set_lum(_e45, _e46);
            b_1 = _e47;
            break;
        }
        case 14u: {
            float _e48 = lum(cb_5);
            vec3 _e49 = set_lum(cs_5, _e48);
            b_1 = _e49;
            break;
        }
        case 15u: {
            float _e50 = lum(cs_5);
            vec3 _e51 = set_lum(cb_5, _e50);
            b_1 = _e51;
            break;
        }
        default: {
            b_1 = cs_5;
            break;
        }
    }
    vec3 _e52 = b_1;
    return _e52;
}

vec4 blend_compose_unpremul(vec3 cb_6, vec3 cs_6, float ab, float as, uint mode_1) {
    float fa = 0.0;
    float fb = 0.0;
    switch(mode_1) {
        case 1u: {
            fa = 1.0;
            fb = 0.0;
            break;
        }
        case 2u: {
            fa = 0.0;
            fb = 1.0;
            break;
        }
        case 3u: {
            fa = 1.0;
            fb = (1.0 - as);
            break;
        }
        case 4u: {
            fa = (1.0 - ab);
            fb = 1.0;
            break;
        }
        case 5u: {
            fa = ab;
            fb = 0.0;
            break;
        }
        case 6u: {
            fa = 0.0;
            fb = as;
            break;
        }
        case 7u: {
            fa = (1.0 - ab);
            fb = 0.0;
            break;
        }
        case 8u: {
            fa = 0.0;
            fb = (1.0 - as);
            break;
        }
        case 9u: {
            fa = ab;
            fb = (1.0 - as);
            break;
        }
        case 10u: {
            fa = (1.0 - ab);
            fb = as;
            break;
        }
        case 11u: {
            fa = (1.0 - ab);
            fb = (1.0 - as);
            break;
        }
        case 12u: {
            fa = 1.0;
            fb = 1.0;
            break;
        }
        case 13u: {
            return min(vec4(1.0), vec4(((as * cs_6) + (ab * cb_6)), (as + ab)));
        }
        default: {
            break;
        }
    }
    float _e45 = fa;
    float as_fa = (as * _e45);
    float _e47 = fb;
    float ab_fb = (ab * _e47);
    vec3 co = ((as_fa * cs_6) + (ab_fb * cb_6));
    return vec4(co, min((as_fa + ab_fb), 1.0));
}

vec4 blend_mix_compose(vec4 backdrop, vec4 src, uint compose_mode, uint mix_mode) {
    vec3 cs_7 = vec3(0.0);
    uint mode_2 = ((mix_mode << 8u) | compose_mode);
    if ((mode_2 == 3u)) {
        return ((backdrop * (1.0 - src.w)) + src);
    }
    float inv_src_a = (1.0 / max(src.w, 1e-15));
    cs_7 = (src.xyz * inv_src_a);
    float inv_backdrop_a = (1.0 / max(backdrop.w, 1e-15));
    vec3 cb_7 = (backdrop.xyz * inv_backdrop_a);
    vec3 _e28 = cs_7;
    vec3 _e29 = blend_mix(cb_7, _e28, mix_mode);
    vec3 _e30 = cs_7;
    cs_7 = mix(_e30, _e29, backdrop.w);
    if ((compose_mode == COMPOSE_SRC_OVER)) {
        vec3 _e36 = cs_7;
        vec3 co_1 = mix(backdrop.xyz, _e36, src.w);
        return vec4(co_1, (src.w + (backdrop.w * (1.0 - src.w))));
    } else {
        vec3 _e47 = cs_7;
        vec4 _e50 = blend_compose_unpremul(cb_7, _e47, backdrop.w, src.w, compose_mode);
        return _e50;
    }
}

void main() {
    uint vertex_index = uint(gl_VertexID);
    BlendInstance instance = BlendInstance(_p2vs_location0, _p2vs_location1, _p2vs_location2, _p2vs_location3, _p2vs_location4, _p2vs_location5, _p2vs_location6, _p2vs_location7);
    VertexOutput out_ = VertexOutput(vec2(0.0), vec2(0.0), vec2(0.0), 0u, 0u, vec2(0.0), vec4(0.0));
    uvec2 _e3 = unpack_u16_pair(instance.geometry_origin);
    uvec2 _e5 = unpack_u16_pair(instance.geometry_size);
    vec2 geometry_size = vec2(_e5);
    uvec2 _e8 = unpack_u16_pair(instance.parent_texture_size);
    uvec2 _e10 = unpack_u16_pair(instance.child_parent_origin);
    uvec2 _e12 = unpack_u16_pair(instance.child_texture_origin);
    float x_1 = float((vertex_index & 1u));
    float y = float((vertex_index >> 1u));
    vec2 parent_texture_xy_1 = (vec2(_e3) + (vec2(x_1, y) * geometry_size));
    vec2 child_local_1 = (parent_texture_xy_1 - vec2(_e10));
    out_.parent_texture_xy = parent_texture_xy_1;
    out_.child_texture_xy = (vec2(_e12) + child_local_1);
    out_.child_local = child_local_1;
    out_.child_rect_size = instance.child_rect_size;
    out_.blend_config = instance.blend_config;
    out_.tex_coord = vec2((float(instance.geometry_alpha_col_idx) + (x_1 * geometry_size.x)), (y * geometry_size.y));
    float ndc_x = (((parent_texture_xy_1.x * 2.0) / float(_e8.x)) - 1.0);
    float ndc_y = (1.0 - ((parent_texture_xy_1.y * 2.0) / float(_e8.y)));
    out_.position = vec4(ndc_x, ndc_y, 0.0, 1.0);
    VertexOutput _e64 = out_;
    _vs2fs_location0 = _e64.parent_texture_xy;
    _vs2fs_location1 = _e64.child_texture_xy;
    _vs2fs_location2 = _e64.child_local;
    _vs2fs_location3 = _e64.child_rect_size;
    _vs2fs_location4 = _e64.blend_config;
    _vs2fs_location5 = _e64.tex_coord;
    gl_Position = _e64.position;
    gl_Position.yz = vec2(-gl_Position.y, gl_Position.z * 2.0 - gl_Position.w);
    return;
}

"###;

    pub const FRAGMENT_SOURCE: &str = r###"#version 300 es

precision highp float;
precision highp int;

struct BlendInstance {
    uint geometry_origin;
    uint geometry_size;
    uint geometry_alpha_col_idx;
    uint parent_texture_size;
    uint child_texture_origin;
    uint child_parent_origin;
    uint child_rect_size;
    uint blend_config;
};
struct VertexOutput {
    vec2 parent_texture_xy;
    vec2 child_texture_xy;
    vec2 child_local;
    uint child_rect_size;
    uint blend_config;
    vec2 tex_coord;
    vec4 position;
};
const uint COMPOSE_CLEAR = 0u;
const uint COMPOSE_COPY = 1u;
const uint COMPOSE_DEST = 2u;
const uint COMPOSE_SRC_OVER = 3u;
const uint COMPOSE_DEST_OVER = 4u;
const uint COMPOSE_SRC_IN = 5u;
const uint COMPOSE_DEST_IN = 6u;
const uint COMPOSE_SRC_OUT = 7u;
const uint COMPOSE_DEST_OUT = 8u;
const uint COMPOSE_SRC_ATOP = 9u;
const uint COMPOSE_DEST_ATOP = 10u;
const uint COMPOSE_XOR = 11u;
const uint COMPOSE_PLUS = 12u;
const uint COMPOSE_PLUS_LIGHTER = 13u;
const uint MIX_NORMAL = 0u;
const uint MIX_MULTIPLY = 1u;
const uint MIX_SCREEN = 2u;
const uint MIX_OVERLAY = 3u;
const uint MIX_DARKEN = 4u;
const uint MIX_LIGHTEN = 5u;
const uint MIX_COLOR_DODGE = 6u;
const uint MIX_COLOR_BURN = 7u;
const uint MIX_HARD_LIGHT = 8u;
const uint MIX_SOFT_LIGHT = 9u;
const uint MIX_DIFFERENCE = 10u;
const uint MIX_EXCLUSION = 11u;
const uint MIX_HUE = 12u;
const uint MIX_SATURATION = 13u;
const uint MIX_COLOR = 14u;
const uint MIX_LUMINOSITY = 15u;
const uint BLEND_HAS_ALPHA = 67108864u;

uniform highp sampler2D _group_0_binding_0_fs;

uniform highp sampler2D _group_0_binding_1_fs;

uniform highp usampler2D _group_0_binding_2_fs;

smooth in vec2 _vs2fs_location0;
smooth in vec2 _vs2fs_location1;
smooth in vec2 _vs2fs_location2;
flat in uint _vs2fs_location3;
flat in uint _vs2fs_location4;
smooth in vec2 _vs2fs_location5;
layout(location = 0) out vec4 _fs2p_location0;

uvec2 unpack_u16_pair(uint value) {
    return uvec2((value & 65535u), (value >> 16u));
}

vec4 load_layer(uint texture_index, ivec2 xy) {
    if ((texture_index == 0u)) {
        vec4 _e6 = texelFetch(_group_0_binding_0_fs, xy, 0);
        return _e6;
    } else {
        vec4 _e9 = texelFetch(_group_0_binding_1_fs, xy, 0);
        return _e9;
    }
}

vec3 screen(vec3 cb, vec3 cs) {
    return ((cb + cs) - (cb * cs));
}

vec3 hard_light(vec3 cb_1, vec3 cs_1) {
    vec3 _e7 = screen(cb_1, ((2.0 * cs_1) - vec3(1.0)));
    return mix(_e7, ((cb_1 * 2.0) * cs_1), lessThanEqual(cs_1, vec3(0.5)));
}

float color_dodge(float cb_2, float cs_2) {
    if ((cb_2 == 0.0)) {
        return 0.0;
    } else {
        if ((cs_2 == 1.0)) {
            return 1.0;
        } else {
            return min(1.0, (cb_2 / (1.0 - cs_2)));
        }
    }
}

float color_burn(float cb_3, float cs_3) {
    if ((cb_3 == 1.0)) {
        return 1.0;
    } else {
        if ((cs_3 == 0.0)) {
            return 0.0;
        } else {
            return (1.0 - min(1.0, ((1.0 - cb_3) / cs_3)));
        }
    }
}

vec3 soft_light(vec3 cb_4, vec3 cs_4) {
    vec3 d = mix(sqrt(cb_4), (((((16.0 * cb_4) - vec3(12.0)) * cb_4) + vec3(4.0)) * cb_4), lessThanEqual(cb_4, vec3(0.25)));
    return mix((cb_4 + (((2.0 * cs_4) - vec3(1.0)) * (d - cb_4))), (cb_4 - (((vec3(1.0) - (2.0 * cs_4)) * cb_4) * (vec3(1.0) - cb_4))), lessThanEqual(cs_4, vec3(0.5)));
}

float lum(vec3 c) {
    vec3 f = vec3(0.3, 0.59, 0.11);
    return dot(c, f);
}

vec3 clip_color(vec3 c_in) {
    vec3 c_1 = vec3(0.0);
    c_1 = c_in;
    vec3 _e2 = c_1;
    float _e3 = lum(_e2);
    float _e5 = c_1.x;
    float _e7 = c_1.y;
    float _e9 = c_1.z;
    float n = min(_e5, min(_e7, _e9));
    float _e13 = c_1.x;
    float _e15 = c_1.y;
    float _e17 = c_1.z;
    float x = max(_e13, max(_e15, _e17));
    if ((n < 0.0)) {
        vec3 _e22 = c_1;
        c_1 = (vec3(_e3) + (((_e22 - vec3(_e3)) * _e3) / vec3((_e3 - n))));
    }
    if ((x > 1.0)) {
        vec3 _e33 = c_1;
        c_1 = (vec3(_e3) + (((_e33 - vec3(_e3)) * (1.0 - _e3)) / vec3((x - _e3))));
    }
    vec3 _e44 = c_1;
    return _e44;
}

vec3 set_lum(vec3 c_2, float l) {
    float _e2 = lum(c_2);
    vec3 _e6 = clip_color((c_2 + vec3((l - _e2))));
    return _e6;
}

void set_sat_inner(inout float cmin, inout float cmid, inout float cmax, float s) {
    float _e4 = cmax;
    float _e5 = cmin;
    if ((_e4 > _e5)) {
        float _e7 = cmid;
        float _e8 = cmin;
        float _e11 = cmax;
        float _e12 = cmin;
        cmid = (((_e7 - _e8) * s) / (_e11 - _e12));
        cmax = s;
    } else {
        cmid = 0.0;
        cmax = 0.0;
    }
    cmin = 0.0;
    return;
}

vec3 set_sat(vec3 c_3, float s_1) {
    float r = 0.0;
    float g = 0.0;
    float b = 0.0;
    r = c_3.x;
    g = c_3.y;
    b = c_3.z;
    float _e8 = r;
    float _e9 = g;
    if ((_e8 <= _e9)) {
        float _e11 = g;
        float _e12 = b;
        if ((_e11 <= _e12)) {
            set_sat_inner(r, g, b, s_1);
        } else {
            float _e14 = r;
            float _e15 = b;
            if ((_e14 <= _e15)) {
                set_sat_inner(r, b, g, s_1);
            } else {
                set_sat_inner(b, r, g, s_1);
            }
        }
    } else {
        float _e17 = r;
        float _e18 = b;
        if ((_e17 <= _e18)) {
            set_sat_inner(g, r, b, s_1);
        } else {
            float _e20 = g;
            float _e21 = b;
            if ((_e20 <= _e21)) {
                set_sat_inner(g, b, r, s_1);
            } else {
                set_sat_inner(b, g, r, s_1);
            }
        }
    }
    float _e23 = r;
    float _e24 = g;
    float _e25 = b;
    return vec3(_e23, _e24, _e25);
}

float sat(vec3 c_4) {
    return (max(c_4.x, max(c_4.y, c_4.z)) - min(c_4.x, min(c_4.y, c_4.z)));
}

vec3 blend_mix(vec3 cb_5, vec3 cs_5, uint mode) {
    vec3 b_1 = vec3(0.0);
    switch(mode) {
        case 1u: {
            b_1 = (cb_5 * cs_5);
            break;
        }
        case 2u: {
            vec3 _e7 = screen(cb_5, cs_5);
            b_1 = _e7;
            break;
        }
        case 3u: {
            vec3 _e8 = hard_light(cs_5, cb_5);
            b_1 = _e8;
            break;
        }
        case 4u: {
            b_1 = min(cb_5, cs_5);
            break;
        }
        case 5u: {
            b_1 = max(cb_5, cs_5);
            break;
        }
        case 6u: {
            float _e13 = color_dodge(cb_5.x, cs_5.x);
            float _e16 = color_dodge(cb_5.y, cs_5.y);
            float _e19 = color_dodge(cb_5.z, cs_5.z);
            b_1 = vec3(_e13, _e16, _e19);
            break;
        }
        case 7u: {
            float _e23 = color_burn(cb_5.x, cs_5.x);
            float _e26 = color_burn(cb_5.y, cs_5.y);
            float _e29 = color_burn(cb_5.z, cs_5.z);
            b_1 = vec3(_e23, _e26, _e29);
            break;
        }
        case 8u: {
            vec3 _e31 = hard_light(cb_5, cs_5);
            b_1 = _e31;
            break;
        }
        case 9u: {
            vec3 _e32 = soft_light(cb_5, cs_5);
            b_1 = _e32;
            break;
        }
        case 10u: {
            b_1 = abs((cb_5 - cs_5));
            break;
        }
        case 11u: {
            b_1 = ((cb_5 + cs_5) - ((2.0 * cb_5) * cs_5));
            break;
        }
        case 12u: {
            float _e40 = sat(cb_5);
            vec3 _e41 = set_sat(cs_5, _e40);
            float _e42 = lum(cb_5);
            vec3 _e43 = set_lum(_e41, _e42);
            b_1 = _e43;
            break;
        }
        case 13u: {
            float _e44 = sat(cs_5);
            vec3 _e45 = set_sat(cb_5, _e44);
            float _e46 = lum(cb_5);
            vec3 _e47 = set_lum(_e45, _e46);
            b_1 = _e47;
            break;
        }
        case 14u: {
            float _e48 = lum(cb_5);
            vec3 _e49 = set_lum(cs_5, _e48);
            b_1 = _e49;
            break;
        }
        case 15u: {
            float _e50 = lum(cs_5);
            vec3 _e51 = set_lum(cb_5, _e50);
            b_1 = _e51;
            break;
        }
        default: {
            b_1 = cs_5;
            break;
        }
    }
    vec3 _e52 = b_1;
    return _e52;
}

vec4 blend_compose_unpremul(vec3 cb_6, vec3 cs_6, float ab, float as, uint mode_1) {
    float fa = 0.0;
    float fb = 0.0;
    switch(mode_1) {
        case 1u: {
            fa = 1.0;
            fb = 0.0;
            break;
        }
        case 2u: {
            fa = 0.0;
            fb = 1.0;
            break;
        }
        case 3u: {
            fa = 1.0;
            fb = (1.0 - as);
            break;
        }
        case 4u: {
            fa = (1.0 - ab);
            fb = 1.0;
            break;
        }
        case 5u: {
            fa = ab;
            fb = 0.0;
            break;
        }
        case 6u: {
            fa = 0.0;
            fb = as;
            break;
        }
        case 7u: {
            fa = (1.0 - ab);
            fb = 0.0;
            break;
        }
        case 8u: {
            fa = 0.0;
            fb = (1.0 - as);
            break;
        }
        case 9u: {
            fa = ab;
            fb = (1.0 - as);
            break;
        }
        case 10u: {
            fa = (1.0 - ab);
            fb = as;
            break;
        }
        case 11u: {
            fa = (1.0 - ab);
            fb = (1.0 - as);
            break;
        }
        case 12u: {
            fa = 1.0;
            fb = 1.0;
            break;
        }
        case 13u: {
            return min(vec4(1.0), vec4(((as * cs_6) + (ab * cb_6)), (as + ab)));
        }
        default: {
            break;
        }
    }
    float _e45 = fa;
    float as_fa = (as * _e45);
    float _e47 = fb;
    float ab_fb = (ab * _e47);
    vec3 co = ((as_fa * cs_6) + (ab_fb * cb_6));
    return vec4(co, min((as_fa + ab_fb), 1.0));
}

vec4 blend_mix_compose(vec4 backdrop, vec4 src, uint compose_mode, uint mix_mode) {
    vec3 cs_7 = vec3(0.0);
    uint mode_2 = ((mix_mode << 8u) | compose_mode);
    if ((mode_2 == 3u)) {
        return ((backdrop * (1.0 - src.w)) + src);
    }
    float inv_src_a = (1.0 / max(src.w, 1e-15));
    cs_7 = (src.xyz * inv_src_a);
    float inv_backdrop_a = (1.0 / max(backdrop.w, 1e-15));
    vec3 cb_7 = (backdrop.xyz * inv_backdrop_a);
    vec3 _e28 = cs_7;
    vec3 _e29 = blend_mix(cb_7, _e28, mix_mode);
    vec3 _e30 = cs_7;
    cs_7 = mix(_e30, _e29, backdrop.w);
    if ((compose_mode == COMPOSE_SRC_OVER)) {
        vec3 _e36 = cs_7;
        vec3 co_1 = mix(backdrop.xyz, _e36, src.w);
        return vec4(co_1, (src.w + (backdrop.w * (1.0 - src.w))));
    } else {
        vec3 _e47 = cs_7;
        vec4 _e50 = blend_compose_unpremul(cb_7, _e47, backdrop.w, src.w, compose_mode);
        return _e50;
    }
}

void main() {
    vec2 parent_texture_xy = _vs2fs_location0;
    vec2 child_texture_xy = _vs2fs_location1;
    vec2 child_local = _vs2fs_location2;
    uint packed_child_rect_size = _vs2fs_location3;
    uint blend_config = _vs2fs_location4;
    vec2 tex_coord = _vs2fs_location5;
    float clip_coverage = 1.0;
    uint column = 0u;
    vec4 child = vec4(0.0);
    bool local = false;
    bool local_1 = false;
    bool local_2 = false;
    uvec2 _e6 = unpack_u16_pair(packed_child_rect_size);
    uint compose = (blend_config & 255u);
    uint mix_mode_1 = ((blend_config >> 8u) & 255u);
    uint opacity = ((blend_config >> 16u) & 255u);
    uint parent_texture_index = ((blend_config >> 24u) & 1u);
    uint child_texture_index = ((blend_config >> 25u) & 1u);
    uint has_alpha = ((blend_config >> 26u) & 1u);
    if ((has_alpha != 0u)) {
        uint alpha_index = uint(floor(tex_coord.x));
        uint y = uint(floor(tex_coord.y));
        uint texel_index = (alpha_index / 4u);
        uint channel_index = (alpha_index % 4u);
        uint texture_width = uvec2(textureSize(_group_0_binding_2_fs, 0).xy).x;
        uvec4 packed_ = texelFetch(_group_0_binding_2_fs, ivec2(uvec2((texel_index % texture_width), (texel_index / texture_width))), 0);
        column = packed_.x;
        switch(channel_index) {
            case 1u: {
                column = packed_.y;
                break;
            }
            case 2u: {
                column = packed_.z;
                break;
            }
            case 3u: {
                column = packed_.w;
                break;
            }
            default: {
                break;
            }
        }
        uint _e57 = column;
        clip_coverage = (float(((_e57 >> (y * 8u)) & 255u)) * 0.003921569);
    }
    vec4 _e67 = load_layer(parent_texture_index, ivec2(parent_texture_xy));
    float child_opacity = (float(opacity) * 0.003921569);
    if ((child_local.x >= 0.0)) {
        local = (child_local.y >= 0.0);
    } else {
        local = false;
    }
    bool _e83 = local;
    if (_e83) {
        local_1 = (child_local.x < float(_e6.x));
    } else {
        local_1 = false;
    }
    bool _e91 = local_1;
    if (_e91) {
        local_2 = (child_local.y < float(_e6.y));
    } else {
        local_2 = false;
    }
    bool _e99 = local_2;
    if (_e99) {
        vec4 _e101 = load_layer(child_texture_index, ivec2(child_texture_xy));
        child = (child_opacity * _e101);
    }
    vec4 _e103 = child;
    vec4 _e104 = blend_mix_compose(_e67, _e103, compose, mix_mode_1);
    float _e105 = clip_coverage;
    _fs2p_location0 = mix(_e67, _e104, _e105);
    return;
}

"###;
    pub mod fragment {
        pub const ALPHAS_TEXTURE: &str = "_group_0_binding_2_fs";
        pub const LAYER_TEXTURE_0: &str = "_group_0_binding_0_fs";
        pub const LAYER_TEXTURE_1: &str = "_group_0_binding_1_fs";
    }
}
#[cfg(feature = "glsl")]
/// Compiled GLSL for `clear.wesl`.
pub mod clear {
    #![allow(
        missing_docs,
        reason = "No metadata to generate precise documentation for generated code."
    )]

    pub const VERTEX_SOURCE: &str = r###"#version 300 es

precision highp float;
precision highp int;

layout(location = 0) in uvec2 _p2vs_location0;
layout(location = 1) in uvec2 _p2vs_location1;
layout(location = 2) in uvec2 _p2vs_location2;

void main() {
    uint vertex_index = uint(gl_VertexID);
    uvec2 origin = _p2vs_location0;
    uvec2 size = _p2vs_location1;
    uvec2 target_size = _p2vs_location2;
    float x = float((vertex_index & 1u));
    float y = float((vertex_index >> 1u));
    float pix_x = (float(origin.x) + (x * float(size.x)));
    float pix_y = (float(origin.y) + (y * float(size.y)));
    float ndc_x = (((pix_x * 2.0) / float(target_size.x)) - 1.0);
    float ndc_y = (1.0 - ((pix_y * 2.0) / float(target_size.y)));
    gl_Position = vec4(ndc_x, ndc_y, 0.0, 1.0);
    gl_Position.yz = vec2(-gl_Position.y, gl_Position.z * 2.0 - gl_Position.w);
    return;
}

"###;

    pub const FRAGMENT_SOURCE: &str = r###"#version 300 es

precision highp float;
precision highp int;

layout(location = 0) out vec4 _fs2p_location0;

void main() {
    vec4 position = gl_FragCoord;
    _fs2p_location0 = vec4(0.0, 0.0, 0.0, 0.0);
    return;
}

"###;
}
#[cfg(feature = "glsl")]
/// Compiled GLSL for `copy.wesl`.
pub mod copy {
    #![allow(
        missing_docs,
        reason = "No metadata to generate precise documentation for generated code."
    )]

    pub const VERTEX_SOURCE: &str = r###"#version 300 es

precision highp float;
precision highp int;

struct CopyInstance {
    uint dest_texture_origin;
    uint source_texture_origin;
    uint copy_rect_size;
    uint dest_texture_size;
};
struct VertexOutput {
    vec2 source_texture_xy;
    vec4 position;
};
layout(location = 0) in uint _p2vs_location0;
layout(location = 1) in uint _p2vs_location1;
layout(location = 2) in uint _p2vs_location2;
layout(location = 3) in uint _p2vs_location3;
smooth out vec2 _vs2fs_location0;

uvec2 unpack_u16_pair(uint value) {
    return uvec2((value & 65535u), (value >> 16u));
}

void main() {
    uint vertex_index = uint(gl_VertexID);
    CopyInstance instance = CopyInstance(_p2vs_location0, _p2vs_location1, _p2vs_location2, _p2vs_location3);
    VertexOutput out_ = VertexOutput(vec2(0.0), vec4(0.0));
    uvec2 _e3 = unpack_u16_pair(instance.dest_texture_origin);
    uvec2 _e5 = unpack_u16_pair(instance.source_texture_origin);
    uvec2 _e7 = unpack_u16_pair(instance.copy_rect_size);
    uvec2 _e9 = unpack_u16_pair(instance.dest_texture_size);
    float x = float((vertex_index & 1u));
    float y = float((vertex_index >> 1u));
    vec2 local = (vec2(x, y) * vec2(_e7));
    vec2 dest_texture_xy = (vec2(_e3) + local);
    out_.source_texture_xy = (vec2(_e5) + local);
    float ndc_x = (((dest_texture_xy.x * 2.0) / float(_e9.x)) - 1.0);
    float ndc_y = (1.0 - ((dest_texture_xy.y * 2.0) / float(_e9.y)));
    out_.position = vec4(ndc_x, ndc_y, 0.0, 1.0);
    VertexOutput _e45 = out_;
    _vs2fs_location0 = _e45.source_texture_xy;
    gl_Position = _e45.position;
    gl_Position.yz = vec2(-gl_Position.y, gl_Position.z * 2.0 - gl_Position.w);
    return;
}

"###;

    pub const FRAGMENT_SOURCE: &str = r###"#version 300 es

precision highp float;
precision highp int;

struct CopyInstance {
    uint dest_texture_origin;
    uint source_texture_origin;
    uint copy_rect_size;
    uint dest_texture_size;
};
struct VertexOutput {
    vec2 source_texture_xy;
    vec4 position;
};
uniform highp sampler2D _group_0_binding_0_fs;

smooth in vec2 _vs2fs_location0;
layout(location = 0) out vec4 _fs2p_location0;

uvec2 unpack_u16_pair(uint value) {
    return uvec2((value & 65535u), (value >> 16u));
}

void main() {
    vec2 source_texture_xy = _vs2fs_location0;
    vec4 _e4 = texelFetch(_group_0_binding_0_fs, ivec2(source_texture_xy), 0);
    _fs2p_location0 = _e4;
    return;
}

"###;
    pub mod fragment {
        pub const SOURCE_TEXTURE: &str = "_group_0_binding_0_fs";
    }
}
#[cfg(feature = "glsl")]
/// Compiled GLSL for `filter.wesl`.
pub mod filter {
    #![allow(
        missing_docs,
        reason = "No metadata to generate precise documentation for generated code."
    )]

    pub const VERTEX_SOURCE: &str = r###"#version 300 es

precision highp float;
precision highp int;

struct FilterInstanceData {
    uint source_origin;
    uint source_size;
    uint dest_origin;
    uint dest_size;
    uint dest_texture_size;
    uint filter_data_offset;
    uint original_origin;
    uint original_size;
    uint filter_pass_kind;
};
struct FilterVertexOutput {
    vec4 position;
    uint filter_data_offset;
    uvec2 source_origin;
    uvec2 source_size;
    uvec2 dest_origin;
    uvec2 dest_size;
    uvec2 original_origin;
    uint filter_pass_kind;
};
const uint FILTER_SIZE_BYTES = 48u;
const uint FILTER_SIZE_U32_ = 12u;
const uint TEXELS_PER_FILTER = 3u;
const uint FILTER_TYPE_OFFSET = 0u;
const uint FILTER_TYPE_FLOOD = 1u;
const uint FILTER_TYPE_GAUSSIAN_BLUR = 2u;
const uint FILTER_TYPE_DROP_SHADOW = 3u;
const uint PASS_COPY = 0u;
const uint PASS_FLOOD = 1u;
const uint PASS_OFFSET = 2u;
const uint PASS_DOWNSCALE = 3u;
const uint PASS_BLUR_H = 4u;
const uint PASS_BLUR_V = 5u;
const uint PASS_UPSCALE = 6u;
const uint PASS_COMPOSITE_DROP_SHADOW = 7u;
const uint PASS_COLORIZE = 8u;
const uint FILTER_ATLAS_PADDING = 6u;
const uint MAX_TAPS_PER_SIDE = 3u;
const vec2 HORIZONTAL = vec2(1.0, 0.0);
const vec2 VERTICAL = vec2(0.0, 1.0);

layout(location = 0) in uint _p2vs_location0;
layout(location = 1) in uint _p2vs_location1;
layout(location = 2) in uint _p2vs_location2;
layout(location = 3) in uint _p2vs_location3;
layout(location = 4) in uint _p2vs_location4;
layout(location = 5) in uint _p2vs_location5;
layout(location = 6) in uint _p2vs_location6;
layout(location = 7) in uint _p2vs_location7;
layout(location = 8) in uint _p2vs_location8;
flat out uint _vs2fs_location0;
flat out uvec2 _vs2fs_location1;
flat out uvec2 _vs2fs_location2;
flat out uvec2 _vs2fs_location3;
flat out uvec2 _vs2fs_location4;
flat out uvec2 _vs2fs_location5;
flat out uint _vs2fs_location6;

uint get_filter_type(uvec4 texel0_) {
    return (texel0_.x & 31u);
}

uint get_filter_header_n_linear_taps(uvec4 texel0_1) {
    return ((texel0_1.x >> 11u) & 3u);
}

float get_offset_dx(uvec4 texel0_2) {
    return uintBitsToFloat(texel0_2.y);
}

float get_offset_dy(uvec4 texel0_3) {
    return uintBitsToFloat(texel0_3.z);
}

uint get_flood_color(uvec4 texel0_4) {
    return texel0_4.y;
}

float get_blur_center_weight(uvec4 texel0_5) {
    return uintBitsToFloat(texel0_5.y);
}

vec3 get_blur_linear_weights(uvec4 texel0_6, uvec4 texel1_) {
    return vec3(uintBitsToFloat(texel0_6.z), uintBitsToFloat(texel0_6.w), uintBitsToFloat(texel1_.x));
}

vec3 get_blur_linear_offsets(uvec4 texel1_1) {
    return vec3(uintBitsToFloat(texel1_1.y), uintBitsToFloat(texel1_1.z), uintBitsToFloat(texel1_1.w));
}

float get_drop_shadow_dx(uvec4 texel2_) {
    return uintBitsToFloat(texel2_.x);
}

float get_drop_shadow_dy(uvec4 texel2_1) {
    return uintBitsToFloat(texel2_1.y);
}

uint get_drop_shadow_color(uvec4 texel2_2) {
    return texel2_2.z;
}

uvec2 unpack_u16_pair(uint value) {
    return uvec2((value & 65535u), (value >> 16u));
}

void main() {
    uint vertex_index = uint(gl_VertexID);
    FilterInstanceData instance = FilterInstanceData(_p2vs_location0, _p2vs_location1, _p2vs_location2, _p2vs_location3, _p2vs_location4, _p2vs_location5, _p2vs_location6, _p2vs_location7, _p2vs_location8);
    FilterVertexOutput out_ = FilterVertexOutput(vec4(0.0), 0u, uvec2(0u), uvec2(0u), uvec2(0u), uvec2(0u), uvec2(0u), 0u);
    uint quad_vertex = (vertex_index % 4u);
    float x = float((quad_vertex & 1u));
    float y = float((quad_vertex >> 1u));
    uvec2 _e11 = unpack_u16_pair(instance.source_origin);
    uvec2 _e13 = unpack_u16_pair(instance.source_size);
    uvec2 _e15 = unpack_u16_pair(instance.dest_origin);
    uvec2 _e17 = unpack_u16_pair(instance.dest_size);
    uvec2 _e19 = unpack_u16_pair(instance.dest_texture_size);
    vec2 dest_texture_size = vec2(_e19);
    uvec2 _e22 = unpack_u16_pair(instance.original_origin);
    uvec2 _e24 = unpack_u16_pair(instance.original_size);
    uvec2 render_size = min(_e24, (_e17 + uvec2(6u)));
    float pix_x = (float(_e15.x) + (x * float(render_size.x)));
    float pix_y = (float(_e15.y) + (y * float(render_size.y)));
    float ndc_x = (((pix_x * 2.0) / dest_texture_size.x) - 1.0);
    float ndc_y = (1.0 - ((pix_y * 2.0) / dest_texture_size.y));
    out_.position = vec4(ndc_x, ndc_y, 0.0, 1.0);
    out_.filter_data_offset = instance.filter_data_offset;
    out_.source_origin = _e11;
    out_.source_size = _e13;
    out_.dest_origin = _e15;
    out_.dest_size = _e17;
    out_.original_origin = _e22;
    out_.filter_pass_kind = instance.filter_pass_kind;
    FilterVertexOutput _e67 = out_;
    gl_Position = _e67.position;
    _vs2fs_location0 = _e67.filter_data_offset;
    _vs2fs_location1 = _e67.source_origin;
    _vs2fs_location2 = _e67.source_size;
    _vs2fs_location3 = _e67.dest_origin;
    _vs2fs_location4 = _e67.dest_size;
    _vs2fs_location5 = _e67.original_origin;
    _vs2fs_location6 = _e67.filter_pass_kind;
    gl_Position.yz = vec2(-gl_Position.y, gl_Position.z * 2.0 - gl_Position.w);
    return;
}

"###;

    pub const FRAGMENT_SOURCE: &str = r###"#version 300 es

precision highp float;
precision highp int;

struct FilterInstanceData {
    uint source_origin;
    uint source_size;
    uint dest_origin;
    uint dest_size;
    uint dest_texture_size;
    uint filter_data_offset;
    uint original_origin;
    uint original_size;
    uint filter_pass_kind;
};
struct FilterVertexOutput {
    vec4 position;
    uint filter_data_offset;
    uvec2 source_origin;
    uvec2 source_size;
    uvec2 dest_origin;
    uvec2 dest_size;
    uvec2 original_origin;
    uint filter_pass_kind;
};
const uint FILTER_SIZE_BYTES = 48u;
const uint FILTER_SIZE_U32_ = 12u;
const uint TEXELS_PER_FILTER = 3u;
const uint FILTER_TYPE_OFFSET = 0u;
const uint FILTER_TYPE_FLOOD = 1u;
const uint FILTER_TYPE_GAUSSIAN_BLUR = 2u;
const uint FILTER_TYPE_DROP_SHADOW = 3u;
const uint PASS_COPY = 0u;
const uint PASS_FLOOD = 1u;
const uint PASS_OFFSET = 2u;
const uint PASS_DOWNSCALE = 3u;
const uint PASS_BLUR_H = 4u;
const uint PASS_BLUR_V = 5u;
const uint PASS_UPSCALE = 6u;
const uint PASS_COMPOSITE_DROP_SHADOW = 7u;
const uint PASS_COLORIZE = 8u;
const uint FILTER_ATLAS_PADDING = 6u;
const uint MAX_TAPS_PER_SIDE = 3u;
const vec2 HORIZONTAL = vec2(1.0, 0.0);
const vec2 VERTICAL = vec2(0.0, 1.0);

uniform highp usampler2D _group_0_binding_0_fs;

uniform highp sampler2D _group_1_binding_0_fs;

uniform highp sampler2D _group_2_binding_0_fs;

flat in uint _vs2fs_location0;
flat in uvec2 _vs2fs_location1;
flat in uvec2 _vs2fs_location2;
flat in uvec2 _vs2fs_location3;
flat in uvec2 _vs2fs_location4;
flat in uvec2 _vs2fs_location5;
flat in uint _vs2fs_location6;
layout(location = 0) out vec4 _fs2p_location0;

uvec4 load_filter_texel(uint texel_offset, uint texel_index) {
    uint w = uvec2(textureSize(_group_0_binding_0_fs, 0).xy).x;
    uint flat_index = (texel_offset + texel_index);
    uvec4 _e11 = texelFetch(_group_0_binding_0_fs, ivec2(uvec2((flat_index % w), (flat_index / w))), 0);
    return _e11;
}

uint get_filter_type(uvec4 texel0_) {
    return (texel0_.x & 31u);
}

uint get_filter_header_n_linear_taps(uvec4 texel0_1) {
    return ((texel0_1.x >> 11u) & 3u);
}

float get_offset_dx(uvec4 texel0_2) {
    return uintBitsToFloat(texel0_2.y);
}

float get_offset_dy(uvec4 texel0_3) {
    return uintBitsToFloat(texel0_3.z);
}

uint get_flood_color(uvec4 texel0_4) {
    return texel0_4.y;
}

float get_blur_center_weight(uvec4 texel0_5) {
    return uintBitsToFloat(texel0_5.y);
}

vec3 get_blur_linear_weights(uvec4 texel0_6, uvec4 texel1_) {
    return vec3(uintBitsToFloat(texel0_6.z), uintBitsToFloat(texel0_6.w), uintBitsToFloat(texel1_.x));
}

vec3 get_blur_linear_offsets(uvec4 texel1_1) {
    return vec3(uintBitsToFloat(texel1_1.y), uintBitsToFloat(texel1_1.z), uintBitsToFloat(texel1_1.w));
}

float get_drop_shadow_dx(uvec4 texel2_) {
    return uintBitsToFloat(texel2_.x);
}

float get_drop_shadow_dy(uvec4 texel2_1) {
    return uintBitsToFloat(texel2_1.y);
}

uint get_drop_shadow_color(uvec4 texel2_2) {
    return texel2_2.z;
}

uvec2 unpack_u16_pair(uint value) {
    return uvec2((value & 65535u), (value >> 16u));
}

vec4 sample_original(uvec2 original_origin_1, vec2 rel_coord) {
    uvec2 source_coord = uvec2((ivec2(original_origin_1) + ivec2(rel_coord)));
    vec4 _e8 = texelFetch(_group_2_binding_0_fs, ivec2(source_coord), 0);
    return _e8;
}

vec4 sample_source(uvec2 source_origin_1, vec2 rel_coord_1) {
    uvec2 source_coord_1 = uvec2((ivec2(source_origin_1) + ivec2(rel_coord_1)));
    vec4 _e8 = texelFetch(_group_1_binding_0_fs, ivec2(source_coord_1), 0);
    return _e8;
}

vec4 sample_source_checked(uvec2 source_origin_2, uvec2 source_size_1, vec2 rel_coord_2) {
    bool local_1 = false;
    bool local_2 = false;
    bool local_3 = false;
    if (!((rel_coord_2.x < 0.0))) {
        local_1 = (rel_coord_2.x >= float(source_size_1.x));
    } else {
        local_1 = true;
    }
    bool _e14 = local_1;
    if (!(_e14)) {
        local_2 = (rel_coord_2.y < 0.0);
    } else {
        local_2 = true;
    }
    bool _e22 = local_2;
    if (!(_e22)) {
        local_3 = (rel_coord_2.y >= float(source_size_1.y));
    } else {
        local_3 = true;
    }
    bool _e31 = local_3;
    if (_e31) {
        return vec4(0.0);
    }
    vec4 _e34 = sample_source(source_origin_2, rel_coord_2);
    return _e34;
}

vec4 downscale(vec4 position_1, uvec2 source_origin_3, uvec2 dest_origin_1) {
    uvec2 frag_coord = uvec2(position_1.xy);
    ivec2 rel = ivec2((frag_coord - dest_origin_1));
    vec2 source_rel_1 = vec2((rel * 2));
    vec2 source_texel = (vec2(source_origin_3) + source_rel_1);
    vec2 source_texture_size = vec2(uvec2(textureSize(_group_1_binding_0_fs, 0).xy));
    vec2 lo = vec2(-0.25);
    vec2 hi = vec2(1.25);
    vec4 s00_ = textureLod(_group_1_binding_0_fs, vec2((((source_texel + vec2(lo.x, lo.y)) + vec2(0.5)) / source_texture_size)), 0.0);
    vec4 s01_ = textureLod(_group_1_binding_0_fs, vec2((((source_texel + vec2(lo.x, hi.y)) + vec2(0.5)) / source_texture_size)), 0.0);
    vec4 s10_ = textureLod(_group_1_binding_0_fs, vec2((((source_texel + vec2(hi.x, lo.y)) + vec2(0.5)) / source_texture_size)), 0.0);
    vec4 s11_ = textureLod(_group_1_binding_0_fs, vec2((((source_texel + vec2(hi.x, hi.y)) + vec2(0.5)) / source_texture_size)), 0.0);
    return ((((s00_ + s01_) + s10_) + s11_) * 0.25);
}

vec4 upscale(vec4 position_2, uvec2 source_origin_4, uvec2 dest_origin_2) {
    uvec2 frag_coord_1 = uvec2(position_2.xy);
    ivec2 rel_1 = ivec2((frag_coord_1 - dest_origin_2));
    vec2 source_base = vec2((rel_1 / ivec2(2)));
    vec2 phase = vec2((rel_1 % ivec2(2)));
    vec2 source_texture_size_1 = vec2(uvec2(textureSize(_group_1_binding_0_fs, 0).xy));
    vec2 sample_offset = mix(vec2(-0.25), vec2(0.25), equal(phase, vec2(1.0)));
    vec2 source_texel_1 = ((vec2(source_origin_4) + source_base) + sample_offset);
    vec4 _e36 = textureLod(_group_1_binding_0_fs, vec2(((source_texel_1 + vec2(0.5)) / source_texture_size_1)), 0.0);
    return _e36;
}

vec4 convolve(uvec2 source_origin_5, vec2 source_rel, vec2 dir, uint n_linear_taps, float center_weight, vec3 weights, vec3 offsets) {
    vec4 color = vec4(0.0);
    float weights_arr[3] = float[3](0.0, 0.0, 0.0);
    float offsets_arr[3] = float[3](0.0, 0.0, 0.0);
    uint i = 0u;
    vec2 source_texel_2 = (vec2(source_origin_5) + source_rel);
    vec2 source_texture_size_2 = vec2(uvec2(textureSize(_group_1_binding_0_fs, 0).xy));
    vec4 _e19 = textureLod(_group_1_binding_0_fs, vec2(((source_texel_2 + vec2(0.5)) / source_texture_size_2)), 0.0);
    color = (_e19 * center_weight);
    weights_arr[0] = weights.x;
    weights_arr[1] = weights.y;
    weights_arr[2] = weights.z;
    offsets_arr[0] = offsets.x;
    offsets_arr[1] = offsets.y;
    offsets_arr[2] = offsets.z;
    bool loop_init = true;
    while(true) {
        if (!loop_init) {
            uint _e72 = i;
            i = (_e72 + 1u);
        }
        loop_init = false;
        uint _e38 = i;
        if ((_e38 < n_linear_taps)) {
        } else {
            break;
        }
        {
            uint _e40 = i;
            float w_1 = weights_arr[_e40];
            uint _e43 = i;
            float _e45 = offsets_arr[_e43];
            vec2 d = (dir * _e45);
            vec4 _e47 = color;
            vec4 _e56 = textureLod(_group_1_binding_0_fs, vec2((((source_texel_2 + d) + vec2(0.5)) / source_texture_size_2)), 0.0);
            color = (_e47 + (_e56 * w_1));
            vec4 _e59 = color;
            vec4 _e68 = textureLod(_group_1_binding_0_fs, vec2((((source_texel_2 - d) + vec2(0.5)) / source_texture_size_2)), 0.0);
            color = (_e59 + (_e68 * w_1));
        }
    }
    vec4 _e74 = color;
    return _e74;
}

void main() {
    uint filter_data_offset = _vs2fs_location0;
    uvec2 source_origin = _vs2fs_location1;
    uvec2 source_size = _vs2fs_location2;
    uvec2 dest_origin = _vs2fs_location3;
    uvec2 dest_size = _vs2fs_location4;
    uvec2 original_origin = _vs2fs_location5;
    uint filter_pass_kind = _vs2fs_location6;
    vec4 position = gl_FragCoord;
    bool local = false;
    vec2 dxdy = vec2(0.0);
    uvec2 frag_coord_2 = uvec2(position.xy);
    vec2 rel_coord_3 = vec2((frag_coord_2 - dest_origin));
    if (!((rel_coord_3.x >= float(dest_size.x)))) {
        local = (rel_coord_3.y >= float(dest_size.y));
    } else {
        local = true;
    }
    bool _e24 = local;
    if (_e24) {
        _fs2p_location0 = vec4(0.0);
        return;
    }
    switch(filter_pass_kind) {
        case 0u: {
            vec4 _e27 = sample_source(source_origin, rel_coord_3);
            _fs2p_location0 = _e27;
            return;
        }
        case 1u: {
            uvec4 _e29 = load_filter_texel(filter_data_offset, 0u);
            uint _e30 = get_flood_color(_e29);
            _fs2p_location0 = (vec4(_e30 & 0xFFu, _e30 >> 8 & 0xFFu, _e30 >> 16 & 0xFFu, _e30 >> 24) / 255.0);
            return;
        }
        case 2u: {
            uvec4 _e33 = load_filter_texel(filter_data_offset, 0u);
            uint _e35 = get_filter_type(_e33);
            if ((_e35 == FILTER_TYPE_DROP_SHADOW)) {
                uvec4 _e39 = load_filter_texel(filter_data_offset, 2u);
                float _e40 = get_drop_shadow_dx(_e39);
                float _e41 = get_drop_shadow_dy(_e39);
                dxdy = vec2(_e40, _e41);
            } else {
                float _e43 = get_offset_dx(_e33);
                float _e44 = get_offset_dy(_e33);
                dxdy = vec2(_e43, _e44);
            }
            vec2 _e46 = dxdy;
            vec4 _e52 = sample_source_checked(source_origin, source_size, (rel_coord_3 - floor((_e46 + vec2(0.5)))));
            _fs2p_location0 = _e52;
            return;
        }
        case 3u: {
            vec4 _e53 = downscale(position, source_origin, dest_origin);
            _fs2p_location0 = _e53;
            return;
        }
        case 4u: {
            uvec4 _e55 = load_filter_texel(filter_data_offset, 0u);
            uvec4 _e57 = load_filter_texel(filter_data_offset, 1u);
            uint _e59 = get_filter_header_n_linear_taps(_e55);
            float _e60 = get_blur_center_weight(_e55);
            vec3 _e61 = get_blur_linear_weights(_e55, _e57);
            vec3 _e62 = get_blur_linear_offsets(_e57);
            vec4 _e63 = convolve(source_origin, rel_coord_3, HORIZONTAL, _e59, _e60, _e61, _e62);
            _fs2p_location0 = _e63;
            return;
        }
        case 5u: {
            uvec4 _e65 = load_filter_texel(filter_data_offset, 0u);
            uvec4 _e67 = load_filter_texel(filter_data_offset, 1u);
            uint _e69 = get_filter_header_n_linear_taps(_e65);
            float _e70 = get_blur_center_weight(_e65);
            vec3 _e71 = get_blur_linear_weights(_e65, _e67);
            vec3 _e72 = get_blur_linear_offsets(_e67);
            vec4 _e73 = convolve(source_origin, rel_coord_3, VERTICAL, _e69, _e70, _e71, _e72);
            _fs2p_location0 = _e73;
            return;
        }
        case 6u: {
            vec4 _e74 = upscale(position, source_origin, dest_origin);
            _fs2p_location0 = _e74;
            return;
        }
        case 7u: {
            uvec4 _e76 = load_filter_texel(filter_data_offset, 2u);
            vec4 _e77 = sample_source(source_origin, rel_coord_3);
            uint _e78 = get_drop_shadow_color(_e76);
            vec4 shadow_color = (vec4(_e78 & 0xFFu, _e78 >> 8 & 0xFFu, _e78 >> 16 & 0xFFu, _e78 >> 24) / 255.0);
            vec4 shadow_result = (shadow_color * _e77.w);
            vec4 _e82 = sample_original(original_origin, rel_coord_3);
            _fs2p_location0 = (_e82 + (shadow_result * (1.0 - _e82.w)));
            return;
        }
        case 8u: {
            uvec4 _e89 = load_filter_texel(filter_data_offset, 2u);
            vec4 _e90 = sample_source(source_origin, rel_coord_3);
            uint _e91 = get_drop_shadow_color(_e89);
            vec4 shadow_color_1 = (vec4(_e91 & 0xFFu, _e91 >> 8 & 0xFFu, _e91 >> 16 & 0xFFu, _e91 >> 24) / 255.0);
            _fs2p_location0 = (shadow_color_1 * _e90.w);
            return;
        }
        default: {
            _fs2p_location0 = vec4(0.0);
            return;
        }
    }
}

"###;
    pub mod fragment {
        pub const FILTER_DATA: &str = "_group_0_binding_0_fs";
        pub const ORIGINAL_TEXTURE: &str = "_group_2_binding_0_fs";
        pub const SOURCE_TEXTURE: &str = "_group_1_binding_0_fs";
    }
}
#[cfg(feature = "glsl")]
/// Compiled GLSL for `render.wesl`.
pub mod render {
    #![allow(
        missing_docs,
        reason = "No metadata to generate precise documentation for generated code."
    )]

    pub const VERTEX_SOURCE: &str = r###"#version 300 es

precision highp float;
precision highp int;

struct Config {
    uint width;
    uint height;
    uint strip_height;
    uint alphas_tex_width_bits;
    uint encoded_paints_tex_width_bits;
    int strip_offset_x;
    int strip_offset_y;
    uint ndc_y_negate;
};
struct StripInstance {
    uint xy;
    uint widths_or_rect_height;
    uint col_idx_or_rect_frac;
    uint payload;
    uint paint_and_rect_flag;
    uint depth_index;
};
struct VertexOutput {
    uint paint_and_rect_flag;
    vec2 tex_coord;
    vec2 sample_xy;
    uint dense_end_or_rect_size;
    uint payload;
    uint rect_frac;
    vec4 position;
};
const uint COLOR_SOURCE_PAYLOAD = 0u;
const uint COLOR_SOURCE_LAYER = 1u;
const uint PAINT_TYPE_SOLID = 0u;
const uint PAINT_TYPE_IMAGE = 1u;
const uint PAINT_TYPE_LINEAR_GRADIENT = 2u;
const uint PAINT_TYPE_RADIAL_GRADIENT = 3u;
const uint PAINT_TYPE_SWEEP_GRADIENT = 4u;
const uint PAINT_TYPE_BLURRED_ROUNDED_RECT = 5u;
const uint PAINT_TEXTURE_INDEX_MASK = 67108863u;
const uint RECT_STRIP_FLAG = 2147483648u;
const uint IMAGE_QUALITY_LOW = 0u;
const uint IMAGE_QUALITY_MEDIUM = 1u;
const uint IMAGE_QUALITY_HIGH = 2u;
const uint IMAGE_SOURCE_ATLAS = 0u;
const uint IMAGE_SOURCE_EXTERNAL = 1u;
const uint GRADIENT_TYPE_LINEAR = 0u;
const uint GRADIENT_TYPE_RADIAL = 1u;
const uint GRADIENT_TYPE_SWEEP = 2u;
const uint RADIAL_GRADIENT_TYPE_STANDARD = 0u;
const uint RADIAL_GRADIENT_TYPE_STRIP = 1u;
const uint RADIAL_GRADIENT_TYPE_FOCAL = 2u;
const float PI = 3.1415927;
const float TWO_PI = 6.2831855;
const float NEARLY_ZERO_TOLERANCE = 0.00024414063;
const uint TINT_MODE_MULTIPLY = 1u;
const uint EXTEND_PAD = 0u;
const uint EXTEND_REPEAT = 1u;
const uint EXTEND_REFLECT = 2u;
const vec4 MF[4] = vec4[4](vec4(0.055555556, -0.5, 0.8333333, -0.3888889), vec4(0.8888889, 0.0, -2.0, 1.1666666), vec4(0.055555556, 0.5, 1.5, -1.1666666), vec4(0.0, 0.0, -0.33333334, 0.3888889));
const uint TINT_MODE_ALPHA_MASK = 0u;

layout(std140) uniform Config_block_0Vertex { Config _group_0_binding_1_vs; };

uniform highp usampler2D _group_2_binding_0_vs;

layout(location = 0) in uint _p2vs_location0;
layout(location = 1) in uint _p2vs_location1;
layout(location = 2) in uint _p2vs_location2;
layout(location = 3) in uint _p2vs_location3;
layout(location = 4) in uint _p2vs_location4;
layout(location = 5) in uint _p2vs_location5;
flat out uint _vs2fs_location0;
smooth out vec2 _vs2fs_location1;
smooth out vec2 _vs2fs_location2;
flat out uint _vs2fs_location3;
flat out uint _vs2fs_location4;
flat out uint _vs2fs_location5;

uvec2 encoded_paint_coord(uint flat_idx) {
    uint _e4 = _group_0_binding_1_vs.encoded_paints_tex_width_bits;
    uint _e11 = _group_0_binding_1_vs.encoded_paints_tex_width_bits;
    return uvec2((flat_idx & ((1u << _e4) - 1u)), (flat_idx >> _e11));
}

uvec4 load_encoded_paint_texel(uint paint_tex_idx, uint texel_offset) {
    uvec2 _e4 = encoded_paint_coord((paint_tex_idx + texel_offset));
    uvec4 _e6 = texelFetch(_group_2_binding_0_vs, ivec2(_e4), 0);
    return _e6;
}

vec2 get_image_translate(uvec4 texel1_, uvec4 texel2_) {
    return vec2(uintBitsToFloat(texel1_.w), uintBitsToFloat(texel2_.x));
}

vec2 get_image_offset(uvec4 texel0_) {
    return vec2(float((texel0_.z >> 16u)), float((texel0_.z & 65535u)));
}

mat2x2 get_image_transform(uvec4 texel0_1, uvec4 texel1_1) {
    return mat2x2(vec2(uintBitsToFloat(texel0_1.w), uintBitsToFloat(texel1_1.x)), vec2(uintBitsToFloat(texel1_1.y), uintBitsToFloat(texel1_1.z)));
}

uint unpack_alphas_from_channel(uvec4 rgba, uint channel_index) {
    switch(channel_index) {
        case 0u: {
            return rgba.x;
        }
        case 1u: {
            return rgba.y;
        }
        case 2u: {
            return rgba.z;
        }
        case 3u: {
            return rgba.w;
        }
        default: {
            return rgba.x;
        }
    }
}

vec2 get_image_size(uvec4 texel0_2) {
    return vec2(float((texel0_2.y >> 16u)), float((texel0_2.y & 65535u)));
}

uvec2 get_image_extend_modes(uvec4 texel0_3) {
    return uvec2(((texel0_3.x >> 2u) & 3u), ((texel0_3.x >> 4u) & 3u));
}

uint get_image_atlas_index(uvec4 texel0_4) {
    return ((texel0_4.x >> 6u) & 255u);
}

uint get_image_quality(uvec4 texel0_5) {
    return (texel0_5.x & 3u);
}

uint get_image_source_kind(uvec4 texel0_6) {
    return ((texel0_6.x >> 14u) & 1u);
}

float get_image_padding(uvec4 texel2_1) {
    return float(texel2_1.w);
}

float extend_mode_normalized(float t, uint mode) {
    switch(mode) {
        case 0u: {
            return clamp(t, 0.0, 1.0);
        }
        case 1u: {
            return fract(t);
        }
        case 2u:
        default: {
            return abs((t - (2.0 * roundEven((0.5 * t)))));
        }
    }
}

float extend_mode(float t_1, uint mode_1, float max_) {
    switch(mode_1) {
        case 0u: {
            return clamp(t_1, 0.5, (max_ - 0.5));
        }
        case 1u: {
            float _e8 = extend_mode_normalized((t_1 / max_), mode_1);
            return (_e8 * max_);
        }
        case 2u:
        default: {
            float _e11 = extend_mode_normalized((t_1 / max_), mode_1);
            return (_e11 * max_);
        }
    }
}

float single_weight(float t_2, float a, float b, float c, float d) {
    return ((t_2 * ((t_2 * ((t_2 * d) + c)) + b)) + a);
}

vec4 cubic_weights(float fract_) {
    float _e5 = single_weight(fract_, 0.055555556, -0.5, 0.8333333, -0.3888889);
    float _e10 = single_weight(fract_, 0.8888889, 0.0, -2.0, 1.1666666);
    float _e15 = single_weight(fract_, 0.055555556, 0.5, 1.5, -1.1666666);
    float _e20 = single_weight(fract_, 0.0, 0.0, -0.33333334, 0.3888889);
    return vec4(_e5, _e10, _e15, _e20);
}

vec4 bicubic_sample(highp sampler2DArray tex, vec2 coords_3, int atlas_idx, vec2 image_offset_3, vec2 image_size_3, uvec2 _extend_modes, float _image_padding) {
    vec2 atlas_max = ((image_offset_3 + image_size_3) - vec2(1.0));
    vec2 frac_coords = fract((coords_3 + vec2(0.5)));
    vec4 _e16 = cubic_weights(frac_coords.x);
    vec4 _e18 = cubic_weights(frac_coords.y);
    vec4 s00_ = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(-1.5, -1.5)), image_offset_3, atlas_max)), atlas_idx), 0);
    vec4 s10_ = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(-0.5, -1.5)), image_offset_3, atlas_max)), atlas_idx), 0);
    vec4 s20_ = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(0.5, -1.5)), image_offset_3, atlas_max)), atlas_idx), 0);
    vec4 s30_ = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(1.5, -1.5)), image_offset_3, atlas_max)), atlas_idx), 0);
    vec4 s01_ = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(-1.5, -0.5)), image_offset_3, atlas_max)), atlas_idx), 0);
    vec4 s11_ = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(-0.5, -0.5)), image_offset_3, atlas_max)), atlas_idx), 0);
    vec4 s21_ = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(0.5, -0.5)), image_offset_3, atlas_max)), atlas_idx), 0);
    vec4 s31_ = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(1.5, -0.5)), image_offset_3, atlas_max)), atlas_idx), 0);
    vec4 s02_ = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(-1.5, 0.5)), image_offset_3, atlas_max)), atlas_idx), 0);
    vec4 s12_ = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(-0.5, 0.5)), image_offset_3, atlas_max)), atlas_idx), 0);
    vec4 s22_ = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(0.5, 0.5)), image_offset_3, atlas_max)), atlas_idx), 0);
    vec4 s32_ = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(1.5, 0.5)), image_offset_3, atlas_max)), atlas_idx), 0);
    vec4 s03_ = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(-1.5, 1.5)), image_offset_3, atlas_max)), atlas_idx), 0);
    vec4 s13_ = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(-0.5, 1.5)), image_offset_3, atlas_max)), atlas_idx), 0);
    vec4 s23_ = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(0.5, 1.5)), image_offset_3, atlas_max)), atlas_idx), 0);
    vec4 s33_ = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(1.5, 1.5)), image_offset_3, atlas_max)), atlas_idx), 0);
    vec4 row0_ = ((((_e16.x * s00_) + (_e16.y * s10_)) + (_e16.z * s20_)) + (_e16.w * s30_));
    vec4 row1_ = ((((_e16.x * s01_) + (_e16.y * s11_)) + (_e16.z * s21_)) + (_e16.w * s31_));
    vec4 row2_ = ((((_e16.x * s02_) + (_e16.y * s12_)) + (_e16.z * s22_)) + (_e16.w * s32_));
    vec4 row3_ = ((((_e16.x * s03_) + (_e16.y * s13_)) + (_e16.z * s23_)) + (_e16.w * s33_));
    vec4 result = ((((_e18.x * row0_) + (_e18.y * row1_)) + (_e18.z * row2_)) + (_e18.w * row3_));
    float a_1 = clamp(result.w, 0.0, 1.0);
    return vec4(clamp(result.xyz, vec3(0.0), vec3(a_1)), a_1);
}

vec4 bilinear_sample(highp sampler2DArray tex_1, vec2 coords_4, int atlas_idx_1, vec2 image_offset_4, vec2 image_size_4, uvec2 _extend_modes_1, float _image_padding_1) {
    vec2 atlas_max_1 = ((image_offset_4 + image_size_4) - vec2(1.0));
    vec2 atlas_uv_clamped = clamp(coords_4, image_offset_4, atlas_max_1);
    vec4 uv_quad = vec4(floor(atlas_uv_clamped), ceil(atlas_uv_clamped));
    vec2 uv_frac = fract(coords_4);
    vec4 a_2 = texelFetch(tex_1, ivec3(ivec2(uv_quad.xy), atlas_idx_1), 0);
    vec4 b_1 = texelFetch(tex_1, ivec3(ivec2(uv_quad.xw), atlas_idx_1), 0);
    vec4 c_1 = texelFetch(tex_1, ivec3(ivec2(uv_quad.zy), atlas_idx_1), 0);
    vec4 d_1 = texelFetch(tex_1, ivec3(ivec2(uv_quad.zw), atlas_idx_1), 0);
    return mix(mix(a_2, b_1, uv_frac.y), mix(c_1, d_1, uv_frac.y), uv_frac.x);
}

mat2x2 get_gradient_transform(uvec4 texel0_7, uvec4 texel1_2) {
    return mat2x2(vec2(uintBitsToFloat(texel0_7.z), uintBitsToFloat(texel0_7.w)), vec2(uintBitsToFloat(texel1_2.x), uintBitsToFloat(texel1_2.y)));
}

vec2 get_gradient_translate(uvec4 texel1_3) {
    return vec2(uintBitsToFloat(texel1_3.z), uintBitsToFloat(texel1_3.w));
}

vec2 apply_gradient_transform(uvec4 texel0_8, uvec4 texel1_4, vec2 fragment_pos) {
    mat2x2 _e3 = get_gradient_transform(texel0_8, texel1_4);
    vec2 _e5 = get_gradient_translate(texel1_4);
    return ((_e3 * fragment_pos) + _e5);
}

uint get_gradient_extend_mode(uvec4 texel0_9) {
    return ((texel0_9.x >> 30u) & 3u);
}

uint get_gradient_start(uvec4 texel0_10) {
    return texel0_10.y;
}

uint get_gradient_texture_width(uvec4 texel0_11) {
    return (texel0_11.x & 268435455u);
}

uint get_radial_kind(uvec4 texel2_2) {
    return (texel2_2.x & 3u);
}

float get_radial_bias(uvec4 texel2_3) {
    return uintBitsToFloat(texel2_3.y);
}

float get_radial_scale(uvec4 texel2_4) {
    return uintBitsToFloat(texel2_4.z);
}

float get_radial_scaled_r0_squared(uvec4 texel3_) {
    return uintBitsToFloat(texel3_.w);
}

float get_radial_fp0_(uvec4 texel2_5) {
    return uintBitsToFloat(texel2_5.w);
}

float get_radial_fp1_(uvec4 texel3_1) {
    return uintBitsToFloat(texel3_1.x);
}

float get_radial_fr1_(uvec4 texel3_2) {
    return uintBitsToFloat(texel3_2.y);
}

float get_radial_f_focal_x(uvec4 texel3_3) {
    return uintBitsToFloat(texel3_3.z);
}

uint get_radial_f_is_swapped(uvec4 texel2_6) {
    return ((texel2_6.x >> 2u) & 1u);
}

vec2 calculate_radial_gradient(vec2 grad_pos_1, uvec4 texel2_7, uvec4 texel3_4) {
    float t_value_1 = 0.0;
    bool is_valid = false;
    float t_3 = 0.0;
    bool local_5 = false;
    bool local_6 = false;
    bool local_7 = false;
    bool local_8 = false;
    float x_pos = grad_pos_1.x;
    float y_pos = grad_pos_1.y;
    uint _e7 = get_radial_kind(texel2_7);
    switch(_e7) {
        case 0u: {
            float radius = sqrt(((x_pos * x_pos) + (y_pos * y_pos)));
            float _e12 = get_radial_bias(texel2_7);
            float _e13 = get_radial_scale(texel2_7);
            t_value_1 = (_e12 + (_e13 * radius));
            is_valid = true;
            break;
        }
        case 1u: {
            float _e17 = get_radial_scaled_r0_squared(texel3_4);
            float p1_ = (_e17 - (y_pos * y_pos));
            is_valid = (p1_ >= 0.0);
            bool _e22 = is_valid;
            if (_e22) {
                t_value_1 = (x_pos + sqrt(p1_));
            } else {
                t_value_1 = 0.0;
            }
            break;
        }
        case 2u:
        default: {
            float _e28 = get_radial_fp0_(texel2_7);
            float _e29 = get_radial_fp1_(texel3_4);
            float _e30 = get_radial_fr1_(texel3_4);
            float _e31 = get_radial_f_focal_x(texel3_4);
            uint _e32 = get_radial_f_is_swapped(texel2_7);
            bool is_focal_on_circle = (abs((1.0 - _e30)) <= NEARLY_ZERO_TOLERANCE);
            if (!(is_focal_on_circle)) {
                local_5 = (_e30 > 1.0);
            } else {
                local_5 = false;
            }
            bool is_well_behaved = local_5;
            bool is_natively_focal = (abs(_e31) <= NEARLY_ZERO_TOLERANCE);
            is_valid = true;
            if (is_focal_on_circle) {
                t_3 = (x_pos + ((y_pos * y_pos) / x_pos));
                float _e52 = t_3;
                if ((_e52 >= 0.0)) {
                    local_6 = (x_pos != 0.0);
                } else {
                    local_6 = false;
                }
                bool _e60 = local_6;
                is_valid = _e60;
            } else {
                if (is_well_behaved) {
                    t_3 = (sqrt(((x_pos * x_pos) + (y_pos * y_pos))) - (x_pos * _e28));
                } else {
                    float xx = (x_pos * x_pos);
                    float yy = (y_pos * y_pos);
                    float discriminant = (xx - yy);
                    if (!((_e32 != 0u))) {
                        local_7 = ((1.0 - _e31) < 0.0);
                    } else {
                        local_7 = true;
                    }
                    bool _e80 = local_7;
                    if (_e80) {
                        t_3 = (-(sqrt(discriminant)) - (x_pos * _e28));
                    } else {
                        t_3 = (sqrt(discriminant) - (x_pos * _e28));
                    }
                    if ((discriminant >= 0.0)) {
                        float _e92 = t_3;
                        local_8 = (_e92 >= 0.0);
                    } else {
                        local_8 = false;
                    }
                    bool _e96 = local_8;
                    is_valid = _e96;
                }
            }
            bool _e97 = is_valid;
            if (_e97) {
                if (((1.0 - _e31) < 0.0)) {
                    float _e102 = t_3;
                    t_3 = -(_e102);
                }
                if (!(is_natively_focal)) {
                    float _e105 = t_3;
                    t_3 = (_e105 + _e29);
                }
                if ((_e32 != 0u)) {
                    float _e109 = t_3;
                    t_3 = (1.0 - _e109);
                }
            }
            float _e112 = t_3;
            t_value_1 = _e112;
            break;
        }
    }
    float _e113 = t_value_1;
    bool _e114 = is_valid;
    return vec2(_e113, (_e114 ? 1.0 : 0.0));
}

float xy_to_unit_angle(float x, float y) {
    float phi = 0.0;
    float xabs = abs(x);
    float yabs = abs(y);
    float slope = (min(xabs, yabs) / max(xabs, yabs));
    float s = (slope * slope);
    phi = (slope * (0.15912117 + (s * (-0.05185397 + (s * (0.02476102 + (s * -0.0070547382)))))));
    float _e20 = phi;
    float _e21 = phi;
    phi = ((xabs < yabs) ? (0.25 - _e21) : _e20);
    float _e26 = phi;
    float _e27 = phi;
    phi = ((x < 0.0) ? (0.5 - _e27) : _e26);
    float _e33 = phi;
    float _e34 = phi;
    phi = ((y < 0.0) ? (1.0 - _e34) : _e33);
    float _e40 = phi;
    float _e41 = phi;
    float _e42 = phi;
    phi = ((_e41 != _e42) ? 0.0 : _e40);
    float _e46 = phi;
    return _e46;
}

float get_sweep_start_angle(uvec4 texel2_8) {
    return uintBitsToFloat(texel2_8.x);
}

float get_sweep_inv_angle_delta(uvec4 texel2_9) {
    return uintBitsToFloat(texel2_9.y);
}

mat2x2 get_blurred_rounded_rect_transform(uvec4 texel0_12) {
    return mat2x2(vec2(uintBitsToFloat(texel0_12.x), uintBitsToFloat(texel0_12.y)), vec2(uintBitsToFloat(texel0_12.z), uintBitsToFloat(texel0_12.w)));
}

vec2 get_blurred_rounded_rect_translate(uvec4 texel1_5) {
    return vec2(uintBitsToFloat(texel1_5.x), uintBitsToFloat(texel1_5.y));
}

vec4 get_blurred_rounded_rect_color(uvec4 texel1_6) {
    uint _e1 = texel1_6.z;
    return (vec4(_e1 & 0xFFu, _e1 >> 8 & 0xFFu, _e1 >> 16 & 0xFFu, _e1 >> 24) / 255.0);
}

uint get_blurred_rounded_rect_invert(uvec4 texel1_7) {
    return texel1_7.w;
}

float get_blurred_rounded_rect_exponent(uvec4 texel2_10) {
    return uintBitsToFloat(texel2_10.x);
}

float get_blurred_rounded_rect_recip_exponent(uvec4 texel2_11) {
    return uintBitsToFloat(texel2_11.y);
}

float get_blurred_rounded_rect_scale(uvec4 texel2_12) {
    return uintBitsToFloat(texel2_12.z);
}

float get_blurred_rounded_rect_std_dev_inv(uvec4 texel2_13) {
    return uintBitsToFloat(texel2_13.w);
}

float get_blurred_rounded_rect_min_edge(uvec4 texel3_5) {
    return uintBitsToFloat(texel3_5.x);
}

float get_blurred_rounded_rect_w(uvec4 texel3_6) {
    return uintBitsToFloat(texel3_6.y);
}

float get_blurred_rounded_rect_h(uvec4 texel3_7) {
    return uintBitsToFloat(texel3_7.z);
}

float get_blurred_rounded_rect_r1_(uvec4 texel3_8) {
    return uintBitsToFloat(texel3_8.w);
}

float get_blurred_rounded_rect_width(uvec4 texel4_) {
    return uintBitsToFloat(texel4_.x);
}

float get_blurred_rounded_rect_height(uvec4 texel4_1) {
    return uintBitsToFloat(texel4_1.y);
}

float erf7_(float x_1) {
    float y_1 = clamp((x_1 * 1.1283792), -100.0, 100.0);
    float yy_1 = (y_1 * y_1);
    float z = (y_1 + ((0.24295 + ((0.03395 + (0.0104 * yy_1)) * yy_1)) * (y_1 * yy_1)));
    return (z / sqrt((1.0 + (z * z))));
}

vec4 calculate_blurred_rounded_rect(vec2 fragment_pos_1, uvec4 texel0_13, uvec4 texel1_8, uvec4 texel2_14, uvec4 texel3_9, uvec4 texel4_2) {
    mat2x2 _e6 = get_blurred_rounded_rect_transform(texel0_13);
    vec2 _e7 = get_blurred_rounded_rect_translate(texel1_8);
    vec4 _e8 = get_blurred_rounded_rect_color(texel1_8);
    uint _e9 = get_blurred_rounded_rect_invert(texel1_8);
    float _e10 = get_blurred_rounded_rect_exponent(texel2_14);
    float _e11 = get_blurred_rounded_rect_recip_exponent(texel2_14);
    float _e12 = get_blurred_rounded_rect_scale(texel2_14);
    float _e13 = get_blurred_rounded_rect_std_dev_inv(texel2_14);
    float _e14 = get_blurred_rounded_rect_min_edge(texel3_9);
    float _e15 = get_blurred_rounded_rect_w(texel3_9);
    float _e16 = get_blurred_rounded_rect_h(texel3_9);
    float _e17 = get_blurred_rounded_rect_r1_(texel3_9);
    float _e18 = get_blurred_rounded_rect_width(texel4_2);
    float _e19 = get_blurred_rounded_rect_height(texel4_2);
    vec2 local_xy = ((_e6 * fragment_pos_1) + _e7);
    float y_2 = (local_xy.y - (0.5 * _e19));
    float y0_ = ((_e17 + abs(y_2)) - (0.5 * _e16));
    float y1_ = max(y0_, 0.0);
    float x_2 = (local_xy.x - (0.5 * _e18));
    float x0_ = ((_e17 + abs(x_2)) - (0.5 * _e15));
    float x1_ = max(x0_, 0.0);
    float d_pos = pow((pow(x1_, _e10) + pow(y1_, _e10)), _e11);
    float d_neg = min(max(x0_, y0_), 0.0);
    float d_2 = ((d_pos + d_neg) - _e17);
    float _e55 = erf7_((_e13 * (_e14 + d_2)));
    float _e57 = erf7_((_e13 * d_2));
    float blur_coverage = (_e12 * (_e55 - _e57));
    float blur_alpha = ((_e9 != 0u) ? (1.0 - blur_coverage) : blur_coverage);
    return (_e8 * blur_alpha);
}

void main() {
    uint in_vertex_index = uint(gl_VertexID);
    StripInstance instance = StripInstance(_p2vs_location0, _p2vs_location1, _p2vs_location2, _p2vs_location3, _p2vs_location4, _p2vs_location5);
    VertexOutput out_ = VertexOutput(0u, vec2(0.0), vec2(0.0), 0u, 0u, 0u, vec4(0.0));
    uint height = 0u;
    bool local = false;
    bool local_1 = false;
    bool local_2 = false;
    out_.sample_xy = vec2(0.0);
    float x_3 = float((in_vertex_index & 1u));
    float y_3 = float((in_vertex_index >> 1u));
    uint x0_1 = (instance.xy & 65535u);
    uint y0_1 = (instance.xy >> 16u);
    uint width = (instance.widths_or_rect_height & 65535u);
    uint dense_width = (instance.widths_or_rect_height >> 16u);
    bool is_rect = ((instance.paint_and_rect_flag & RECT_STRIP_FLAG) != 0u);
    uint _e31 = _group_0_binding_1_vs.strip_height;
    height = _e31;
    if (is_rect) {
        height = dense_width;
        out_.dense_end_or_rect_size = (width | (dense_width << 16u));
        out_.rect_frac = instance.col_idx_or_rect_frac;
    } else {
        out_.dense_end_or_rect_size = (instance.col_idx_or_rect_frac + dense_width);
        out_.rect_frac = 0u;
    }
    int _e47 = _group_0_binding_1_vs.strip_offset_x;
    float pix_x = (float((int(x0_1) + _e47)) + (x_3 * float(width)));
    int _e56 = _group_0_binding_1_vs.strip_offset_y;
    uint _e59 = height;
    float pix_y = (float((int(y0_1) + _e56)) + (y_3 * float(_e59)));
    uint _e67 = _group_0_binding_1_vs.width;
    float ndc_x = (((pix_x * 2.0) / float(_e67)) - 1.0);
    uint _e76 = _group_0_binding_1_vs.height;
    float ndc_y = (1.0 - ((pix_y * 2.0) / float(_e76)));
    uint color_source = ((instance.paint_and_rect_flag >> 29u) & 3u);
    if ((color_source == COLOR_SOURCE_PAYLOAD)) {
        uint paint_type = ((instance.paint_and_rect_flag >> 26u) & 7u);
        uint scene_strip_x = (instance.payload & 65535u);
        uint scene_strip_y = (instance.payload >> 16u);
        if ((paint_type == PAINT_TYPE_IMAGE)) {
            uint paint_tex_idx_1 = (instance.paint_and_rect_flag & PAINT_TEXTURE_INDEX_MASK);
            uvec4 _e105 = load_encoded_paint_texel(paint_tex_idx_1, 0u);
            uvec4 _e107 = load_encoded_paint_texel(paint_tex_idx_1, 1u);
            uvec4 _e109 = load_encoded_paint_texel(paint_tex_idx_1, 2u);
            uint _e115 = height;
            vec2 pos = vec2((float(scene_strip_x) + (x_3 * float(width))), (float(scene_strip_y) + (y_3 * float(_e115))));
            vec2 _e121 = get_image_translate(_e107, _e109);
            vec2 _e122 = get_image_offset(_e105);
            mat2x2 _e124 = get_image_transform(_e105, _e107);
            out_.sample_xy = ((_e121 + _e122) + (_e124 * pos));
        } else {
            if (!((paint_type == PAINT_TYPE_LINEAR_GRADIENT))) {
                local = (paint_type == PAINT_TYPE_RADIAL_GRADIENT);
            } else {
                local = true;
            }
            bool _e135 = local;
            if (!(_e135)) {
                local_1 = (paint_type == PAINT_TYPE_SWEEP_GRADIENT);
            } else {
                local_1 = true;
            }
            bool _e142 = local_1;
            if (!(_e142)) {
                local_2 = (paint_type == PAINT_TYPE_BLURRED_ROUNDED_RECT);
            } else {
                local_2 = true;
            }
            bool _e149 = local_2;
            if (_e149) {
                uint _e156 = height;
                out_.sample_xy = vec2((float(scene_strip_x) + (x_3 * float(width))), (float(scene_strip_y) + (y_3 * float(_e156))));
            }
        }
    } else {
        if ((color_source == COLOR_SOURCE_LAYER)) {
            uint src_x = (instance.payload & 65535u);
            uint src_y = (instance.payload >> 16u);
            uint _e175 = height;
            out_.sample_xy = vec2((float(src_x) + (x_3 * float(width))), (float(src_y) + (y_3 * float(_e175))));
        }
    }
    float col_offset = (is_rect ? 0.0 : float(instance.col_idx_or_rect_frac));
    uint _e188 = height;
    out_.tex_coord = vec2((col_offset + (x_3 * float(width))), (y_3 * float(_e188)));
    float z_1 = (1.0 - (float(instance.depth_index) / 16777216.0));
    uint _e201 = _group_0_binding_1_vs.ndc_y_negate;
    float final_ndc_y = ((_e201 != 0u) ? -(ndc_y) : ndc_y);
    out_.position = vec4(ndc_x, final_ndc_y, z_1, 1.0);
    out_.payload = instance.payload;
    out_.paint_and_rect_flag = instance.paint_and_rect_flag;
    VertexOutput _e212 = out_;
    _vs2fs_location0 = _e212.paint_and_rect_flag;
    _vs2fs_location1 = _e212.tex_coord;
    _vs2fs_location2 = _e212.sample_xy;
    _vs2fs_location3 = _e212.dense_end_or_rect_size;
    _vs2fs_location4 = _e212.payload;
    _vs2fs_location5 = _e212.rect_frac;
    gl_Position = _e212.position;
    gl_Position.yz = vec2(-gl_Position.y, gl_Position.z * 2.0 - gl_Position.w);
    return;
}

"###;

    pub mod vertex {
        pub const CONFIG: &str = "Config_block_0Vertex";
        pub const ENCODED_PAINTS_TEXTURE: &str = "_group_2_binding_0_vs";
    }
    pub const FRAGMENT_SOURCE: &str = r###"#version 300 es

precision highp float;
precision highp int;

struct Config {
    uint width;
    uint height;
    uint strip_height;
    uint alphas_tex_width_bits;
    uint encoded_paints_tex_width_bits;
    int strip_offset_x;
    int strip_offset_y;
    uint ndc_y_negate;
};
struct StripInstance {
    uint xy;
    uint widths_or_rect_height;
    uint col_idx_or_rect_frac;
    uint payload;
    uint paint_and_rect_flag;
    uint depth_index;
};
struct VertexOutput {
    uint paint_and_rect_flag;
    vec2 tex_coord;
    vec2 sample_xy;
    uint dense_end_or_rect_size;
    uint payload;
    uint rect_frac;
    vec4 position;
};
const uint COLOR_SOURCE_PAYLOAD = 0u;
const uint COLOR_SOURCE_LAYER = 1u;
const uint PAINT_TYPE_SOLID = 0u;
const uint PAINT_TYPE_IMAGE = 1u;
const uint PAINT_TYPE_LINEAR_GRADIENT = 2u;
const uint PAINT_TYPE_RADIAL_GRADIENT = 3u;
const uint PAINT_TYPE_SWEEP_GRADIENT = 4u;
const uint PAINT_TYPE_BLURRED_ROUNDED_RECT = 5u;
const uint PAINT_TEXTURE_INDEX_MASK = 67108863u;
const uint RECT_STRIP_FLAG = 2147483648u;
const uint IMAGE_QUALITY_LOW = 0u;
const uint IMAGE_QUALITY_MEDIUM = 1u;
const uint IMAGE_QUALITY_HIGH = 2u;
const uint IMAGE_SOURCE_ATLAS = 0u;
const uint IMAGE_SOURCE_EXTERNAL = 1u;
const uint GRADIENT_TYPE_LINEAR = 0u;
const uint GRADIENT_TYPE_RADIAL = 1u;
const uint GRADIENT_TYPE_SWEEP = 2u;
const uint RADIAL_GRADIENT_TYPE_STANDARD = 0u;
const uint RADIAL_GRADIENT_TYPE_STRIP = 1u;
const uint RADIAL_GRADIENT_TYPE_FOCAL = 2u;
const float PI = 3.1415927;
const float TWO_PI = 6.2831855;
const float NEARLY_ZERO_TOLERANCE = 0.00024414063;
const uint TINT_MODE_MULTIPLY = 1u;
const uint EXTEND_PAD = 0u;
const uint EXTEND_REPEAT = 1u;
const uint EXTEND_REFLECT = 2u;
const vec4 MF[4] = vec4[4](vec4(0.055555556, -0.5, 0.8333333, -0.3888889), vec4(0.8888889, 0.0, -2.0, 1.1666666), vec4(0.055555556, 0.5, 1.5, -1.1666666), vec4(0.0, 0.0, -0.33333334, 0.3888889));
const uint TINT_MODE_ALPHA_MASK = 0u;

layout(std140) uniform Config_block_0Fragment { Config _group_0_binding_1_fs; };

uniform highp sampler2DArray _group_1_binding_0_fs;

uniform highp sampler2D _group_1_binding_1_fs;

uniform highp usampler2D _group_2_binding_0_fs;

uniform highp sampler2D _group_3_binding_0_fs;

uniform highp usampler2D _group_0_binding_0_fs;

uniform highp sampler2D _group_0_binding_2_fs;

flat in uint _vs2fs_location0;
smooth in vec2 _vs2fs_location1;
smooth in vec2 _vs2fs_location2;
flat in uint _vs2fs_location3;
flat in uint _vs2fs_location4;
flat in uint _vs2fs_location5;
layout(location = 0) out vec4 _fs2p_location0;

uvec2 encoded_paint_coord(uint flat_idx) {
    uint _e4 = _group_0_binding_1_fs.encoded_paints_tex_width_bits;
    uint _e11 = _group_0_binding_1_fs.encoded_paints_tex_width_bits;
    return uvec2((flat_idx & ((1u << _e4) - 1u)), (flat_idx >> _e11));
}

uvec4 load_encoded_paint_texel(uint paint_tex_idx, uint texel_offset) {
    uvec2 _e4 = encoded_paint_coord((paint_tex_idx + texel_offset));
    uvec4 _e6 = texelFetch(_group_2_binding_0_fs, ivec2(_e4), 0);
    return _e6;
}

vec2 get_image_translate(uvec4 texel1_, uvec4 texel2_) {
    return vec2(uintBitsToFloat(texel1_.w), uintBitsToFloat(texel2_.x));
}

vec2 get_image_offset(uvec4 texel0_) {
    return vec2(float((texel0_.z >> 16u)), float((texel0_.z & 65535u)));
}

mat2x2 get_image_transform(uvec4 texel0_1, uvec4 texel1_1) {
    return mat2x2(vec2(uintBitsToFloat(texel0_1.w), uintBitsToFloat(texel1_1.x)), vec2(uintBitsToFloat(texel1_1.y), uintBitsToFloat(texel1_1.z)));
}

uint unpack_alphas_from_channel(uvec4 rgba, uint channel_index) {
    switch(channel_index) {
        case 0u: {
            return rgba.x;
        }
        case 1u: {
            return rgba.y;
        }
        case 2u: {
            return rgba.z;
        }
        case 3u: {
            return rgba.w;
        }
        default: {
            return rgba.x;
        }
    }
}

vec2 get_image_size(uvec4 texel0_2) {
    return vec2(float((texel0_2.y >> 16u)), float((texel0_2.y & 65535u)));
}

uvec2 get_image_extend_modes(uvec4 texel0_3) {
    return uvec2(((texel0_3.x >> 2u) & 3u), ((texel0_3.x >> 4u) & 3u));
}

uint get_image_atlas_index(uvec4 texel0_4) {
    return ((texel0_4.x >> 6u) & 255u);
}

uint get_image_quality(uvec4 texel0_5) {
    return (texel0_5.x & 3u);
}

uint get_image_source_kind(uvec4 texel0_6) {
    return ((texel0_6.x >> 14u) & 1u);
}

float get_image_padding(uvec4 texel2_1) {
    return float(texel2_1.w);
}

float extend_mode_normalized(float t, uint mode) {
    switch(mode) {
        case 0u: {
            return clamp(t, 0.0, 1.0);
        }
        case 1u: {
            return fract(t);
        }
        case 2u:
        default: {
            return abs((t - (2.0 * roundEven((0.5 * t)))));
        }
    }
}

float extend_mode(float t_1, uint mode_1, float max_) {
    switch(mode_1) {
        case 0u: {
            return clamp(t_1, 0.5, (max_ - 0.5));
        }
        case 1u: {
            float _e8 = extend_mode_normalized((t_1 / max_), mode_1);
            return (_e8 * max_);
        }
        case 2u:
        default: {
            float _e11 = extend_mode_normalized((t_1 / max_), mode_1);
            return (_e11 * max_);
        }
    }
}

float single_weight(float t_2, float a, float b, float c, float d) {
    return ((t_2 * ((t_2 * ((t_2 * d) + c)) + b)) + a);
}

vec4 cubic_weights(float fract_) {
    float _e5 = single_weight(fract_, 0.055555556, -0.5, 0.8333333, -0.3888889);
    float _e10 = single_weight(fract_, 0.8888889, 0.0, -2.0, 1.1666666);
    float _e15 = single_weight(fract_, 0.055555556, 0.5, 1.5, -1.1666666);
    float _e20 = single_weight(fract_, 0.0, 0.0, -0.33333334, 0.3888889);
    return vec4(_e5, _e10, _e15, _e20);
}

vec4 external_bicubic_sample(vec2 coords, vec2 image_offset, vec2 image_size) {
    vec2 atlas_max = ((image_offset + image_size) - vec2(1.0));
    vec2 frac_coords = fract((coords + vec2(0.5)));
    vec4 _e12 = cubic_weights(frac_coords.x);
    vec4 _e14 = cubic_weights(frac_coords.y);
    vec4 s00_ = texelFetch(_group_1_binding_1_fs, ivec2(clamp((coords + vec2(-1.5, -1.5)), image_offset, atlas_max)), 0);
    vec4 s10_ = texelFetch(_group_1_binding_1_fs, ivec2(clamp((coords + vec2(-0.5, -1.5)), image_offset, atlas_max)), 0);
    vec4 s20_ = texelFetch(_group_1_binding_1_fs, ivec2(clamp((coords + vec2(0.5, -1.5)), image_offset, atlas_max)), 0);
    vec4 s30_ = texelFetch(_group_1_binding_1_fs, ivec2(clamp((coords + vec2(1.5, -1.5)), image_offset, atlas_max)), 0);
    vec4 s01_ = texelFetch(_group_1_binding_1_fs, ivec2(clamp((coords + vec2(-1.5, -0.5)), image_offset, atlas_max)), 0);
    vec4 s11_ = texelFetch(_group_1_binding_1_fs, ivec2(clamp((coords + vec2(-0.5, -0.5)), image_offset, atlas_max)), 0);
    vec4 s21_ = texelFetch(_group_1_binding_1_fs, ivec2(clamp((coords + vec2(0.5, -0.5)), image_offset, atlas_max)), 0);
    vec4 s31_ = texelFetch(_group_1_binding_1_fs, ivec2(clamp((coords + vec2(1.5, -0.5)), image_offset, atlas_max)), 0);
    vec4 s02_ = texelFetch(_group_1_binding_1_fs, ivec2(clamp((coords + vec2(-1.5, 0.5)), image_offset, atlas_max)), 0);
    vec4 s12_ = texelFetch(_group_1_binding_1_fs, ivec2(clamp((coords + vec2(-0.5, 0.5)), image_offset, atlas_max)), 0);
    vec4 s22_ = texelFetch(_group_1_binding_1_fs, ivec2(clamp((coords + vec2(0.5, 0.5)), image_offset, atlas_max)), 0);
    vec4 s32_ = texelFetch(_group_1_binding_1_fs, ivec2(clamp((coords + vec2(1.5, 0.5)), image_offset, atlas_max)), 0);
    vec4 s03_ = texelFetch(_group_1_binding_1_fs, ivec2(clamp((coords + vec2(-1.5, 1.5)), image_offset, atlas_max)), 0);
    vec4 s13_ = texelFetch(_group_1_binding_1_fs, ivec2(clamp((coords + vec2(-0.5, 1.5)), image_offset, atlas_max)), 0);
    vec4 s23_ = texelFetch(_group_1_binding_1_fs, ivec2(clamp((coords + vec2(0.5, 1.5)), image_offset, atlas_max)), 0);
    vec4 s33_ = texelFetch(_group_1_binding_1_fs, ivec2(clamp((coords + vec2(1.5, 1.5)), image_offset, atlas_max)), 0);
    vec4 row0_ = ((((_e12.x * s00_) + (_e12.y * s10_)) + (_e12.z * s20_)) + (_e12.w * s30_));
    vec4 row1_ = ((((_e12.x * s01_) + (_e12.y * s11_)) + (_e12.z * s21_)) + (_e12.w * s31_));
    vec4 row2_ = ((((_e12.x * s02_) + (_e12.y * s12_)) + (_e12.z * s22_)) + (_e12.w * s32_));
    vec4 row3_ = ((((_e12.x * s03_) + (_e12.y * s13_)) + (_e12.z * s23_)) + (_e12.w * s33_));
    vec4 result = ((((_e14.x * row0_) + (_e14.y * row1_)) + (_e14.z * row2_)) + (_e14.w * row3_));
    float a_1 = clamp(result.w, 0.0, 1.0);
    return vec4(clamp(result.xyz, vec3(0.0), vec3(a_1)), a_1);
}

vec4 external_bilinear_sample(vec2 coords_1, vec2 image_offset_1, vec2 image_size_1) {
    vec2 atlas_max_1 = ((image_offset_1 + image_size_1) - vec2(1.0));
    vec2 atlas_uv_clamped = clamp(coords_1, image_offset_1, atlas_max_1);
    vec4 uv_quad = vec4(floor(atlas_uv_clamped), ceil(atlas_uv_clamped));
    vec2 uv_frac = fract(coords_1);
    vec4 a_2 = texelFetch(_group_1_binding_1_fs, ivec2(uv_quad.xy), 0);
    vec4 b_1 = texelFetch(_group_1_binding_1_fs, ivec2(uv_quad.xw), 0);
    vec4 c_1 = texelFetch(_group_1_binding_1_fs, ivec2(uv_quad.zy), 0);
    vec4 d_1 = texelFetch(_group_1_binding_1_fs, ivec2(uv_quad.zw), 0);
    return mix(mix(a_2, b_1, uv_frac.y), mix(c_1, d_1, uv_frac.y), uv_frac.x);
}

vec4 sample_external_image(uint quality, vec2 coords_2, vec2 image_offset_2, vec2 image_size_2) {
    if ((quality == IMAGE_QUALITY_HIGH)) {
        vec4 _e6 = external_bicubic_sample(coords_2, image_offset_2, image_size_2);
        return _e6;
    }
    if ((quality == IMAGE_QUALITY_MEDIUM)) {
        vec4 _e12 = external_bilinear_sample((coords_2 - vec2(0.5)), image_offset_2, image_size_2);
        return _e12;
    }
    vec4 _e16 = texelFetch(_group_1_binding_1_fs, ivec2(uvec2(coords_2)), 0);
    return _e16;
}

vec4 bicubic_sample(highp sampler2DArray tex, vec2 coords_3, int atlas_idx, vec2 image_offset_3, vec2 image_size_3, uvec2 _extend_modes, float _image_padding) {
    vec2 atlas_max_2 = ((image_offset_3 + image_size_3) - vec2(1.0));
    vec2 frac_coords_1 = fract((coords_3 + vec2(0.5)));
    vec4 _e16 = cubic_weights(frac_coords_1.x);
    vec4 _e18 = cubic_weights(frac_coords_1.y);
    vec4 s00_1 = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(-1.5, -1.5)), image_offset_3, atlas_max_2)), atlas_idx), 0);
    vec4 s10_1 = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(-0.5, -1.5)), image_offset_3, atlas_max_2)), atlas_idx), 0);
    vec4 s20_1 = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(0.5, -1.5)), image_offset_3, atlas_max_2)), atlas_idx), 0);
    vec4 s30_1 = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(1.5, -1.5)), image_offset_3, atlas_max_2)), atlas_idx), 0);
    vec4 s01_1 = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(-1.5, -0.5)), image_offset_3, atlas_max_2)), atlas_idx), 0);
    vec4 s11_1 = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(-0.5, -0.5)), image_offset_3, atlas_max_2)), atlas_idx), 0);
    vec4 s21_1 = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(0.5, -0.5)), image_offset_3, atlas_max_2)), atlas_idx), 0);
    vec4 s31_1 = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(1.5, -0.5)), image_offset_3, atlas_max_2)), atlas_idx), 0);
    vec4 s02_1 = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(-1.5, 0.5)), image_offset_3, atlas_max_2)), atlas_idx), 0);
    vec4 s12_1 = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(-0.5, 0.5)), image_offset_3, atlas_max_2)), atlas_idx), 0);
    vec4 s22_1 = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(0.5, 0.5)), image_offset_3, atlas_max_2)), atlas_idx), 0);
    vec4 s32_1 = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(1.5, 0.5)), image_offset_3, atlas_max_2)), atlas_idx), 0);
    vec4 s03_1 = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(-1.5, 1.5)), image_offset_3, atlas_max_2)), atlas_idx), 0);
    vec4 s13_1 = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(-0.5, 1.5)), image_offset_3, atlas_max_2)), atlas_idx), 0);
    vec4 s23_1 = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(0.5, 1.5)), image_offset_3, atlas_max_2)), atlas_idx), 0);
    vec4 s33_1 = texelFetch(tex, ivec3(ivec2(clamp((coords_3 + vec2(1.5, 1.5)), image_offset_3, atlas_max_2)), atlas_idx), 0);
    vec4 row0_1 = ((((_e16.x * s00_1) + (_e16.y * s10_1)) + (_e16.z * s20_1)) + (_e16.w * s30_1));
    vec4 row1_1 = ((((_e16.x * s01_1) + (_e16.y * s11_1)) + (_e16.z * s21_1)) + (_e16.w * s31_1));
    vec4 row2_1 = ((((_e16.x * s02_1) + (_e16.y * s12_1)) + (_e16.z * s22_1)) + (_e16.w * s32_1));
    vec4 row3_1 = ((((_e16.x * s03_1) + (_e16.y * s13_1)) + (_e16.z * s23_1)) + (_e16.w * s33_1));
    vec4 result_1 = ((((_e18.x * row0_1) + (_e18.y * row1_1)) + (_e18.z * row2_1)) + (_e18.w * row3_1));
    float a_3 = clamp(result_1.w, 0.0, 1.0);
    return vec4(clamp(result_1.xyz, vec3(0.0), vec3(a_3)), a_3);
}

vec4 bilinear_sample(highp sampler2DArray tex_1, vec2 coords_4, int atlas_idx_1, vec2 image_offset_4, vec2 image_size_4, uvec2 _extend_modes_1, float _image_padding_1) {
    vec2 atlas_max_3 = ((image_offset_4 + image_size_4) - vec2(1.0));
    vec2 atlas_uv_clamped_1 = clamp(coords_4, image_offset_4, atlas_max_3);
    vec4 uv_quad_1 = vec4(floor(atlas_uv_clamped_1), ceil(atlas_uv_clamped_1));
    vec2 uv_frac_1 = fract(coords_4);
    vec4 a_4 = texelFetch(tex_1, ivec3(ivec2(uv_quad_1.xy), atlas_idx_1), 0);
    vec4 b_2 = texelFetch(tex_1, ivec3(ivec2(uv_quad_1.xw), atlas_idx_1), 0);
    vec4 c_2 = texelFetch(tex_1, ivec3(ivec2(uv_quad_1.zy), atlas_idx_1), 0);
    vec4 d_2 = texelFetch(tex_1, ivec3(ivec2(uv_quad_1.zw), atlas_idx_1), 0);
    return mix(mix(a_4, b_2, uv_frac_1.y), mix(c_2, d_2, uv_frac_1.y), uv_frac_1.x);
}

mat2x2 get_gradient_transform(uvec4 texel0_7, uvec4 texel1_2) {
    return mat2x2(vec2(uintBitsToFloat(texel0_7.z), uintBitsToFloat(texel0_7.w)), vec2(uintBitsToFloat(texel1_2.x), uintBitsToFloat(texel1_2.y)));
}

vec2 get_gradient_translate(uvec4 texel1_3) {
    return vec2(uintBitsToFloat(texel1_3.z), uintBitsToFloat(texel1_3.w));
}

vec2 apply_gradient_transform(uvec4 texel0_8, uvec4 texel1_4, vec2 fragment_pos) {
    mat2x2 _e3 = get_gradient_transform(texel0_8, texel1_4);
    vec2 _e5 = get_gradient_translate(texel1_4);
    return ((_e3 * fragment_pos) + _e5);
}

vec4 sample_gradient_lut(float t_value, uint extend_mode_1, uint gradient_start, uint texture_width) {
    float _e4 = extend_mode_normalized(t_value, extend_mode_1);
    uint t_offset = uint((_e4 * float((texture_width - 1u))));
    uint flat_coord = (gradient_start + t_offset);
    uint gradient_tex_width = uvec2(textureSize(_group_3_binding_0_fs, 0).xy).x;
    uint tex_x = (flat_coord % gradient_tex_width);
    uint tex_y = (flat_coord / gradient_tex_width);
    vec4 gradient_color = texelFetch(_group_3_binding_0_fs, ivec2(uvec2(tex_x, tex_y)), 0);
    return gradient_color;
}

uint get_gradient_extend_mode(uvec4 texel0_9) {
    return ((texel0_9.x >> 30u) & 3u);
}

uint get_gradient_start(uvec4 texel0_10) {
    return texel0_10.y;
}

uint get_gradient_texture_width(uvec4 texel0_11) {
    return (texel0_11.x & 268435455u);
}

uint get_radial_kind(uvec4 texel2_2) {
    return (texel2_2.x & 3u);
}

float get_radial_bias(uvec4 texel2_3) {
    return uintBitsToFloat(texel2_3.y);
}

float get_radial_scale(uvec4 texel2_4) {
    return uintBitsToFloat(texel2_4.z);
}

float get_radial_scaled_r0_squared(uvec4 texel3_) {
    return uintBitsToFloat(texel3_.w);
}

float get_radial_fp0_(uvec4 texel2_5) {
    return uintBitsToFloat(texel2_5.w);
}

float get_radial_fp1_(uvec4 texel3_1) {
    return uintBitsToFloat(texel3_1.x);
}

float get_radial_fr1_(uvec4 texel3_2) {
    return uintBitsToFloat(texel3_2.y);
}

float get_radial_f_focal_x(uvec4 texel3_3) {
    return uintBitsToFloat(texel3_3.z);
}

uint get_radial_f_is_swapped(uvec4 texel2_6) {
    return ((texel2_6.x >> 2u) & 1u);
}

vec2 calculate_radial_gradient(vec2 grad_pos_1, uvec4 texel2_7, uvec4 texel3_4) {
    float t_value_1 = 0.0;
    bool is_valid = false;
    float t_3 = 0.0;
    bool local_5 = false;
    bool local_6 = false;
    bool local_7 = false;
    bool local_8 = false;
    float x_pos = grad_pos_1.x;
    float y_pos = grad_pos_1.y;
    uint _e7 = get_radial_kind(texel2_7);
    switch(_e7) {
        case 0u: {
            float radius = sqrt(((x_pos * x_pos) + (y_pos * y_pos)));
            float _e12 = get_radial_bias(texel2_7);
            float _e13 = get_radial_scale(texel2_7);
            t_value_1 = (_e12 + (_e13 * radius));
            is_valid = true;
            break;
        }
        case 1u: {
            float _e17 = get_radial_scaled_r0_squared(texel3_4);
            float p1_ = (_e17 - (y_pos * y_pos));
            is_valid = (p1_ >= 0.0);
            bool _e22 = is_valid;
            if (_e22) {
                t_value_1 = (x_pos + sqrt(p1_));
            } else {
                t_value_1 = 0.0;
            }
            break;
        }
        case 2u:
        default: {
            float _e28 = get_radial_fp0_(texel2_7);
            float _e29 = get_radial_fp1_(texel3_4);
            float _e30 = get_radial_fr1_(texel3_4);
            float _e31 = get_radial_f_focal_x(texel3_4);
            uint _e32 = get_radial_f_is_swapped(texel2_7);
            bool is_focal_on_circle = (abs((1.0 - _e30)) <= NEARLY_ZERO_TOLERANCE);
            if (!(is_focal_on_circle)) {
                local_5 = (_e30 > 1.0);
            } else {
                local_5 = false;
            }
            bool is_well_behaved = local_5;
            bool is_natively_focal = (abs(_e31) <= NEARLY_ZERO_TOLERANCE);
            is_valid = true;
            if (is_focal_on_circle) {
                t_3 = (x_pos + ((y_pos * y_pos) / x_pos));
                float _e52 = t_3;
                if ((_e52 >= 0.0)) {
                    local_6 = (x_pos != 0.0);
                } else {
                    local_6 = false;
                }
                bool _e60 = local_6;
                is_valid = _e60;
            } else {
                if (is_well_behaved) {
                    t_3 = (sqrt(((x_pos * x_pos) + (y_pos * y_pos))) - (x_pos * _e28));
                } else {
                    float xx = (x_pos * x_pos);
                    float yy = (y_pos * y_pos);
                    float discriminant = (xx - yy);
                    if (!((_e32 != 0u))) {
                        local_7 = ((1.0 - _e31) < 0.0);
                    } else {
                        local_7 = true;
                    }
                    bool _e80 = local_7;
                    if (_e80) {
                        t_3 = (-(sqrt(discriminant)) - (x_pos * _e28));
                    } else {
                        t_3 = (sqrt(discriminant) - (x_pos * _e28));
                    }
                    if ((discriminant >= 0.0)) {
                        float _e92 = t_3;
                        local_8 = (_e92 >= 0.0);
                    } else {
                        local_8 = false;
                    }
                    bool _e96 = local_8;
                    is_valid = _e96;
                }
            }
            bool _e97 = is_valid;
            if (_e97) {
                if (((1.0 - _e31) < 0.0)) {
                    float _e102 = t_3;
                    t_3 = -(_e102);
                }
                if (!(is_natively_focal)) {
                    float _e105 = t_3;
                    t_3 = (_e105 + _e29);
                }
                if ((_e32 != 0u)) {
                    float _e109 = t_3;
                    t_3 = (1.0 - _e109);
                }
            }
            float _e112 = t_3;
            t_value_1 = _e112;
            break;
        }
    }
    float _e113 = t_value_1;
    bool _e114 = is_valid;
    return vec2(_e113, (_e114 ? 1.0 : 0.0));
}

float xy_to_unit_angle(float x, float y) {
    float phi = 0.0;
    float xabs = abs(x);
    float yabs = abs(y);
    float slope = (min(xabs, yabs) / max(xabs, yabs));
    float s = (slope * slope);
    phi = (slope * (0.15912117 + (s * (-0.05185397 + (s * (0.02476102 + (s * -0.0070547382)))))));
    float _e20 = phi;
    float _e21 = phi;
    phi = ((xabs < yabs) ? (0.25 - _e21) : _e20);
    float _e26 = phi;
    float _e27 = phi;
    phi = ((x < 0.0) ? (0.5 - _e27) : _e26);
    float _e33 = phi;
    float _e34 = phi;
    phi = ((y < 0.0) ? (1.0 - _e34) : _e33);
    float _e40 = phi;
    float _e41 = phi;
    float _e42 = phi;
    phi = ((_e41 != _e42) ? 0.0 : _e40);
    float _e46 = phi;
    return _e46;
}

float get_sweep_start_angle(uvec4 texel2_8) {
    return uintBitsToFloat(texel2_8.x);
}

float get_sweep_inv_angle_delta(uvec4 texel2_9) {
    return uintBitsToFloat(texel2_9.y);
}

mat2x2 get_blurred_rounded_rect_transform(uvec4 texel0_12) {
    return mat2x2(vec2(uintBitsToFloat(texel0_12.x), uintBitsToFloat(texel0_12.y)), vec2(uintBitsToFloat(texel0_12.z), uintBitsToFloat(texel0_12.w)));
}

vec2 get_blurred_rounded_rect_translate(uvec4 texel1_5) {
    return vec2(uintBitsToFloat(texel1_5.x), uintBitsToFloat(texel1_5.y));
}

vec4 get_blurred_rounded_rect_color(uvec4 texel1_6) {
    uint _e1 = texel1_6.z;
    return (vec4(_e1 & 0xFFu, _e1 >> 8 & 0xFFu, _e1 >> 16 & 0xFFu, _e1 >> 24) / 255.0);
}

uint get_blurred_rounded_rect_invert(uvec4 texel1_7) {
    return texel1_7.w;
}

float get_blurred_rounded_rect_exponent(uvec4 texel2_10) {
    return uintBitsToFloat(texel2_10.x);
}

float get_blurred_rounded_rect_recip_exponent(uvec4 texel2_11) {
    return uintBitsToFloat(texel2_11.y);
}

float get_blurred_rounded_rect_scale(uvec4 texel2_12) {
    return uintBitsToFloat(texel2_12.z);
}

float get_blurred_rounded_rect_std_dev_inv(uvec4 texel2_13) {
    return uintBitsToFloat(texel2_13.w);
}

float get_blurred_rounded_rect_min_edge(uvec4 texel3_5) {
    return uintBitsToFloat(texel3_5.x);
}

float get_blurred_rounded_rect_w(uvec4 texel3_6) {
    return uintBitsToFloat(texel3_6.y);
}

float get_blurred_rounded_rect_h(uvec4 texel3_7) {
    return uintBitsToFloat(texel3_7.z);
}

float get_blurred_rounded_rect_r1_(uvec4 texel3_8) {
    return uintBitsToFloat(texel3_8.w);
}

float get_blurred_rounded_rect_width(uvec4 texel4_) {
    return uintBitsToFloat(texel4_.x);
}

float get_blurred_rounded_rect_height(uvec4 texel4_1) {
    return uintBitsToFloat(texel4_1.y);
}

float erf7_(float x_1) {
    float y_1 = clamp((x_1 * 1.1283792), -100.0, 100.0);
    float yy_1 = (y_1 * y_1);
    float z = (y_1 + ((0.24295 + ((0.03395 + (0.0104 * yy_1)) * yy_1)) * (y_1 * yy_1)));
    return (z / sqrt((1.0 + (z * z))));
}

vec4 calculate_blurred_rounded_rect(vec2 fragment_pos_1, uvec4 texel0_13, uvec4 texel1_8, uvec4 texel2_14, uvec4 texel3_9, uvec4 texel4_2) {
    mat2x2 _e6 = get_blurred_rounded_rect_transform(texel0_13);
    vec2 _e7 = get_blurred_rounded_rect_translate(texel1_8);
    vec4 _e8 = get_blurred_rounded_rect_color(texel1_8);
    uint _e9 = get_blurred_rounded_rect_invert(texel1_8);
    float _e10 = get_blurred_rounded_rect_exponent(texel2_14);
    float _e11 = get_blurred_rounded_rect_recip_exponent(texel2_14);
    float _e12 = get_blurred_rounded_rect_scale(texel2_14);
    float _e13 = get_blurred_rounded_rect_std_dev_inv(texel2_14);
    float _e14 = get_blurred_rounded_rect_min_edge(texel3_9);
    float _e15 = get_blurred_rounded_rect_w(texel3_9);
    float _e16 = get_blurred_rounded_rect_h(texel3_9);
    float _e17 = get_blurred_rounded_rect_r1_(texel3_9);
    float _e18 = get_blurred_rounded_rect_width(texel4_2);
    float _e19 = get_blurred_rounded_rect_height(texel4_2);
    vec2 local_xy = ((_e6 * fragment_pos_1) + _e7);
    float y_2 = (local_xy.y - (0.5 * _e19));
    float y0_ = ((_e17 + abs(y_2)) - (0.5 * _e16));
    float y1_ = max(y0_, 0.0);
    float x_2 = (local_xy.x - (0.5 * _e18));
    float x0_ = ((_e17 + abs(x_2)) - (0.5 * _e15));
    float x1_ = max(x0_, 0.0);
    float d_pos = pow((pow(x1_, _e10) + pow(y1_, _e10)), _e11);
    float d_neg = min(max(x0_, y0_), 0.0);
    float d_3 = ((d_pos + d_neg) - _e17);
    float _e55 = erf7_((_e13 * (_e14 + d_3)));
    float _e57 = erf7_((_e13 * d_3));
    float blur_coverage = (_e12 * (_e55 - _e57));
    float blur_alpha = ((_e9 != 0u) ? (1.0 - blur_coverage) : blur_coverage);
    return (_e8 * blur_alpha);
}

void main() {
    uint paint_and_rect_flag = _vs2fs_location0;
    vec2 tex_coord = _vs2fs_location1;
    vec2 sample_xy = _vs2fs_location2;
    uint dense_end_or_rect_size = _vs2fs_location3;
    uint payload = _vs2fs_location4;
    uint rect_frac = _vs2fs_location5;
    vec4 position = gl_FragCoord;
    float alpha = 1.0;
    bool local_3 = false;
    bool local_4 = false;
    vec4 final_color = vec4(0.0);
    vec4 sample_color = vec4(0.0);
    vec2 grad_pos = vec2(0.0);
    bool is_rect = ((paint_and_rect_flag & RECT_STRIP_FLAG) != 0u);
    if (is_rect) {
        local_3 = (rect_frac != 0u);
    } else {
        local_3 = false;
    }
    bool _e18 = local_3;
    if (_e18) {
        vec4 frac = (vec4(rect_frac & 0xFFu, rect_frac >> 8 & 0xFFu, rect_frac >> 16 & 0xFFu, rect_frac >> 24) / 255.0);
        vec2 rect_size = vec2(float((dense_end_or_rect_size & 65535u)), float((dense_end_or_rect_size >> 16u)));
        vec2 bottom_and_right = min((tex_coord + vec2(0.5)), (rect_size - frac.zw));
        vec2 top_and_left = max((tex_coord - vec2(0.5)), frac.xy);
        vec2 a_5 = clamp((bottom_and_right - top_and_left), vec2(0.0), vec2(1.0));
        alpha = (a_5.x * a_5.y);
    } else {
        if (!(is_rect)) {
            local_4 = (dense_end_or_rect_size != 0u);
        } else {
            local_4 = false;
        }
        bool _e53 = local_4;
        if (_e53) {
            uint alphas_index = uint(floor(tex_coord.x));
            uint y_3 = uint(floor(tex_coord.y));
            uvec2 tex_dimensions = uvec2(textureSize(_group_0_binding_0_fs, 0).xy);
            uint alphas_tex_width = tex_dimensions.x;
            uint texel_index = (alphas_index / 4u);
            uint channel_index_1 = (alphas_index % 4u);
            uint tex_x_1 = (texel_index & (alphas_tex_width - 1u));
            uint _e72 = _group_0_binding_1_fs.alphas_tex_width_bits;
            uint tex_y_1 = (texel_index >> _e72);
            uvec4 rgba_values = texelFetch(_group_0_binding_0_fs, ivec2(uvec2(tex_x_1, tex_y_1)), 0);
            uint _e78 = unpack_alphas_from_channel(rgba_values, channel_index_1);
            alpha = (float(((_e78 >> (y_3 * 8u)) & 255u)) * 0.003921569);
        }
    }
    uint color_source = ((paint_and_rect_flag >> 29u) & 3u);
    if ((color_source == COLOR_SOURCE_PAYLOAD)) {
        uint paint_type = ((paint_and_rect_flag >> 26u) & 7u);
        if ((paint_type == PAINT_TYPE_SOLID)) {
            float _e100 = alpha;
            final_color = (_e100 * (vec4(payload & 0xFFu, payload >> 8 & 0xFFu, payload >> 16 & 0xFFu, payload >> 24) / 255.0));
        } else {
            if ((paint_type == PAINT_TYPE_IMAGE)) {
                uint paint_tex_idx_1 = (paint_and_rect_flag & PAINT_TEXTURE_INDEX_MASK);
                uvec4 _e108 = load_encoded_paint_texel(paint_tex_idx_1, 0u);
                uvec4 _e110 = load_encoded_paint_texel(paint_tex_idx_1, 1u);
                uvec4 _e112 = load_encoded_paint_texel(paint_tex_idx_1, 2u);
                vec2 _e113 = get_image_offset(_e108);
                vec2 _e114 = get_image_size(_e108);
                uvec2 _e115 = get_image_extend_modes(_e108);
                uint _e116 = get_image_atlas_index(_e108);
                uint _e117 = get_image_quality(_e108);
                uint _e118 = get_image_source_kind(_e108);
                float _e119 = get_image_padding(_e112);
                uint packed_tint = _e112.y;
                vec4 image_tint = (vec4(packed_tint & 0xFFu, packed_tint >> 8 & 0xFFu, packed_tint >> 16 & 0xFFu, packed_tint >> 24) / 255.0);
                bool is_multiply = (_e112.z == TINT_MODE_MULTIPLY);
                vec2 local_xy_1 = (sample_xy - _e113);
                float _e131 = extend_mode((local_xy_1.x + 1e-5), _e115.x, _e114.x);
                float _e136 = extend_mode((local_xy_1.y + 1e-5), _e115.y, _e114.y);
                vec2 extended_xy = vec2(_e131, _e136);
                if ((_e118 == IMAGE_SOURCE_EXTERNAL)) {
                    vec2 final_xy = (_e113 + extended_xy);
                    vec4 _e142 = sample_external_image(_e117, final_xy, _e113, _e114);
                    sample_color = _e142;
                } else {
                    if ((_e117 == IMAGE_QUALITY_HIGH)) {
                        vec2 final_xy_1 = (_e113 + extended_xy);
                        vec4 _e148 = bicubic_sample(_group_1_binding_0_fs, final_xy_1, int(_e116), _e113, _e114, _e115, _e119);
                        sample_color = _e148;
                    } else {
                        if ((_e117 == IMAGE_QUALITY_MEDIUM)) {
                            vec2 final_xy_2 = ((_e113 + extended_xy) - vec2(0.5));
                            vec4 _e157 = bilinear_sample(_group_1_binding_0_fs, final_xy_2, int(_e116), _e113, _e114, _e115, _e119);
                            sample_color = _e157;
                        } else {
                            vec2 final_xy_3 = (_e113 + extended_xy);
                            vec4 _e163 = texelFetch(_group_1_binding_0_fs, ivec3(uvec2(final_xy_3), int(_e116)), 0);
                            sample_color = _e163;
                        }
                    }
                }
                float _e164 = alpha;
                float _e166 = sample_color.w;
                vec4 _e168 = sample_color;
                final_color = (_e164 * (is_multiply ? (_e168 * image_tint) : (image_tint * _e166)));
            } else {
                if ((paint_type == PAINT_TYPE_LINEAR_GRADIENT)) {
                    uint paint_tex_idx_2 = (paint_and_rect_flag & PAINT_TEXTURE_INDEX_MASK);
                    uvec4 _e177 = load_encoded_paint_texel(paint_tex_idx_2, 0u);
                    uvec4 _e179 = load_encoded_paint_texel(paint_tex_idx_2, 1u);
                    vec2 _e180 = apply_gradient_transform(_e177, _e179, sample_xy);
                    float t_value_2 = (_e180.x + 1e-5);
                    uint _e184 = get_gradient_extend_mode(_e177);
                    uint _e185 = get_gradient_start(_e177);
                    uint _e186 = get_gradient_texture_width(_e177);
                    vec4 _e187 = sample_gradient_lut(t_value_2, _e184, _e185, _e186);
                    float _e188 = alpha;
                    final_color = (_e188 * _e187);
                } else {
                    if ((paint_type == PAINT_TYPE_RADIAL_GRADIENT)) {
                        uint paint_tex_idx_3 = (paint_and_rect_flag & PAINT_TEXTURE_INDEX_MASK);
                        uvec4 _e195 = load_encoded_paint_texel(paint_tex_idx_3, 0u);
                        uvec4 _e197 = load_encoded_paint_texel(paint_tex_idx_3, 1u);
                        uvec4 _e199 = load_encoded_paint_texel(paint_tex_idx_3, 2u);
                        uvec4 _e201 = load_encoded_paint_texel(paint_tex_idx_3, 3u);
                        vec2 _e202 = apply_gradient_transform(_e195, _e197, sample_xy);
                        vec2 _e203 = calculate_radial_gradient(_e202, _e199, _e201);
                        uint _e205 = get_gradient_extend_mode(_e195);
                        uint _e206 = get_gradient_start(_e195);
                        uint _e207 = get_gradient_texture_width(_e195);
                        vec4 _e208 = sample_gradient_lut(_e203.x, _e205, _e206, _e207);
                        float _e214 = alpha;
                        final_color = ((_e203.y != 0.0) ? (_e214 * _e208) : vec4(0.0, 0.0, 0.0, 0.0));
                    } else {
                        if ((paint_type == PAINT_TYPE_SWEEP_GRADIENT)) {
                            uint paint_tex_idx_4 = (paint_and_rect_flag & PAINT_TEXTURE_INDEX_MASK);
                            uvec4 _e225 = load_encoded_paint_texel(paint_tex_idx_4, 0u);
                            uvec4 _e227 = load_encoded_paint_texel(paint_tex_idx_4, 1u);
                            uvec4 _e229 = load_encoded_paint_texel(paint_tex_idx_4, 2u);
                            vec2 _e230 = apply_gradient_transform(_e225, _e227, sample_xy);
                            grad_pos = _e230;
                            vec2 _e232 = grad_pos;
                            vec2 _e233 = grad_pos;
                            grad_pos = mix(_e232, vec2(0.0), lessThan(abs(_e233), vec2(0.00024414063)));
                            float _e242 = grad_pos.x;
                            float _e244 = grad_pos.y;
                            float _e245 = xy_to_unit_angle(_e242, _e244);
                            float angle = (_e245 * TWO_PI);
                            float _e248 = get_sweep_start_angle(_e229);
                            float _e250 = get_sweep_inv_angle_delta(_e229);
                            float t_value_3 = ((angle - _e248) * _e250);
                            uint _e252 = get_gradient_extend_mode(_e225);
                            uint _e253 = get_gradient_start(_e225);
                            uint _e254 = get_gradient_texture_width(_e225);
                            vec4 _e255 = sample_gradient_lut(t_value_3, _e252, _e253, _e254);
                            float _e256 = alpha;
                            final_color = (_e256 * _e255);
                        } else {
                            if ((paint_type == PAINT_TYPE_BLURRED_ROUNDED_RECT)) {
                                uint paint_tex_idx_5 = (paint_and_rect_flag & PAINT_TEXTURE_INDEX_MASK);
                                uvec4 _e263 = load_encoded_paint_texel(paint_tex_idx_5, 0u);
                                uvec4 _e265 = load_encoded_paint_texel(paint_tex_idx_5, 1u);
                                uvec4 _e267 = load_encoded_paint_texel(paint_tex_idx_5, 2u);
                                uvec4 _e269 = load_encoded_paint_texel(paint_tex_idx_5, 3u);
                                uvec4 _e271 = load_encoded_paint_texel(paint_tex_idx_5, 4u);
                                float _e272 = alpha;
                                vec4 _e273 = calculate_blurred_rounded_rect(sample_xy, _e263, _e265, _e267, _e269, _e271);
                                final_color = (_e272 * _e273);
                            }
                        }
                    }
                }
            }
        }
    } else {
        if ((color_source == COLOR_SOURCE_LAYER)) {
            float layer_opacity = (float((paint_and_rect_flag & 255u)) * 0.003921569);
            float _e282 = alpha;
            vec4 _e287 = texelFetch(_group_0_binding_2_fs, ivec2(sample_xy), 0);
            final_color = ((_e282 * layer_opacity) * _e287);
        } else {
            final_color = vec4(0.0);
        }
    }
    vec4 _e291 = final_color;
    _fs2p_location0 = _e291;
    return;
}

"###;
    pub mod fragment {
        pub const CONFIG: &str = "Config_block_0Fragment";
        pub const ALPHAS_TEXTURE: &str = "_group_0_binding_0_fs";
        pub const ATLAS_TEXTURE_ARRAY: &str = "_group_1_binding_0_fs";
        pub const ENCODED_PAINTS_TEXTURE: &str = "_group_2_binding_0_fs";
        pub const EXTERNAL_TEXTURE: &str = "_group_1_binding_1_fs";
        pub const GRADIENT_TEXTURE: &str = "_group_3_binding_0_fs";
        pub const LAYER_INPUT_TEXTURE: &str = "_group_0_binding_2_fs";
    }
}
