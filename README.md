<div align="center">

<img src="examples/scampii.gif" alt="scampii" width="400">

# scampii

An animated pixel-art shrimp for your terminal.

[![Crates.io](https://img.shields.io/crates/v/scampii)](https://crates.io/crates/scampii)
[![docs.rs](https://docs.rs/scampii/badge.svg)](https://docs.rs/scampii)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/scampii)](LICENSE-MIT)
[![MSRV](https://img.shields.io/badge/MSRV-1.85-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0.html)

> Zero-dependency animated pixel art. One command, instant shrimp.
> Works in iTerm2, VS Code, WezTerm, Ghostty, and Kitty.

</div>

## Quickstart

```bash
cargo install scampii && scampii
```

Press any key to exit.

## Usage

```bash
scampii              # classic orange, fullscreen animation
scampii ocean        # blue
scampii barbie       # hot pink
scampii ff00ff       # any hex color
scampii auto         # match terminal foreground color
scampii --inline     # print one frame inline (acts like an emoji)
```

### Export PNG

```bash
scampii --emoji 128           # single frame, 128x128 emoji PNG
scampii --emoji 128 ocean     # any theme
scampii --png                 # all 3 animation frames
scampii --png --emoji 128     # all 3 frames as emoji PNGs
```

## Library

```rust
let mut anim = scampii::Animation::new(scampii::Theme::classic());
let mut out = std::io::stdout();

// Draw 30 frames (~3 seconds of animation)
for _ in 0..30 {
    anim.draw(&mut out).unwrap(); // zero-alloc: frames are pre-cached
    std::thread::sleep(std::time::Duration::from_millis(100));
}
```

RGB tuples work too:

```rust
let mut anim = scampii::Animation::new((0xFF, 0x00, 0x99));
```

Custom theme:

```rust
let mut theme = scampii::Theme::from_color(0xFF, 0x00, 0x99);
theme.set_color(scampii::Hue::Antenna, 0xFF, 0x80, 0xCC);
let mut anim = scampii::Animation::new(theme);
```

Export a PNG:

```rust
let data = scampii::png::render_emoji(
    &scampii::FRAMES[0],
    &scampii::Theme::classic(),
    128,
);
std::fs::write("scampii.png", &data).unwrap();
```

## Feature flags

| Flag  | Default | Description |
|-------|---------|-------------|
| `cli` | Yes     | Builds the `scampii` binary (adds `clap` dependency) |

To use as a library without clap:

```toml
[dependencies]
scampii = { version = "0.1", default-features = false }
```

## Themes

| Name       | Hex       |
| ---------- | --------- |
| `classic`  | `#E8732A` |
| `ocean`    | `#2090DD` |
| `forest`   | `#30A840` |
| `neon`     | `#FF00FF` |
| `gold`     | `#FFB43C` |
| `ice`      | `#88CCFF` |
| `lava`     | `#FF3300` |
| `midnight` | `#663399` |
| `barbie`   | `#E0218A` |

Or pass any hex color: `scampii c0ffee`

## Terminals

Renders via the iTerm2 inline image protocol (OSC 1337):

- iTerm2
- VS Code / Cursor
- WezTerm
- Ghostty
- Kitty

## Environment

| Variable        | Effect              |
| --------------- | ------------------- |
| `SCAMPII_COLOR` | Default color/theme |

## Requirements

- **Rust 1.85+** (edition 2024)
- True-color terminal with iTerm2 inline image support (OSC 1337)
- macOS or Linux

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT OR Apache-2.0
