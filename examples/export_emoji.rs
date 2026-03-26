//! Export a 128x128 emoji PNG.
//!
//! Run with: cargo run --example export_emoji

fn main() {
    let theme = scampii::Theme::preset("ocean").unwrap();
    let data = scampii::png::render_emoji(&scampii::FRAMES[0], &theme, 128);
    std::fs::write("scampii_ocean.png", &data).unwrap();
    println!("Wrote scampii_ocean.png (128x128)");
}
