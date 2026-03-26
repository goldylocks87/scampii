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
  lib.rs       -- Crate root, Animation (pre-cached inline PNG frames)
  main.rs      -- CLI (clap, optional via "cli" feature)
  pixel.rs     -- Hue enum (16 palette slots), packed pixel encoding
  frame.rs     -- Sprite data (3 animation frames, 32x26 raw / 24x26 cropped)
  theme.rs     -- Named presets and HSB hue-shifting
  color.rs     -- RGB/HSB math, hex and OSC color parsing
  png.rs       -- PNG encoder, rasteriser, base64, emoji export
  terminal.rs  -- RAII raw-mode guard, terminal foreground color query
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
