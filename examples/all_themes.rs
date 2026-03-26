//! Print one frame of every built-in theme.
//!
//! Run with: cargo run --example all_themes

fn main() {
    let mut out = std::io::stdout();
    for &name in scampii::Theme::PRESET_NAMES {
        let theme = scampii::Theme::preset(name).unwrap();
        let mut anim = scampii::Animation::new(theme);
        print!("{name:>10}: ");
        anim.draw(&mut out).unwrap();
        println!();
    }
}
