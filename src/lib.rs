//! # scampii
//!
//! An animated pixel-art scampii for your terminal.
//!
//! `scampii` renders a 24x26 pixel sprite as a looping animation with
//! automatic terminal protocol detection. It supports pixel-perfect rendering
//! via iTerm2, Kitty, and Sixel image protocols, with a half-block Unicode
//! fallback for terminals that lack image support.
//!
//! ## Quick start (binary)
//!
//! ```text
//! scampii               # default scampii orange
//! scampii ocean         # blue ocean theme
//! scampii ff00ff        # any hex color
//! scampii --inline      # no alternate screen
//! scampii -p kitty      # force a specific protocol
//! ```
//!
//! ## Library usage
//!
//! The simplest way — auto-detects protocol, manages frame cycling:
//!
//! ```rust,no_run
//! let mut anim = scampii::Animation::new(scampii::Theme::classic());
//! let mut out = std::io::stdout();
//! anim.draw(&mut out).unwrap(); // renders next frame, advances automatically
//! ```
//!
//! For more control, use the renderers directly:
//!
//! ```rust,no_run
//! use scampii::{Theme, Renderer, FRAMES};
//!
//! let theme = Theme::classic();
//! let mut renderer = Renderer::new();
//! let mut out = std::io::stdout();
//! renderer.draw(&mut out, &FRAMES[0], &theme).unwrap();
//! ```
//!
//! ## Custom themes
//!
//! Themes shift scampii's 10-step body palette to any target hue while
//! preserving eye colors (pupil, iris, white, ring).
//!
//! ```rust
//! use scampii::Theme;
//!
//! let magenta = Theme::from_color(0xFF, 0x00, 0xFF);
//! let ocean = Theme::preset("ocean").unwrap();
//! let classic = Theme::classic();
//! ```

pub(crate) mod color;
pub mod error;
pub mod frame;
pub mod iterm;
pub mod kitty;
pub(crate) mod pixel;
pub mod raster;
pub mod sixel;
pub mod terminal;
pub mod theme;

// Re-export the most commonly used types at the crate root.
pub use color::parse_hex_color;
pub use error::ScampiiError;
pub use frame::{PackedFrame, Renderer, FRAMES, FRAME_COUNT, FRAME_HEIGHT, FRAME_WIDTH};
pub use iterm::ItermRenderer;
pub use kitty::KittyRenderer;
pub use pixel::{unpack_pixel, Hue};
pub use raster::MAX_SCALE;
pub use sixel::SixelRenderer;
pub use terminal::{query_terminal_fg, RawModeGuard};
pub use theme::Theme;

// ---------------------------------------------------------------------------
// Animation (high-level API)
// ---------------------------------------------------------------------------

/// High-level animated shrimp — auto-detects protocol, manages frame cycling.
///
/// This is the simplest way to use scampii. Create an `Animation`, then call
/// `draw()` in a loop. Frame cycling and protocol selection are handled for you.
///
/// ```rust,no_run
/// let mut anim = scampii::Animation::new(scampii::Theme::classic());
/// let mut out = std::io::stdout();
/// loop {
///     anim.draw(&mut out).unwrap();
///     std::thread::sleep(std::time::Duration::from_millis(100));
/// }
/// ```
pub struct Animation {
    theme: Theme,
    protocol: Protocol,
    scale: u8,
    frame_idx: usize,
    halfblock: Renderer,
    iterm: ItermRenderer,
    kitty: KittyRenderer,
    sixel: SixelRenderer,
}

impl Animation {
    /// Create a new animation with the given theme.
    ///
    /// Auto-detects the best protocol for the current terminal.
    /// Default scale is 4.
    pub fn new(theme: Theme) -> Self {
        Self {
            theme,
            protocol: detect_protocol(),
            scale: 4,
            frame_idx: 0,
            halfblock: Renderer::new(),
            iterm: ItermRenderer::new(),
            kitty: KittyRenderer::new(),
            sixel: SixelRenderer::new(),
        }
    }

    /// Set the pixel scale factor for image protocols (1..=16).
    ///
    /// Has no effect in halfblock mode. Default is 4.
    pub fn scale(mut self, scale: u8) -> Self {
        self.scale = scale;
        self
    }

    /// Force a specific rendering protocol instead of auto-detecting.
    pub fn protocol(mut self, protocol: Protocol) -> Self {
        self.protocol = protocol;
        self
    }

    /// Access the theme (e.g. to call `set_color`).
    pub fn theme_mut(&mut self) -> &mut Theme {
        &mut self.theme
    }

    /// Render the next frame and advance the animation.
    pub fn draw<Out: std::io::Write>(&mut self, out: &mut Out) -> Result<(), ScampiiError> {
        let frame = &FRAMES[self.frame_idx];
        match self.protocol {
            Protocol::Iterm => self.iterm.draw(out, frame, &self.theme, self.scale)?,
            Protocol::Kitty => self.kitty.draw(out, frame, &self.theme, self.scale)?,
            Protocol::Sixel => self.sixel.draw(out, frame, &self.theme, self.scale)?,
            Protocol::Halfblock => self.halfblock.draw(out, frame, &self.theme)?,
        }
        self.frame_idx = (self.frame_idx + 1) % FRAME_COUNT;
        Ok(())
    }
}

impl std::fmt::Debug for Animation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Animation")
            .field("protocol", &self.protocol)
            .field("scale", &self.scale)
            .field("frame_idx", &self.frame_idx)
            .finish()
    }
}

// ---------------------------------------------------------------------------
// Protocol detection
// ---------------------------------------------------------------------------

/// Supported rendering protocols for terminal image output.
///
/// Use [`detect_protocol`] to automatically select the best protocol for the
/// current terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Protocol {
    /// iTerm2 inline image protocol (OSC 1337).
    Iterm,
    /// Kitty graphics protocol.
    Kitty,
    /// DEC Sixel graphics.
    Sixel,
    /// Half-block character fallback (works in any true-color terminal).
    Halfblock,
}

impl std::str::FromStr for Protocol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "iterm" | "iterm2" => Ok(Self::Iterm),
            "kitty" => Ok(Self::Kitty),
            "sixel" => Ok(Self::Sixel),
            "halfblock" | "half-block" | "unicode" | "fallback" => Ok(Self::Halfblock),
            _ => Err(format!(
                "unknown protocol '{s}' (valid: iterm, kitty, sixel, halfblock)"
            )),
        }
    }
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Iterm => f.write_str("iterm"),
            Self::Kitty => f.write_str("kitty"),
            Self::Sixel => f.write_str("sixel"),
            Self::Halfblock => f.write_str("halfblock"),
        }
    }
}

/// Auto-detect the best rendering protocol for the current terminal.
///
/// Checks `VSCODE_CWD`, `TERM_PROGRAM`, and `TERM` environment variables.
/// Falls back to [`Protocol::Halfblock`] when no image protocol is detected.
pub fn detect_protocol() -> Protocol {
    // VS Code / Cursor set VSCODE_* env vars but often leave TERM_PROGRAM empty.
    // Both support the iTerm2 inline image protocol (OSC 1337).
    if std::env::var_os("VSCODE_CWD").is_some() {
        return Protocol::Iterm;
    }

    let term_program = std::env::var("TERM_PROGRAM");
    let term_program = term_program.as_deref().unwrap_or("");

    match term_program {
        "iTerm.app" => return Protocol::Iterm,
        "WezTerm" => return Protocol::Iterm,
        "kitty" => return Protocol::Kitty,
        "ghostty" => return Protocol::Kitty,
        "vscode" => return Protocol::Iterm,
        "Apple_Terminal" => return Protocol::Halfblock,
        _ => {}
    }

    // Only check $TERM if TERM_PROGRAM didn't identify the terminal.
    if term_program.is_empty() {
        match std::env::var("TERM").as_deref() {
            Ok(t) if t.starts_with("foot") || t.starts_with("mlterm") => {
                return Protocol::Sixel;
            }
            _ => {}
        }
    }

    Protocol::Halfblock
}
