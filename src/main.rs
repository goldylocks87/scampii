//! CLI entry point for scampii.

use std::io::{stdout, BufWriter};
use std::time::Duration;

use clap::Parser;
use crossterm::{cursor, execute, terminal};

use scampii::{
    parse_hex_color, query_terminal_fg, RawModeGuard, ScampiiError, Theme, FRAMES,
};

/// An animated pixel-art shrimp for your terminal.
///
/// Examples:
///   scampii                  Default orange scampii
///   scampii ocean            Blue ocean theme
///   scampii ff00ff           Any hex color
///   scampii auto             Match terminal foreground
///   scampii --emoji 128      Export 128x128 PNG emoji
#[derive(Parser, Debug)]
#[command(name = "scampii", version, about)]
struct Cli {
    /// Color theme or hex color (classic, ocean, forest, neon, gold, ice, lava, midnight, barbie).
    #[arg(default_value = "classic", env = "SCAMPII_COLOR", value_name = "COLOR")]
    color: String,

    /// Render inline (no alternate screen).
    #[arg(short, long)]
    inline: bool,

    /// Export as a square emoji PNG instead of animating.
    #[arg(long, value_name = "SIZE")]
    emoji: Option<u32>,

    /// Export all 3 animation frames as PNGs.
    #[arg(long)]
    png: bool,

    /// Display scale in character cells (default: 2 wide x 1 tall).
    /// Higher values render a bigger shrimp.
    #[arg(short, long, default_value_t = 1)]
    scale: u8,
}

fn resolve_theme(arg: &str) -> Theme {
    if arg.eq_ignore_ascii_case("auto") {
        let (r, g, b) = query_terminal_fg().unwrap_or((220, 110, 40));
        return Theme::from_color(r, g, b);
    }
    if let Some(preset) = Theme::preset(&arg.to_lowercase()) {
        return preset;
    }
    if let Some((r, g, b)) = parse_hex_color(arg) {
        return Theme::from_color(r, g, b);
    }
    eprintln!(
        "scampii: unknown color '{arg}', using classic orange\n  hint: try a preset ({}) or hex color (e.g. ff00ff)",
        Theme::PRESET_NAMES.join(", ")
    );
    Theme::classic()
}

fn run() -> Result<(), ScampiiError> {
    let cli = Cli::parse();
    let theme = resolve_theme(&cli.color);

    // ── PNG export ───────────────────────────────────────────────────────
    if cli.png || cli.emoji.is_some() {
        if let Some(size) = cli.emoji {
            if cli.png {
                for (i, data) in scampii::png::render_all_emoji(&theme, size).iter().enumerate() {
                    let path = format!("scampii_emoji_{i}.png");
                    std::fs::write(&path, data).map_err(ScampiiError::Io)?;
                    eprintln!("{path} ({size}x{size})");
                }
            } else {
                let data = scampii::png::render_emoji(&FRAMES[0], &theme, size);
                std::fs::write("scampii_emoji.png", &data).map_err(ScampiiError::Io)?;
                eprintln!("scampii_emoji.png ({size}x{size})");
            }
        } else {
            for (i, data) in scampii::png::render_all_frames(&theme, 1).iter().enumerate() {
                let path = format!("scampii_{i}.png");
                std::fs::write(&path, data).map_err(ScampiiError::Io)?;
                eprintln!("{path}");
            }
        }
        return Ok(());
    }

    // ── Inline: print one frame and exit (acts like an emoji) ──────────
    if cli.inline {
        let mut out = BufWriter::new(stdout().lock());
        let mut anim = scampii::Animation::new(theme).scale(cli.scale);
        anim.draw(&mut out)?;
        return Ok(());
    }

    // ── Fullscreen animation ─────────────────────────────────────────────
    let raw_stdout = stdout();
    let mut stdout = BufWriter::new(raw_stdout.lock());
    terminal::enable_raw_mode()?;

    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All),
    )?;
    let _guard = RawModeGuard::alternate_screen();

    let mut anim = scampii::Animation::new(theme).scale(cli.scale);

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        anim.draw(&mut stdout)?;

        if crossterm::event::poll(Duration::from_millis(100))? {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(_) => break,
                crossterm::event::Event::Resize(..) => {
                    let _ = execute!(stdout, terminal::Clear(terminal::ClearType::All));
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("scampii: {e}");
        std::process::exit(1);
    }
}
