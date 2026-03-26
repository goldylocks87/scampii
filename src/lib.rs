#![warn(missing_docs)]

//! # scampii
//!
//! An animated pixel-art shrimp for your terminal.
//!
//! Renders a 24x26 pixel sprite as a looping inline PNG animation via the
//! iTerm2 image protocol (OSC 1337). Works in iTerm2, VS Code, Cursor,
//! WezTerm, and any terminal supporting inline images.
//!
//! ## Quick start
//!
//! ```rust,no_run
//! let mut anim = scampii::Animation::new(scampii::Theme::classic());
//! let mut out = std::io::stdout();
//!
//! for _ in 0..30 {
//!     anim.draw(&mut out).unwrap();
//!     std::thread::sleep(std::time::Duration::from_millis(100));
//! }
//! ```
//!
//! ## Themes
//!
//! ```rust
//! use scampii::Theme;
//!
//! let magenta = Theme::from_color(0xFF, 0x00, 0xFF);
//! let ocean = Theme::preset("ocean").unwrap();
//! ```
//!
//! ## Utilities
//!
//! - [`parse_hex_color`]: Parse `"ff6600"` or `"#ff6600"` into `(u8, u8, u8)`.
//! - [`png`]: Export frames as PNG files or emoji-sized images.

pub(crate) mod color;
pub(crate) mod error;
pub(crate) mod frame;
pub(crate) mod pixel;
pub mod png;
pub(crate) mod terminal;
pub mod theme;

pub use color::parse_hex_color;
pub use error::ScampiiError;
pub use frame::{PackedFrame, FRAMES, FRAME_COUNT};
pub use pixel::{unpack_pixel, Hue};
pub use terminal::{query_terminal_fg, RawModeGuard};
pub use theme::Theme;

// ---------------------------------------------------------------------------
// Animation
// ---------------------------------------------------------------------------

/// Animated scampii -- pre-renders all frames at construction, zero-alloc draw.
///
/// # Example
///
/// ```rust,no_run
/// let mut anim = scampii::Animation::new(scampii::Theme::classic());
/// let mut out = std::io::stdout();
///
/// loop {
///     anim.draw(&mut out).unwrap();
///     std::thread::sleep(std::time::Duration::from_millis(100));
/// }
/// ```
///
/// Any [`Theme`] or `(r, g, b)` tuple works:
///
/// ```rust,no_run
/// let mut anim = scampii::Animation::new(scampii::Theme::from((0xFF, 0x00, 0x99)));
/// ```
pub struct Animation {
    /// Pre-built iTerm2 escape payloads for each frame (PNG + base64 + OSC 1337).
    frames: Vec<Vec<u8>>,
    frame_idx: usize,
    theme: Theme,
    cell_scale: u8,
}

impl Animation {
    /// Create a new animation with the given theme.
    ///
    /// Accepts anything that converts into a [`Theme`] -- a `Theme` value,
    /// an `(r, g, b)` tuple, or a preset name via [`Theme::preset`].
    ///
    /// Pre-renders all frames as base64-encoded PNGs wrapped in the iTerm2
    /// inline image escape sequence. The [`draw`](Self::draw) hot loop does
    /// zero allocation.
    pub fn new(theme: impl Into<Theme>) -> Self {
        let theme = theme.into();
        let mut anim = Self {
            frames: Vec::new(),
            frame_idx: 0,
            theme,
            cell_scale: 1,
        };
        anim.rebuild_frames();
        anim
    }

    /// Set the display scale in character cells. Default is 1.
    ///
    /// At scale 1 the shrimp is 2 cells wide and 1 cell tall.
    /// At scale 4 it is 8 cells wide and 4 cells tall.
    /// Scales with terminal zoom just like text.
    pub fn scale(mut self, scale: u8) -> Self {
        self.cell_scale = scale.max(1);
        self.rebuild_frames();
        self
    }

    fn rebuild_frames(&mut self) {
        let w = 2 * self.cell_scale as u16;
        let h = self.cell_scale as u16;
        self.frames = FRAMES
            .iter()
            .map(|frame| {
                let png_data = png::render_png(frame, &self.theme, self.cell_scale.max(1) * 2);
                let mut payload = Vec::with_capacity(png_data.len() * 4 / 3 + 128);
                let header = format!(
                    "\x1b]1337;File=inline=1;width={w};height={h};preserveAspectRatio=1:"
                );
                payload.extend_from_slice(header.as_bytes());
                png::base64_encode(&png_data, &mut payload);
                payload.push(0x07);
                payload
            })
            .collect();
    }

    /// Render the next frame inline and advance the animation.
    ///
    /// This is a single `write_all` + `flush` -- no encoding, no allocation.
    pub fn draw<Out: std::io::Write>(&mut self, out: &mut Out) -> Result<(), ScampiiError> {
        out.write_all(&self.frames[self.frame_idx])?;
        out.flush()?;
        self.frame_idx = (self.frame_idx + 1) % FRAME_COUNT;
        Ok(())
    }

    /// Current frame index (0..[`FRAME_COUNT`]).
    pub fn frame_index(&self) -> usize {
        self.frame_idx
    }

    /// Reset the animation to frame 0.
    pub fn reset(&mut self) {
        self.frame_idx = 0;
    }
}

impl std::fmt::Debug for Animation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Animation")
            .field("frame_idx", &self.frame_idx)
            .field(
                "payload_sizes",
                &self.frames.iter().map(|f| f.len()).collect::<Vec<_>>(),
            )
            .finish()
    }
}
