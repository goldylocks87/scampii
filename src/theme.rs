//! Theme system -- maps each [`Hue`] to a resolved terminal RGB color.

use crate::color::rgb_to_hsb;
use crate::pixel::{Hue, ALL_HUES, HUE_COUNT};

/// A resolved color lookup table for the scampii palette.
///
/// Themes are cheap to construct and immutable once built. The animation loop
/// indexes into the LUT by [`Hue`] variant (via `repr(u8)`) to avoid per-pixel
/// branching.
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    /// Color for each [`Hue`] variant, indexed by `hue as usize`.
    ///
    /// Library consumers can read this directly for bulk pixel processing
    /// without per-pixel method call overhead. Use [`Theme::color`] for
    /// single lookups.
    pub lut: [(u8, u8, u8); HUE_COUNT],
}

impl Theme {
    /// HSB shift -- rotates the scampii palette to match a target color.
    pub fn from_color(r: u8, g: u8, b: u8) -> Self {
        let (tgt_h, _tgt_s, _tgt_b) = rgb_to_hsb(r, g, b);
        let mut lut = [(0u8, 0u8, 0u8); HUE_COUNT];
        for hue in ALL_HUES {
            lut[hue as usize] = hue.resolve(tgt_h);
        }
        Self { lut }
    }

    /// Original GIF palette -- no shifting, canonical RGB values.
    pub fn classic() -> Self {
        let mut lut = [(0u8, 0u8, 0u8); HUE_COUNT];
        for hue in ALL_HUES {
            lut[hue as usize] = hue.rgb();
        }
        Self { lut }
    }

    /// Override the color for a specific hue in this theme.
    ///
    /// Useful for tweaking individual elements (e.g. brightening antennae)
    /// without changing the canonical palette.
    pub fn set_color(&mut self, hue: Hue, r: u8, g: u8, b: u8) {
        self.lut[hue as usize] = (r, g, b);
    }

    /// Build from a named preset.
    ///
    /// Returns `None` if the name is not recognized. Available presets:
    /// `classic`, `ocean`, `forest`, `neon`, `gold`, `ice`, `lava`, `midnight`, `barbie`.
    pub fn preset(name: &str) -> Option<Self> {
        match name {
            "classic" => Some(Self::classic()),
            "ocean" => Some(Self::from_color(0x20, 0x90, 0xDD)),
            "forest" => Some(Self::from_color(0x30, 0xA8, 0x40)),
            "neon" => Some(Self::from_color(0xFF, 0x00, 0xFF)),
            "gold" => Some(Self::from_color(0xFF, 0xB4, 0x3C)),
            "ice" => Some(Self::from_color(0x88, 0xCC, 0xFF)),
            "lava" => Some(Self::from_color(0xFF, 0x33, 0x00)),
            "midnight" => Some(Self::from_color(0x66, 0x33, 0x99)),
            "barbie" => {
                let mut t = Self::from_color(0xE0, 0x21, 0x8A);
                t.set_color(Hue::Antenna, 0x80, 0x35, 0x5A);
                t.set_color(Hue::Leg, 0x66, 0x20, 0x40);
                Some(t)
            }
            _ => None,
        }
    }

    /// All recognized preset names.
    pub const PRESET_NAMES: &[&str] = &[
        "classic", "ocean", "forest", "neon", "gold", "ice", "lava", "midnight", "barbie",
    ];

    /// Look up the resolved RGB color for a given hue.
    #[inline]
    pub fn color(&self, hue: Hue) -> (u8, u8, u8) {
        self.lut[hue as usize]
    }
}

impl Default for Theme {
    /// Returns the classic (original) scampii palette.
    fn default() -> Self {
        Self::classic()
    }
}

impl From<(u8, u8, u8)> for Theme {
    /// Create a theme shifted to the given RGB color.
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::from_color(r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every name in `PRESET_NAMES` must return `Some` from `Theme::preset`.
    #[test]
    fn preset_returns_some_for_all_names() {
        for &name in Theme::PRESET_NAMES {
            assert!(
                Theme::preset(name).is_some(),
                "Theme::preset({name:?}) returned None"
            );
        }
    }

    /// Unknown names must return `None`.
    #[test]
    fn preset_returns_none_for_unknown() {
        assert!(Theme::preset("doesnotexist").is_none());
        assert!(Theme::preset("").is_none());
        assert!(Theme::preset("OCEAN").is_none()); // case-sensitive
    }

    /// Classic theme preserves the canonical palette exactly.
    #[test]
    fn classic_matches_canonical_rgb() {
        let theme = Theme::classic();
        for hue in ALL_HUES {
            assert_eq!(theme.color(hue), hue.rgb(), "mismatch for {hue:?}");
        }
    }

    /// Theme implements Clone.
    #[test]
    fn theme_is_clone() {
        let a = Theme::classic();
        let b = a.clone();
        for hue in ALL_HUES {
            assert_eq!(a.color(hue), b.color(hue));
        }
    }
}
