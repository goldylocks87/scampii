//! Render a single frame as a pixel-perfect image.
//!
//! Auto-detects your terminal's image protocol. In unsupported terminals
//! you'll get the halfblock fallback.
//!
//! ```sh
//! cargo run --example iterm_image
//! ```

fn main() {
    let mut anim = scampii::Animation::new(scampii::Theme::classic()).scale(4);
    let mut out = std::io::stdout();
    anim.draw(&mut out).expect("failed to render");
    println!();
}
