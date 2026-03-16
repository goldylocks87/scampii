//! Render a single scampii frame to stdout.
//!
//! This is the simplest possible use of the scampii library.
//!
//! ```sh
//! cargo run --example single_frame
//! ```

fn main() {
    let mut anim = scampii::Animation::new(scampii::Theme::classic());
    let mut out = std::io::stdout();
    anim.draw(&mut out).expect("failed to render frame");
}
