//! Print frame 0 in every built-in theme preset.
//!
//! ```sh
//! cargo run --example themes
//! ```

use std::io::Write;

use scampii::{Renderer, Theme, FRAMES};

fn main() {
    let mut renderer = Renderer::new();
    let mut stdout = std::io::stdout();

    for &name in Theme::PRESET_NAMES {
        let theme = Theme::preset(name).expect("preset exists");

        let mut buf: Vec<u8> = Vec::new();
        renderer
            .draw(&mut buf, &FRAMES[0], &theme)
            .expect("failed to render frame");

        println!("--- {name} ---");
        stdout.write_all(&buf).expect("failed to write");
    }
}
