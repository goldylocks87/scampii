//! Create a custom theme in 3 lines.
//!
//! Run with: cargo run --example custom_theme

fn main() {
    let mut anim = scampii::Animation::new(scampii::Theme::from_color(0xFF, 0x00, 0x99));
    anim.theme_mut()
        .set_color(scampii::Hue::Antenna, 0xFF, 0x80, 0xCC);
    anim.theme_mut()
        .set_color(scampii::Hue::Leg, 0x99, 0x00, 0x55);

    let mut out = std::io::stdout();
    anim.draw(&mut out).unwrap();
}
