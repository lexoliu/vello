// Copyright 2026 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Shared extend mode (pad/repeat/reflect) handling for images and gradients.

const EXTEND_PAD: u32 = 0u;
const EXTEND_REPEAT: u32 = 1u;
const EXTEND_REFLECT: u32 = 2u;

fn extend_mode(t: f32, mode: u32, max: f32) -> f32 {
    switch mode {
        case EXTEND_PAD: {
            // `t` is a sampling position in image space, where texel `i` covers `[i, i + 1)` and
            // has its centre at `i + 0.5`. Padding saturates that position at the centres of the
            // outermost texels, so the whole first and last texel remain reachable: filtered
            // sampling subtracts the half texel that turns a centre into a corner *after* this
            // clamp, and clamping to `max - 1.0` here would stop half a texel short of the last
            // centre, freezing the trailing edge on a blend of the final two texels.
            return clamp(t, 0.5, max - 0.5);
        }
        case EXTEND_REPEAT: {
            return extend_mode_normalized(t / max, mode) * max;
        }
        case EXTEND_REFLECT, default: {
            return extend_mode_normalized(t / max, mode) * max;
        }
    }
}

fn extend_mode_normalized(t: f32, mode: u32) -> f32 {
    switch mode {
        case EXTEND_PAD: {
            return clamp(t, 0.0, 1.0);
        }
        case EXTEND_REPEAT: {
            return fract(t);
        }
        case EXTEND_REFLECT, default: {
            return abs(t - 2.0 * round(0.5 * t));
        }
    }
}
