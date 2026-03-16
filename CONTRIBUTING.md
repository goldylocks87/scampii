# Contributing to scampii

Thanks for your interest in contributing. This document covers how to build,
test, and extend the crate.

## Building and Testing

```bash
# Build
cargo build

# Run the animation
cargo run --release

# Run all tests
cargo test

# Lint (must pass with zero warnings)
cargo clippy -- -D warnings

# Format
cargo fmt
```

All PRs must pass `cargo fmt --check`, `cargo clippy -- -D warnings`, and
`cargo test` before merge.

## Code Structure

```
src/
  lib.rs       -- Crate root, public re-exports (Hue, unpack_pixel, renderers)
  main.rs      -- CLI (clap, optional via "cli" feature), protocol auto-detection
  pixel.rs     -- [pub(crate)] Hue enum (16 palette slots), packed pixel encoding
  frame.rs     -- Sprite data (3 animation frames), halfblock Renderer
  theme.rs     -- Named presets and HSB hue-shifting
  color.rs     -- [pub(crate)] RGB/HSB math, hex and OSC color parsing
  raster.rs    -- [pub(crate)] Frame-to-RGBA rasteriser, base64 encoder
  iterm.rs     -- iTerm2 inline image protocol (ItermRenderer, PNG encoder)
  kitty.rs     -- Kitty graphics protocol (KittyRenderer, chunked base64 RGBA)
  sixel.rs     -- DEC Sixel protocol (SixelRenderer, RLE-compressed bands)
  terminal.rs  -- RAII raw-mode guard, terminal foreground query
  error.rs     -- ScampiiError
```

## How to Add a New Color Theme

Color themes are defined in `src/theme.rs` in the `Theme::preset` method.

1. Pick a name and an RGB color that represents the theme.
2. Add a match arm in `Theme::preset`:

   ```rust
   "mytheme" => Some(Self::from_color(0xRR, 0xGG, 0xBB)),
   ```

3. Add the name to `Theme::PRESET_NAMES`.
4. Update the CLI help text in `src/main.rs` (the `color` argument doc comment).
5. Run `cargo test` to verify the new preset passes the existing test that
   checks every name in `PRESET_NAMES` resolves to `Some`.

That is it. `from_color` handles the HSB hue rotation automatically.

## How to Add a New Rendering Protocol

1. Create a new module `src/myprotocol.rs`.
2. Implement a free function and a struct wrapper (see `iterm.rs` for the pattern):

   ```rust
   pub fn draw_myprotocol<Out: Write>(
       buf: &mut Vec<u8>,    // reusable scratch buffer
       out: &mut Out,        // output sink
       frame: &PackedFrame,  // sprite frame data
       theme: &Theme,        // resolved color LUT
       scale: u8,            // pixel scale factor
   ) -> Result<(), ScampiiError>

   pub struct MyProtocolRenderer { buf: Vec<u8> }
   // impl with fn draw(...) that delegates to draw_myprotocol
   ```

3. Use `raster::rasterise` if you need RGBA pixel data, or iterate the frame
   directly with `unpack_pixel` for text-based protocols.
4. Register the module in `src/lib.rs` (`pub mod myprotocol;`) and add a
   re-export for the renderer struct.
5. Add a variant to the `Protocol` enum in `src/main.rs` and wire it into the
   `detect_protocol` function and the animation loop.
6. Add tests. At minimum: a smoke test that renders frame 0 and checks the
   output is non-empty and starts/ends with the expected escape sequences.

## PR Guidelines

- Run `cargo fmt` before committing.
- Run `cargo clippy -- -D warnings` and fix all warnings.
- Run `cargo test` and ensure all tests pass.
- Keep commits focused. One logical change per commit.
- Do not introduce new dependencies without discussion. If it can be done in
  20 lines of code, do it in 20 lines of code.

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
Be respectful, constructive, and welcoming.
