//! Frame data and half-block fallback renderer.
//!
//! Each animation frame is a 2D grid of packed pixel bytes. The three frames
//! animate the scampii's antennae and legs in a looping sequence.
//!
//! Pixels are stored as `u8` values where `0` means transparent and `1..=16`
//! maps to a [`Hue`](crate::pixel::Hue) variant via
//! [`unpack_pixel`].
//!
//! The [`Renderer`] provides the half-block Unicode fallback for terminals that
//! do not support image protocols (iTerm2, Kitty, Sixel).

use std::io::Write;

use crate::error::ScampiiError;
use crate::pixel::unpack_pixel;
use crate::pixel::{EI, EP, EW, NN, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _A, _E, _L};
use crate::theme::Theme;

// ---------------------------------------------------------------------------
// Frame geometry
// ---------------------------------------------------------------------------

/// Frame width in pixels.
pub const FRAME_WIDTH: usize = 32;

/// Frame height in pixels.
pub const FRAME_HEIGHT: usize = 26;

/// The number of animation frames.
pub const FRAME_COUNT: usize = 3;

/// A complete animation frame stored as packed `u8` pixel bytes.
pub type PackedFrame = [[u8; FRAME_WIDTH]; FRAME_HEIGHT];

// ---------------------------------------------------------------------------
// Frame data
// ---------------------------------------------------------------------------

/// The three animation frames (32x26 pixels each, 2,496 bytes total).
///
/// Each pixel is a `u8`: `0` = transparent, `1..=16` = a `Hue` variant.
#[rustfmt::skip]
pub static FRAMES: [PackedFrame; FRAME_COUNT] = [
    // Frame 0 -- antennae sweep left
    [
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_A,NN,NN,NN,NN,_A,_A,_A,_A,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_A,NN,NN,NN,_A,NN,NN,NN,NN,NN,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_A,_A,_A,NN,NN,NN,NN,NN,NN,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_E,_E,_E,NN,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,_2,_2,_2,_2,_2,_2,_2,_2,_E,_E,EI,EW,_E,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,_2,_4,_8,_8,_6,_7,_4,_4,_4,_E,_E,EP,EI,_E,_2,_2,_2,_2,_2,_2,_2,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,_2,_4,_8,_9,_9,_8,_8,_6,_6,_4,_E,_E,_E,_E,_E,_7,_4,_4,_2,_1,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,_2,_4,_6,_9,_8,_6,_6,_7,_7,_7,_7,_4,_E,_E,_E,_7,_3,_2,_1,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_4,_2,_8,_6,_6,_7,_7,_7,_7,_7,_7,_7,_7,_7,_3,_2,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_4,_5,_2,_6,_7,_7,_4,_7,_4,_7,_4,_7,_4,_3,_2,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_6,_9,_6,_2,_0,_3,_2,_3,_2,_3,_2,_3,_2,_0,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_7,_8,_6,_7,_4,_0,_L,_0,_0,_L,_0,_L,_L,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,_2,_6,_7,_4,_2,_L,NN,_L,NN,_L,NN,NN,_L,NN,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_5,_2,_2,_2,_1,NN,NN,_L,NN,NN,_L,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_7,_9,_6,_4,_4,_1,NN,NN,NN,NN,_L,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,_2,_4,_8,_6,_4,_1,NN,NN,NN,NN,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,_2,_4,_4,_2,_4,_1,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,_2,_2,_4,_7,_4,_4,_1,_1,_1,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_2,_2,_4,_2,_1,_2,_3,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_2,_1,_2,_1,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_1,_2,_1,_3,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_1,_3,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    ],
    // Frame 1 -- antennae sweep up-left
    [
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,_A,_A,_A,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_A,_A,_A,_A,_A,_A,_A,_A,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_E,_E,_E,NN,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,_2,_2,_2,_2,_2,_2,_2,_2,_E,_E,EI,EW,_E,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,_2,_4,_8,_8,_6,_7,_4,_4,_4,_E,_E,EP,EI,_E,_2,_2,_2,_2,_2,_2,_2,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,_2,_4,_8,_9,_9,_8,_8,_6,_6,_4,_E,_E,_E,_E,_E,_7,_4,_4,_2,_1,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,_2,_4,_6,_9,_8,_6,_6,_7,_7,_7,_7,_4,_E,_E,_E,_7,_3,_2,_1,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_4,_2,_8,_6,_6,_7,_7,_7,_7,_7,_7,_7,_7,_7,_3,_2,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_4,_5,_2,_6,_7,_7,_4,_7,_4,_7,_4,_7,_4,_3,_2,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_6,_9,_6,_2,_0,_3,_2,_3,_2,_3,_2,_3,_2,_1,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_7,_8,_6,_7,_4,_L,_0,_0,_L,_0,_0,_L,_0,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,_2,_6,_7,_4,_2,_L,NN,NN,_L,NN,NN,_L,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_5,_2,_2,_2,_1,NN,_L,NN,NN,_L,NN,NN,_L,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_7,_9,_6,_4,_4,_1,NN,NN,NN,_L,NN,NN,_L,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,_2,_4,_8,_6,_4,_1,NN,NN,NN,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,_2,_4,_4,_2,_4,_1,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,_2,_2,_4,_7,_4,_4,_1,_1,_1,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_2,_2,_4,_2,_1,_2,_3,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_2,_1,_2,_1,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_1,_2,_1,_3,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_1,_3,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    ],
    // Frame 2 -- antennae swept right
    [
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_A,_A,_A,_A,NN,NN,NN,NN,_A,_A,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_A,_A,_A,_A,NN,NN,NN,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_E,_E,_E,NN,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,_2,_2,_2,_2,_2,_2,_2,_2,_E,_E,EI,EW,_E,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,_2,_4,_8,_8,_6,_7,_4,_4,_4,_E,_E,EP,EI,_E,_2,_2,_2,_2,_2,_2,_2,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,_2,_4,_8,_9,_9,_8,_8,_6,_6,_4,_E,_E,_E,_E,_E,_7,_4,_4,_2,_1,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,_2,_4,_6,_9,_8,_6,_6,_7,_7,_7,_7,_4,_E,_E,_E,_7,_3,_2,_1,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_4,_2,_8,_6,_6,_7,_7,_7,_7,_7,_7,_7,_7,_7,_3,_2,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_4,_5,_2,_6,_7,_7,_4,_7,_4,_7,_4,_7,_4,_3,_2,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_6,_9,_6,_2,_0,_3,_2,_3,_2,_3,_2,_3,_2,_1,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_7,_8,_6,_7,_4,_0,_0,_L,_0,_0,_L,_0,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,_2,_6,_7,_4,_2,_0,NN,_L,NN,NN,_L,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_5,_2,_2,_2,_1,NN,NN,NN,_L,NN,NN,_L,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_7,_9,_6,_4,_4,_1,NN,NN,_L,NN,NN,_L,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,_2,_4,_8,_6,_4,_1,NN,NN,NN,NN,_L,NN,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,_2,_4,_4,_2,_4,_1,_1,NN,NN,NN,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,_2,_2,_4,_7,_4,_4,_1,_1,_1,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_2,_2,_4,_2,_1,_2,_3,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_2,_1,_2,_1,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_1,_2,_1,_3,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_1,_3,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    ],
];

// ---------------------------------------------------------------------------
// Crop bounds (tightest bounding box across all 3 frames)
// ---------------------------------------------------------------------------

/// Leftmost column with content in any frame.
pub(crate) const COMPACT_X0: usize = 5;
/// One past the rightmost column with content in any frame.
pub(crate) const COMPACT_X1: usize = 29;
/// Cropped height in terminal rows (half-block: 2 pixel rows per line).
#[cfg(test)]
const COMPACT_H: usize = FRAME_HEIGHT / 2;

// ---------------------------------------------------------------------------
// Rendering helpers
// ---------------------------------------------------------------------------

/// u8-to-decimal-ASCII lookup table (avoids division in the render loop).
static U8_DECIMAL_LUT: [(u8, [u8; 3]); 256] = {
    let mut table = [(0u8, [0u8; 3]); 256];
    let mut i: u16 = 0;
    while i < 256 {
        let v = i as u8;
        if v >= 100 {
            table[i as usize] = (3, [b'0' + v / 100, b'0' + (v / 10) % 10, b'0' + v % 10]);
        } else if v >= 10 {
            table[i as usize] = (2, [b'0' + v / 10, b'0' + v % 10, 0]);
        } else {
            table[i as usize] = (1, [b'0' + v, 0, 0]);
        }
        i += 1;
    }
    table
};

/// Write a `u8` as decimal ASCII into `buf`.
#[inline]
fn write_u8_decimal(buf: &mut Vec<u8>, val: u8) {
    let (len, digits) = U8_DECIMAL_LUT[val as usize];
    buf.extend_from_slice(&digits[..len as usize]);
}

/// Write a raw SGR 38;2;R;G;B (set foreground RGB) escape into `buf`.
#[inline]
fn write_fg_rgb(buf: &mut Vec<u8>, r: u8, g: u8, b: u8) {
    buf.extend_from_slice(b"\x1b[38;2;");
    write_u8_decimal(buf, r);
    buf.push(b';');
    write_u8_decimal(buf, g);
    buf.push(b';');
    write_u8_decimal(buf, b);
    buf.push(b'm');
}

/// Write a raw SGR 48;2;R;G;B (set background RGB) escape into `buf`.
#[inline]
fn write_bg_rgb(buf: &mut Vec<u8>, r: u8, g: u8, b: u8) {
    buf.extend_from_slice(b"\x1b[48;2;");
    write_u8_decimal(buf, r);
    buf.push(b';');
    write_u8_decimal(buf, g);
    buf.push(b';');
    write_u8_decimal(buf, b);
    buf.push(b'm');
}

/// Upper half-block: foreground fills the top half of the cell.
const UPPER_HALF: &[u8] = "\u{2580}".as_bytes();
/// Lower half-block: foreground fills the bottom half of the cell.
const LOWER_HALF: &[u8] = "\u{2584}".as_bytes();
/// Reset background to terminal default.
const RESET_BG: &[u8] = b"\x1b[49m";

// ---------------------------------------------------------------------------
// Renderer
// ---------------------------------------------------------------------------

/// Half-block Unicode renderer (fallback for terminals without image protocols).
#[derive(Debug)]
pub struct Renderer {
    buf: Vec<u8>,
}

impl Renderer {
    /// Create a new renderer with a pre-allocated buffer.
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(32 * 1024),
        }
    }

    /// Render a frame using half-block characters (24 columns x 13 rows).
    pub fn draw<Out: Write>(
        &mut self,
        stdout: &mut Out,
        frame: &PackedFrame,
        theme: &Theme,
    ) -> Result<(), ScampiiError> {
        self.buf.clear();

        for row_pair in (0..FRAME_HEIGHT).step_by(2) {
            let top_row = &frame[row_pair];
            let bot_row = &frame[row_pair + 1];
            let mut bg_active = false;
            // Track last-emitted fg/bg RGB to skip redundant SGR sequences.
            // Reset each row because `\x1b[0m` at row end clears all attributes.
            let mut last_fg: Option<(u8, u8, u8)> = None;
            let mut last_bg: Option<(u8, u8, u8)> = None;

            for col in COMPACT_X0..COMPACT_X1 {
                let top = unpack_pixel(top_row[col]);
                let bot = unpack_pixel(bot_row[col]);

                match (top, bot) {
                    (Some(t), Some(b)) => {
                        // Both halves occupied: fg = top color, bg = bottom color.
                        let fg_rgb = theme.lut[t as usize];
                        let bg_rgb = theme.lut[b as usize];
                        if last_fg != Some(fg_rgb) {
                            write_fg_rgb(&mut self.buf, fg_rgb.0, fg_rgb.1, fg_rgb.2);
                            last_fg = Some(fg_rgb);
                        }
                        if last_bg != Some(bg_rgb) {
                            write_bg_rgb(&mut self.buf, bg_rgb.0, bg_rgb.1, bg_rgb.2);
                            last_bg = Some(bg_rgb);
                        }
                        self.buf.extend_from_slice(UPPER_HALF);
                        bg_active = true;
                    }
                    (Some(t), None) => {
                        // Top only: need default background.
                        if bg_active {
                            self.buf.extend_from_slice(RESET_BG);
                            bg_active = false;
                            last_bg = None;
                        }
                        let fg_rgb = theme.lut[t as usize];
                        if last_fg != Some(fg_rgb) {
                            write_fg_rgb(&mut self.buf, fg_rgb.0, fg_rgb.1, fg_rgb.2);
                            last_fg = Some(fg_rgb);
                        }
                        self.buf.extend_from_slice(UPPER_HALF);
                    }
                    (None, Some(b)) => {
                        // Bottom only: need default background.
                        if bg_active {
                            self.buf.extend_from_slice(RESET_BG);
                            bg_active = false;
                            last_bg = None;
                        }
                        let fg_rgb = theme.lut[b as usize];
                        if last_fg != Some(fg_rgb) {
                            write_fg_rgb(&mut self.buf, fg_rgb.0, fg_rgb.1, fg_rgb.2);
                            last_fg = Some(fg_rgb);
                        }
                        self.buf.extend_from_slice(LOWER_HALF);
                    }
                    (None, None) => {
                        // Empty cell: reset bg if needed, emit space.
                        if bg_active {
                            self.buf.extend_from_slice(RESET_BG);
                            bg_active = false;
                            last_bg = None;
                        }
                        // A space does not use the fg color, so invalidate
                        // tracking -- the next colored cell must re-emit fg.
                        last_fg = None;
                        self.buf.push(b' ');
                    }
                }
            }

            self.buf.extend_from_slice(b"\x1b[0m\r\n"); // reset + newline
        }

        stdout.write_all(&self.buf)?;
        stdout.flush()?;
        Ok(())
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `draw` must produce half-block characters, background color
    /// sequences, and fit in the expected row count.
    #[test]
    fn draw_smoke() {
        let theme = Theme::classic();
        let mut renderer = Renderer::new();
        let mut out: Vec<u8> = Vec::new();

        renderer
            .draw(&mut out, &FRAMES[0], &theme)
            .expect("draw should not fail writing to Vec");

        let output = String::from_utf8_lossy(&out);
        assert!(
            !output.starts_with("\x1b[H"),
            "output should NOT contain cursor-home escape (callers add it)"
        );
        assert!(
            output.contains('\u{2580}') || output.contains('\u{2584}'),
            "output should contain half-block characters"
        );
        assert!(
            output.contains("\x1b[48;2;"),
            "output should contain background color escapes"
        );
        let row_count = output.matches("\x1b[0m\r\n").count();
        assert_eq!(
            row_count, COMPACT_H,
            "output should have exactly {COMPACT_H} rows, got {row_count}"
        );
    }

    /// All three frames must render without error.
    #[test]
    fn all_frames_render() {
        let theme = Theme::classic();
        let mut renderer = Renderer::new();

        for (i, frame) in FRAMES.iter().enumerate() {
            let mut out: Vec<u8> = Vec::new();
            renderer
                .draw(&mut out, frame, &theme)
                .unwrap_or_else(|e| panic!("draw failed on frame {i}: {e}"));
            assert!(!out.is_empty(), "frame {i} produced empty output");
        }
    }

    /// Verify no bg color bleeds into transparent regions.
    #[test]
    fn draw_no_bg_bleed() {
        let theme = Theme::classic();
        let mut renderer = Renderer::new();
        let mut out: Vec<u8> = Vec::new();

        renderer
            .draw(&mut out, &FRAMES[0], &theme)
            .expect("draw should not fail");

        let output = String::from_utf8_lossy(&out);
        // Split into rows (delimited by \x1b[0m\r\n)
        for (row_idx, row) in output.split("\x1b[0m\r\n").enumerate() {
            if row.is_empty() {
                continue;
            }
            // Track whether we have an active background within this row
            let mut bg_set = false;
            let mut i = 0;
            let bytes = row.as_bytes();
            while i < bytes.len() {
                if i + 5 <= bytes.len() && &bytes[i..i + 5] == b"\x1b[49m" {
                    bg_set = false;
                    i += 5;
                } else if i + 7 <= bytes.len() && &bytes[i..i + 7] == b"\x1b[48;2;" {
                    bg_set = true;
                    // skip to 'm'
                    while i < bytes.len() && bytes[i] != b'm' {
                        i += 1;
                    }
                    i += 1;
                } else if bytes[i] == b' ' {
                    assert!(
                        !bg_set,
                        "row {row_idx}: space rendered with active background (bg bleed)"
                    );
                    i += 1;
                } else {
                    i += 1;
                }
            }
        }
    }

    /// H must be even for compact mode (pairs rows 0+1, 2+3, ...).
    #[test]
    fn frame_height_is_even() {
        assert_eq!(
            FRAME_HEIGHT % 2,
            0,
            "FRAME_HEIGHT must be even for compact half-block rendering"
        );
    }

    /// Verify the lookup table produces the same results as manual formatting.
    #[test]
    fn u8_decimal_lut_correctness() {
        for val in 0..=255u8 {
            let mut lut_buf = Vec::new();
            write_u8_decimal(&mut lut_buf, val);

            let expected = val.to_string();
            let lut_str = std::str::from_utf8(&lut_buf).unwrap();
            assert_eq!(
                lut_str, expected,
                "LUT mismatch for {val}: got {lut_str:?}, expected {expected:?}"
            );
        }
    }

    /// Verify that fg/bg dedup actually reduces the number of escape sequences
    /// compared to a naive approach (one escape per non-empty cell).
    #[test]
    fn compact_dedup_reduces_escapes() {
        let theme = Theme::classic();
        let mut renderer = Renderer::new();
        let mut out: Vec<u8> = Vec::new();
        renderer.draw(&mut out, &FRAMES[0], &theme).unwrap();

        let output = String::from_utf8_lossy(&out);

        // Count fg escape sequences
        let fg_count = output.matches("\x1b[38;2;").count();

        // Count total non-empty cells (each would need an fg escape naively)
        let total_nonempty: usize = (0..FRAME_HEIGHT)
            .step_by(2)
            .map(|row_pair| {
                (COMPACT_X0..COMPACT_X1)
                    .filter(|&col| {
                        unpack_pixel(FRAMES[0][row_pair][col]).is_some()
                            || unpack_pixel(FRAMES[0][row_pair + 1][col]).is_some()
                    })
                    .count()
            })
            .sum();

        assert!(
            fg_count < total_nonempty,
            "dedup should reduce fg escapes: {fg_count} fg sequences vs {total_nonempty} non-empty cells"
        );
        assert!(fg_count > 0, "should have at least one fg escape");
    }

    /// Verify that no bare background escape appears on rows that contain no
    /// "both halves occupied" cells.
    #[test]
    fn compact_no_spurious_bg_on_empty_rows() {
        let theme = Theme::classic();
        let mut renderer = Renderer::new();
        let mut out: Vec<u8> = Vec::new();
        renderer.draw(&mut out, &FRAMES[0], &theme).unwrap();

        let output = String::from_utf8_lossy(&out);

        // Split by the row terminator; each piece is one rendered row
        for (row_idx, line) in output.split("\x1b[0m\r\n").enumerate() {
            if line.is_empty() {
                continue;
            }
            let row_pair = row_idx * 2;
            if row_pair + 1 >= FRAME_HEIGHT {
                continue;
            }
            let top_row = &FRAMES[0][row_pair];
            let bot_row = &FRAMES[0][row_pair + 1];

            let has_both = (COMPACT_X0..COMPACT_X1).any(|col| {
                unpack_pixel(top_row[col]).is_some() && unpack_pixel(bot_row[col]).is_some()
            });

            if !has_both {
                assert!(
                    !line.contains("\x1b[48;2;"),
                    "row {row_idx} has no 'both' cells but contains a bg escape"
                );
            }
        }
    }
}
