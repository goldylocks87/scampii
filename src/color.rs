//! HSB color math, color-shifting utilities, and color parsing.
//!
//! All color operations work in the HSB (Hue-Saturation-Brightness) color
//! space. Scampii's palette is defined in terms of RGB, but hue
//! rotation happens in HSB to preserve the perceptual shading curves that give
//! scampii its form.

/// Epsilon for floating-point comparisons in color math.
const F32_EPS: f32 = 1.0 / 512.0;

/// Dominant hue angle of the scampii palette (warm orange, ~20 degrees).
const BASE_HUE: f32 = 20.0;

/// Convert 8-bit RGB to HSB (hue 0..360, saturation 0..1, brightness 0..1).
#[inline]
pub fn rgb_to_hsb(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;
    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let delta = max - min;

    let h = if delta < F32_EPS {
        0.0
    } else if (max - rf).abs() < F32_EPS {
        60.0 * ((gf - bf) / delta).rem_euclid(6.0)
    } else if (max - gf).abs() < F32_EPS {
        60.0 * (((bf - rf) / delta) + 2.0)
    } else {
        60.0 * (((rf - gf) / delta) + 4.0)
    };
    let s = if max < F32_EPS { 0.0 } else { delta / max };
    (h, s, max)
}

/// Convert HSB (hue 0..360, saturation 0..1, brightness 0..1) to 8-bit RGB.
#[inline]
pub fn hsb_to_rgb(h: f32, s: f32, b: f32) -> (u8, u8, u8) {
    let h = ((h % 360.0) + 360.0) % 360.0;
    let c = b * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = b - c;
    let (r1, g1, b1) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    (
        ((r1 + m) * 255.0).round() as u8,
        ((g1 + m) * 255.0).round() as u8,
        ((b1 + m) * 255.0).round() as u8,
    )
}

/// Shift a pixel's hue to match a target color.
///
/// Preserves the source saturation and brightness - those encode scampii's
/// 3D form and shading. `BASE_HUE` (20 degrees) is the dominant hue of the
/// scampii palette. The shift `tgt_h - BASE_HUE` rotates all body
/// hues so scampii takes on the target color while retaining its
/// light/shadow contours.
#[inline]
pub fn color_shift(src_h: f32, src_s: f32, src_b: f32, tgt_h: f32) -> (u8, u8, u8) {
    let h = src_h + (tgt_h - BASE_HUE);
    hsb_to_rgb(h, src_s, src_b)
}

/// Parse an OSC 10 response like `"\x1b]10;rgb:RRRR/GGGG/BBBB\x1b\\"` into `(r, g, b)`.
///
/// Handles 1, 2, 3, and 4 hex-digit channel values for maximum terminal
/// compatibility. The X11 color spec allows any of these widths.
pub fn parse_osc_color(s: &str) -> Option<(u8, u8, u8)> {
    let rgb_start = s.find("rgb:")?;
    let rgb_part = &s[rgb_start + 4..];
    let rgb_part = rgb_part.split(['\x1b', '\x07']).next()?;
    let mut parts = rgb_part.split('/');
    let r_hex = parts.next()?;
    let g_hex = parts.next()?;
    let b_hex = parts.next()?;

    /// Scale an N-digit hex channel value to 8-bit.
    fn parse_channel(h: &str) -> Option<u8> {
        let val = u16::from_str_radix(h, 16).ok()?;
        match h.len() {
            1 => Some((val as u8) * 17),
            2 => Some(val as u8),
            3 => Some((val >> 4) as u8),
            4 => Some((val >> 8) as u8),
            _ => None,
        }
    }

    Some((
        parse_channel(r_hex)?,
        parse_channel(g_hex)?,
        parse_channel(b_hex)?,
    ))
}

/// Parse a hex color string like `"ff6600"` or `"#ff6600"` into `(r, g, b)`.
pub fn parse_hex_color(s: &str) -> Option<(u8, u8, u8)> {
    let s = s.trim_start_matches('#');
    if s.len() != 6 || !s.is_ascii() {
        return None;
    }
    let r = u8::from_str_radix(&s[0..2], 16).ok()?;
    let g = u8::from_str_radix(&s[2..4], 16).ok()?;
    let b = u8::from_str_radix(&s[4..6], 16).ok()?;
    Some((r, g, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_round_trip() {
        let (h, s, b) = rgb_to_hsb(255, 0, 0);
        let (r, g, b2) = hsb_to_rgb(h, s, b);
        assert_eq!((r, g, b2), (255, 0, 0));
    }

    #[test]
    fn parse_hex_valid() {
        assert_eq!(parse_hex_color("#ff6600"), Some((255, 102, 0)));
        assert_eq!(parse_hex_color("e8732a"), Some((232, 115, 42)));
    }

    #[test]
    fn parse_hex_invalid() {
        assert_eq!(parse_hex_color("zzzzzz"), None);
        assert_eq!(parse_hex_color("fff"), None);
    }

    #[test]
    fn parse_osc_16bit() {
        let resp = "\x1b]10;rgb:ffff/0000/8080\x1b\\";
        assert_eq!(parse_osc_color(resp), Some((255, 0, 128)));
    }

    #[test]
    fn parse_osc_8bit() {
        let resp = "\x1b]10;rgb:ff/00/80\x07";
        assert_eq!(parse_osc_color(resp), Some((255, 0, 128)));
    }

    #[test]
    fn parse_osc_4bit() {
        let resp = "\x1b]10;rgb:f/0/8\x07";
        assert_eq!(parse_osc_color(resp), Some((255, 0, 136)));
    }

    #[test]
    fn parse_osc_12bit() {
        let resp = "\x1b]10;rgb:fff/000/800\x07";
        assert_eq!(parse_osc_color(resp), Some((255, 0, 128)));
    }

    #[test]
    fn black_hsb() {
        let (h, s, b) = rgb_to_hsb(0, 0, 0);
        assert_eq!(h, 0.0);
        assert_eq!(s, 0.0);
        assert_eq!(b, 0.0);
    }

    #[test]
    fn white_hsb() {
        let (h, s, b) = rgb_to_hsb(255, 255, 255);
        assert_eq!(h, 0.0);
        assert_eq!(s, 0.0);
        assert!((b - 1.0).abs() < F32_EPS);
    }
}
