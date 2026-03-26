//! Frame data -- three 32x26 pixel animation frames.

use crate::pixel::{EI, EP, EW, NN, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _A, _E, _L};

/// Frame width in pixels.
pub const FRAME_WIDTH: usize = 32;

/// Frame height in pixels.
pub const FRAME_HEIGHT: usize = 26;

/// The number of animation frames.
pub const FRAME_COUNT: usize = 3;

/// A complete animation frame stored as packed `u8` pixel bytes.
pub type PackedFrame = [[u8; FRAME_WIDTH]; FRAME_HEIGHT];

/// Leftmost column with content in any frame.
pub(crate) const COMPACT_X0: usize = 5;

/// One past the rightmost column with content in any frame.
pub(crate) const COMPACT_X1: usize = 29;

/// The three animation frames (32x26 pixels each).
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
        [NN,NN,NN,NN,NN,NN,_2,_6,_7,_4,_2,_0,_L,_0,_L,_0,_L,_0,_L,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_5,_2,_2,_2,_1,NN,_L,NN,_L,NN,_L,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_7,_9,_6,_4,_4,_1,_L,NN,NN,NN,NN,_L,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
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
    // Frame 2 -- antennae sweep right
    [
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_A,_A,_A,_A,_A,_A,_A,_A,_A,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_A,_A,_A,_A,NN,_E,_E,_E,NN,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,NN,_2,_2,_2,_2,_2,_2,_2,_2,_E,_E,EI,EW,_E,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,NN,_2,_4,_8,_8,_6,_7,_4,_4,_4,_E,_E,EP,EI,_E,_2,_2,_2,_2,_2,_2,_2,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,NN,_2,_4,_8,_9,_9,_8,_8,_6,_6,_4,_E,_E,_E,_E,_E,_7,_4,_4,_2,_1,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,_2,_4,_6,_9,_8,_6,_6,_7,_7,_7,_7,_4,_E,_E,_E,_7,_3,_2,_1,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_4,_2,_8,_6,_6,_7,_7,_7,_7,_7,_7,_7,_7,_7,_3,_2,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_4,_5,_2,_6,_7,_7,_4,_7,_4,_7,_4,_7,_4,_3,_2,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_6,_9,_6,_2,_0,_3,_2,_3,_2,_3,_2,_3,_2,_0,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_7,_8,_6,_7,_4,_0,_0,_L,_0,_0,_L,_0,_L,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,NN,_2,_6,_7,_4,_2,_0,_L,_0,_L,_0,_L,_0,NN,_L,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_5,_2,_2,_2,_1,NN,NN,_L,NN,_L,NN,_L,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
        [NN,NN,NN,NN,NN,_2,_7,_9,_6,_4,_4,_1,NN,NN,NN,_L,NN,NN,_L,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
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
];
