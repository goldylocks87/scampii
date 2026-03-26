//! Pixel-level types: [`Hue`] for the scampii's color palette.
//!
//! Each pixel in a frame is either transparent (`0`) or a packed [`Hue`]
//! value (`1..=HUE_COUNT`). The `hue_variants!` macro keeps variant count,
//! RGB values, packed aliases, and [`ALL_HUES`] in sync automatically.

use crate::color::{color_shift, rgb_to_hsb};

// ---------------------------------------------------------------------------
// Hue (generated via macro)
// ---------------------------------------------------------------------------

/// Declares all [`Hue`] variants and derives `HUE_COUNT`, `ALL_HUES`,
/// `Hue::rgb()`, and the packed-pixel alias constants.
///
/// Adding a new variant here is the only change needed -- everything else
/// stays in sync automatically.
macro_rules! hue_variants {
    (
        $(
            $variant:ident : rgb($r:expr, $g:expr, $b:expr), alias($alias:ident), packed($packed:expr)
        );+ $(;)?
    ) => {
        /// A named color step in the scampii palette.
        ///
        /// `S0` through `S9` form a brightness ramp from darkest to brightest
        /// body color. Eye variants (`EyePupil`, `EyeIris`, `EyeWhite`,
        /// `EyeRing`) keep fixed colors regardless of the active theme. Legs
        /// and antennae hue-shift along with the body.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(u8)]
        #[non_exhaustive]
        pub enum Hue {
            $( #[doc = concat!("Palette slot: rgb(", stringify!($r), ", ", stringify!($g), ", ", stringify!($b), ")")] $variant ),+
        }

        /// Total number of [`Hue`] variants.
        pub const HUE_COUNT: usize = {
            let mut n = 0u32;
            $( { let _ = Hue::$variant; n += 1; } )+
            n as usize
        };

        /// All [`Hue`] variants in declaration order, for iteration.
        pub const ALL_HUES: [Hue; HUE_COUNT] = [ $( Hue::$variant ),+ ];

        impl Hue {
            /// The canonical RGB triple for this hue in the original scampii palette.
            #[inline]
            pub const fn rgb(self) -> (u8, u8, u8) {
                match self {
                    $( Hue::$variant => ($r, $g, $b) ),+
                }
            }
        }

        // Packed pixel aliases for frame data (0 = None, 1.. = Some(variant)).
        $( #[doc(hidden)] pub const $alias: u8 = $packed; )+

        /// Packed pixel value representing a transparent (empty) pixel.
        #[doc(hidden)]
        pub const NN: u8 = 0;
    }
}

#[rustfmt::skip]
hue_variants! {
    S0       : rgb(  8,   7,   7), alias(_0), packed( 1);
    S1       : rgb( 26,  19,  17), alias(_1), packed( 2);
    S2       : rgb( 97,  39,  33), alias(_2), packed( 3);
    S3       : rgb(176,  91,  44), alias(_3), packed( 4);
    S4       : rgb(185,  69,  29), alias(_4), packed( 5);
    S5       : rgb(211, 151,  65), alias(_5), packed( 6);
    S6       : rgb(232, 138,  54), alias(_6), packed( 7);
    S7       : rgb(241, 100,  31), alias(_7), packed( 8);
    S8       : rgb(252, 165, 112), alias(_8), packed( 9);
    S9       : rgb(255, 240, 137), alias(_9), packed(10);
    EyePupil : rgb( 86,  79,  91), alias(EP), packed(11);
    EyeIris  : rgb(210, 210, 210), alias(EI), packed(12);
    EyeWhite : rgb(255, 255, 255), alias(EW), packed(13);
    Leg      : rgb( 26,  19,  17), alias(_L), packed(14);
    Antenna  : rgb( 20,  16,  16), alias(_A), packed(15);
    EyeRing  : rgb( 14,  12,  12), alias(_E), packed(16);
}

// Compile-time assertion: packed values must fit in u8.
const _: () = assert!(HUE_COUNT <= 255, "Too many Hue variants for u8 packing");

impl Hue {
    /// The canonical HSB triple for this hue, derived from [`Self::rgb`].
    #[inline]
    pub fn hsb(self) -> (f32, f32, f32) {
        let (r, g, b) = self.rgb();
        rgb_to_hsb(r, g, b)
    }

    /// Returns `true` if this hue represents an eye element whose color
    /// should not be shifted by themes. Legs and antennae DO shift with
    /// the body so the whole scampii recolors consistently.
    pub const fn is_structural(self) -> bool {
        matches!(
            self,
            Self::EyePupil | Self::EyeIris | Self::EyeWhite | Self::EyeRing
        )
    }

    /// Resolve this hue to a terminal RGB color, shifted toward the target hue.
    ///
    /// Structural hues keep their canonical RGB. Body hues are rotated in HSB
    /// space so scampii takes on the target color while preserving its
    /// shading curves.
    #[inline]
    pub fn resolve(self, tgt_h: f32) -> (u8, u8, u8) {
        if self.is_structural() {
            self.rgb()
        } else {
            let (h, s, b) = self.hsb();
            color_shift(h, s, b, tgt_h)
        }
    }
}

impl std::fmt::Display for Hue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Decode a packed pixel byte into `Option<Hue>`.
///
/// `0` maps to `None` (transparent). `1..=HUE_COUNT` maps to the corresponding
/// variant in `ALL_HUES`.
#[inline]
pub const fn unpack_pixel(byte: u8) -> Option<Hue> {
    if byte == 0 || byte as usize > HUE_COUNT {
        None
    } else {
        Some(ALL_HUES[(byte - 1) as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pack -> unpack round-trip for every `Hue` variant.
    #[test]
    fn unpack_pixel_round_trip() {
        assert_eq!(unpack_pixel(NN), None);

        for (i, &hue) in ALL_HUES.iter().enumerate() {
            let packed = (i as u8) + 1;
            let unpacked = unpack_pixel(packed);
            assert_eq!(
                unpacked,
                Some(hue),
                "round-trip failed for packed value {packed} (expected {hue:?})"
            );
        }
    }

    /// Out-of-range packed values return `None`.
    #[test]
    fn unpack_pixel_out_of_range() {
        assert_eq!(unpack_pixel((HUE_COUNT as u8) + 1), None);
        assert_eq!(unpack_pixel(255), None);
    }

    /// Only eye hues are structural (fixed color). Legs and antennae shift.
    #[test]
    fn structural_hues() {
        assert!(!Hue::S0.is_structural());
        assert!(!Hue::S9.is_structural());
        assert!(!Hue::Leg.is_structural());
        assert!(!Hue::Antenna.is_structural());
        assert!(Hue::EyePupil.is_structural());
        assert!(Hue::EyeIris.is_structural());
        assert!(Hue::EyeWhite.is_structural());
        assert!(Hue::EyeRing.is_structural());
    }
}
