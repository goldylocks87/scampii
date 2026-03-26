# Changelog

## 0.1.2 — 2026-03-26

- Simplified to single rendering path: inline PNG via iTerm2 protocol (OSC 1337)
- Pre-cached animation frames for zero-allocation `draw()` hot loop
- Added `--inline` flag for single-frame emoji output
- Added `--emoji` and `--png` flags for PNG export
- Removed Kitty, Sixel, and halfblock renderers
- Removed `thiserror` dependency
- Tightened public API surface (`pub(crate)` for internal modules)

## 0.1.0 — 2026-03-15

- Initial release
- 3-frame animated scampii sprite (24x26 visible pixels, hand-crafted pixel art)
- 9 color theme presets + hex color + auto-detect terminal foreground
- Library API for embedding in TUI applications
