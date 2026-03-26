//! PNG encoding and rasterisation.
//!
//! Converts packed pixel frames into RGBA bitmaps and encodes them as PNG.
//! Includes base64 encoding for the iTerm2 inline image protocol.
//!
//! ## Which function do I need?
//!
//! | I want to...                         | Use                     |
//! |--------------------------------------|-------------------------|
//! | Embed one animation frame in a file  | [`render_png`]          |
//! | Make a Slack/Discord emoji            | [`render_emoji`]        |
//! | Export all 3 frames                   | [`render_all_frames`]   |
//! | Export all 3 frames as emoji          | [`render_all_emoji`]    |
//!
//! # Example
//!
//! ```rust
//! let theme = scampii::Theme::classic();
//! let png_bytes = scampii::png::render_emoji(&scampii::FRAMES[0], &theme, 128);
//! assert!(png_bytes.starts_with(&[0x89, b'P', b'N', b'G']));
//! ```

use crate::frame::{PackedFrame, COMPACT_X0, COMPACT_X1, FRAME_HEIGHT};
use crate::pixel::unpack_pixel;
use crate::theme::Theme;

/// Cropped sprite width in pixels.
pub(crate) const CROP_W: usize = COMPACT_X1 - COMPACT_X0;

/// Cropped sprite height in pixels.
pub(crate) const CROP_H: usize = FRAME_HEIGHT;

/// Maximum allowed scale factor for PNG export.
const MAX_SCALE: u8 = 16;

// ---------------------------------------------------------------------------
// Rasteriser
// ---------------------------------------------------------------------------

/// Rasterise a frame to RGBA pixels at the given scale.
pub(crate) fn rasterise(frame: &PackedFrame, theme: &Theme, scale: u8) -> (Vec<u8>, u32, u32) {
    let s = scale.clamp(1, MAX_SCALE) as usize;
    let w = CROP_W * s;
    let h = CROP_H * s;
    let mut rgba = vec![0u8; w * h * 4];

    for (y, row) in frame.iter().enumerate().take(CROP_H) {
        for x in 0..CROP_W {
            let px = row[x + COMPACT_X0];
            let (r, g, b, a) = match unpack_pixel(px) {
                Some(hue) => {
                    let (cr, cg, cb) = theme.color(hue);
                    (cr, cg, cb, 255u8)
                }
                None => (0, 0, 0, 0),
            };
            for sy in 0..s {
                for sx in 0..s {
                    let idx = ((y * s + sy) * w + (x * s + sx)) * 4;
                    rgba[idx] = r;
                    rgba[idx + 1] = g;
                    rgba[idx + 2] = b;
                    rgba[idx + 3] = a;
                }
            }
        }
    }

    (rgba, w as u32, h as u32)
}

// ---------------------------------------------------------------------------
// Base64
// ---------------------------------------------------------------------------

const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Base64-encode `input`, appending the result to `out`.
pub(crate) fn base64_encode(input: &[u8], out: &mut Vec<u8>) {
    let mut i = 0;
    let len = input.len();
    while i + 2 < len {
        let n = ((input[i] as u32) << 16) | ((input[i + 1] as u32) << 8) | (input[i + 2] as u32);
        out.push(BASE64_CHARS[((n >> 18) & 0x3F) as usize]);
        out.push(BASE64_CHARS[((n >> 12) & 0x3F) as usize]);
        out.push(BASE64_CHARS[((n >> 6) & 0x3F) as usize]);
        out.push(BASE64_CHARS[(n & 0x3F) as usize]);
        i += 3;
    }
    let remaining = len - i;
    if remaining == 2 {
        let n = ((input[i] as u32) << 16) | ((input[i + 1] as u32) << 8);
        out.push(BASE64_CHARS[((n >> 18) & 0x3F) as usize]);
        out.push(BASE64_CHARS[((n >> 12) & 0x3F) as usize]);
        out.push(BASE64_CHARS[((n >> 6) & 0x3F) as usize]);
        out.push(b'=');
    } else if remaining == 1 {
        let n = (input[i] as u32) << 16;
        out.push(BASE64_CHARS[((n >> 18) & 0x3F) as usize]);
        out.push(BASE64_CHARS[((n >> 12) & 0x3F) as usize]);
        out.push(b'=');
        out.push(b'=');
    }
}

// ---------------------------------------------------------------------------
// Public PNG API
// ---------------------------------------------------------------------------

/// Render a single frame as a PNG byte vector.
pub fn render_png(frame: &PackedFrame, theme: &Theme, scale: u8) -> Vec<u8> {
    let (rgba, w, h) = rasterise(frame, theme, scale);
    encode_rgba_png(&rgba, w, h)
}

/// Render a frame as a square emoji-ready PNG.
///
/// Crops to content bounds, nearest-neighbor upscales, and centers on a
/// transparent canvas of `size × size` pixels.
pub fn render_emoji(frame: &PackedFrame, theme: &Theme, size: u32) -> Vec<u8> {
    let (rgba_1x, w1, h1) = rasterise(frame, theme, 1);
    let w1 = w1 as usize;
    let h1 = h1 as usize;

    let mut min_x = w1;
    let mut min_y = h1;
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    for y in 0..h1 {
        for x in 0..w1 {
            if rgba_1x[(y * w1 + x) * 4 + 3] > 0 {
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
            }
        }
    }

    let sz = size.max(1) as usize;

    if max_x < min_x || max_y < min_y {
        let empty = vec![0u8; sz * sz * 4];
        return encode_rgba_png(&empty, sz as u32, sz as u32);
    }

    let cw = max_x - min_x + 1;
    let ch = max_y - min_y + 1;
    let s = (sz / cw).min(sz / ch).max(1);
    let sw = cw * s;
    let sh = ch * s;
    let ox = (sz - sw) / 2;
    let oy = (sz - sh) / 2;

    let mut out = vec![0u8; sz * sz * 4];
    for dy in 0..sh {
        let src_y = min_y + dy / s;
        for dx in 0..sw {
            let src_x = min_x + dx / s;
            let src_idx = (src_y * w1 + src_x) * 4;
            let dst_idx = ((oy + dy) * sz + (ox + dx)) * 4;
            out[dst_idx..dst_idx + 4].copy_from_slice(&rgba_1x[src_idx..src_idx + 4]);
        }
    }

    encode_rgba_png(&out, sz as u32, sz as u32)
}

/// Render all animation frames as PNGs.
pub fn render_all_frames(theme: &Theme, scale: u8) -> Vec<Vec<u8>> {
    crate::FRAMES.iter().map(|f| render_png(f, theme, scale)).collect()
}

/// Render all animation frames as emoji-sized PNGs.
pub fn render_all_emoji(theme: &Theme, size: u32) -> Vec<Vec<u8>> {
    crate::FRAMES.iter().map(|f| render_emoji(f, theme, size)).collect()
}

// ---------------------------------------------------------------------------
// Minimal PNG encoder (uncompressed STORE deflate)
// ---------------------------------------------------------------------------

fn encode_rgba_png(rgba: &[u8], w: u32, h: u32) -> Vec<u8> {
    let mut png = Vec::with_capacity(rgba.len() + 1024);

    // PNG signature
    png.extend_from_slice(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]);

    // IHDR
    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&w.to_be_bytes());
    ihdr.extend_from_slice(&h.to_be_bytes());
    ihdr.push(8); // bit depth
    ihdr.push(6); // color type: RGBA
    ihdr.push(0); // compression
    ihdr.push(0); // filter
    ihdr.push(0); // interlace
    write_chunk(&mut png, b"IHDR", &ihdr);

    // IDAT
    let row_bytes = (w as usize) * 4;
    let mut raw = Vec::with_capacity((h as usize) * (1 + row_bytes));
    for y in 0..h as usize {
        raw.push(0); // filter: None
        let start = y * row_bytes;
        raw.extend_from_slice(&rgba[start..start + row_bytes]);
    }
    let deflated = zlib_store(&raw);
    write_chunk(&mut png, b"IDAT", &deflated);

    // IEND
    write_chunk(&mut png, b"IEND", &[]);

    png
}

fn write_chunk(out: &mut Vec<u8>, chunk_type: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    out.extend_from_slice(chunk_type);
    out.extend_from_slice(data);
    // Incremental CRC over chunk_type then data (no allocation)
    let crc = crc32_update(crc32_update(0xFFFF_FFFF, chunk_type), data) ^ 0xFFFF_FFFF;
    out.extend_from_slice(&crc.to_be_bytes());
}

fn zlib_store(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len() + 64);
    out.push(0x78);
    out.push(0x01);

    if data.is_empty() {
        // Empty STORE block with BFINAL=1
        out.extend_from_slice(&[0x01, 0x00, 0x00, 0xFF, 0xFF]);
    } else {
        let max_block = 65535usize;
        let mut offset = 0;
        while offset < data.len() {
            let remaining = data.len() - offset;
            let block_len = remaining.min(max_block);
            let is_final = offset + block_len >= data.len();
            out.push(if is_final { 0x01 } else { 0x00 });
            let len16 = block_len as u16;
            out.extend_from_slice(&len16.to_le_bytes());
            out.extend_from_slice(&(!len16).to_le_bytes());
            out.extend_from_slice(&data[offset..offset + block_len]);
            offset += block_len;
        }
    }

    let adler = adler32(data);
    out.extend_from_slice(&adler.to_be_bytes());
    out
}

/// Incremental CRC-32 update (call repeatedly, XOR with 0xFFFFFFFF at end).
fn crc32_update(mut crc: u32, data: &[u8]) -> u32 {
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB8_8320;
            } else {
                crc >>= 1;
            }
        }
    }
    crc
}

fn adler32(data: &[u8]) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for &byte in data {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::FRAMES;
    use crate::theme::Theme;

    #[test]
    fn base64_rfc_vectors() {
        let cases: &[(&[u8], &str)] = &[
            (b"", ""),
            (b"f", "Zg=="),
            (b"fo", "Zm8="),
            (b"foo", "Zm9v"),
            (b"foob", "Zm9vYg=="),
            (b"fooba", "Zm9vYmE="),
            (b"foobar", "Zm9vYmFy"),
        ];
        for &(input, expected) in cases {
            let mut out = Vec::new();
            base64_encode(input, &mut out);
            assert_eq!(
                std::str::from_utf8(&out).unwrap(),
                expected,
                "base64 mismatch for {:?}",
                std::str::from_utf8(input).unwrap()
            );
        }
    }

    #[test]
    fn rasterise_dimensions_scale_1() {
        let theme = Theme::classic();
        let (rgba, w, h) = rasterise(&FRAMES[0], &theme, 1);
        assert_eq!(w, CROP_W as u32);
        assert_eq!(h, CROP_H as u32);
        assert_eq!(rgba.len(), (w as usize) * (h as usize) * 4);
    }

    #[test]
    fn rasterise_dimensions_scale_2() {
        let theme = Theme::classic();
        let (rgba, w, h) = rasterise(&FRAMES[0], &theme, 2);
        assert_eq!(w, (CROP_W * 2) as u32);
        assert_eq!(h, (CROP_H * 2) as u32);
        assert_eq!(rgba.len(), (w as usize) * (h as usize) * 4);
    }

    #[test]
    fn rasterise_clamps_excessive_scale() {
        let theme = Theme::classic();
        let (rgba, w, h) = rasterise(&FRAMES[0], &theme, 255);
        assert_eq!(w, (CROP_W * MAX_SCALE as usize) as u32);
        assert_eq!(h, (CROP_H * MAX_SCALE as usize) as u32);
        assert_eq!(rgba.len(), (w as usize) * (h as usize) * 4);
    }

    #[test]
    fn rasterise_transparent_pixel_is_zero_alpha() {
        let theme = Theme::classic();
        let (rgba, _w, _h) = rasterise(&FRAMES[0], &theme, 1);
        // Row 0 is entirely transparent
        for x in 0..CROP_W {
            let idx = x * 4;
            assert_eq!(rgba[idx + 3], 0, "pixel ({x}, 0) should be transparent");
        }
    }

    #[test]
    fn render_png_valid_header() {
        let data = render_png(&FRAMES[0], &Theme::classic(), 1);
        assert_eq!(&data[..8], &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]);
    }

    #[test]
    fn render_emoji_valid_png() {
        let data = render_emoji(&FRAMES[0], &Theme::classic(), 128);
        assert_eq!(&data[..8], &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]);
        assert!(data.len() > 100);
    }

    #[test]
    fn render_all_frames_count() {
        let frames = render_all_frames(&Theme::classic(), 1);
        assert_eq!(frames.len(), crate::FRAME_COUNT);
    }

    #[test]
    fn render_all_emoji_count() {
        let emojis = render_all_emoji(&Theme::classic(), 64);
        assert_eq!(emojis.len(), crate::FRAME_COUNT);
    }

    #[test]
    fn emoji_different_themes() {
        let classic = render_emoji(&FRAMES[0], &Theme::classic(), 64);
        let ocean = render_emoji(&FRAMES[0], &Theme::preset("ocean").unwrap(), 64);
        assert_ne!(classic, ocean);
    }

    #[test]
    fn empty_zlib_is_valid() {
        // zlib_store with empty input should produce a valid stream
        let out = zlib_store(&[]);
        assert_eq!(out[0], 0x78); // CMF
        assert_eq!(out[1], 0x01); // FLG
        assert_eq!(out[2], 0x01); // BFINAL=1, BTYPE=00
    }
}
