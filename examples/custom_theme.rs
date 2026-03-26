//! Create a custom-themed shrimp animation.
//!
//! Run with: cargo run --example custom_theme

fn main() {
    let mut theme = scampii::Theme::from_color(0xFF, 0x00, 0x99);
    theme.set_color(scampii::Hue::Antenna, 0xFF, 0x80, 0xCC);
    theme.set_color(scampii::Hue::Leg, 0x99, 0x00, 0x55);

    let mut anim = scampii::Animation::new(theme);
    let mut out = std::io::stdout();

    // Animate for 5 seconds (50 frames at 100ms each)
    for _ in 0..50 {
        anim.draw(&mut out).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
