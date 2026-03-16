//! CLI entry point for the scampii terminal animation.

use std::io::{stdout, BufWriter};
use std::time::Duration;

use clap::Parser;
use crossterm::{cursor, execute, terminal};

use scampii::{
    detect_protocol, parse_hex_color, query_terminal_fg, ItermRenderer, KittyRenderer, Protocol,
    RawModeGuard, Renderer, ScampiiError, SixelRenderer, Theme, FRAMES,
};

/// Default frame delay in milliseconds.
const FRAME_DELAY_MS: u64 = 100;

/// Default pixel scale factor.
const DEFAULT_SCALE: u8 = 4;

/// An animated pixel-art scampii for your terminal.
///
/// Supports pixel-perfect rendering via iTerm2, Kitty, and Sixel image
/// protocols, with a Unicode halfblock fallback for any terminal.
/// Press any key to exit.
///
/// Examples:
///   scampii                  Default orange scampii
///   scampii ocean            Blue ocean theme
///   scampii ff00ff           Any hex color
///   scampii auto             Match terminal foreground
///   scampii -p kitty         Force Kitty protocol
///   scampii --inline ocean   Render in scrollback
#[derive(Parser, Debug)]
#[command(
    name = "scampii",
    version,
    about,
    after_help = "Press any key to exit the animation."
)]
struct Cli {
    /// Color theme or hex color.
    ///
    /// Accepts a preset name (classic, ocean, forest, neon, gold, ice, lava,
    /// midnight), a hex color (ff6600, #e8732a), or "auto" to detect the
    /// terminal's foreground color.
    #[arg(default_value = "classic", env = "SCAMPII_COLOR", value_name = "COLOR")]
    color: String,

    /// Pixel scale factor for image protocols (each sprite pixel = NxN screen pixels).
    ///
    /// At scale=1 the image is only 24x26 pixels. Default is 4.
    /// Has no effect in halfblock mode.
    #[arg(short, long, default_value_t = DEFAULT_SCALE)]
    scale: u8,

    /// Force a specific rendering protocol instead of auto-detecting.
    ///
    /// Values: iterm, kitty, sixel, halfblock.
    #[arg(short, long)]
    protocol: Option<Protocol>,

    /// Render in normal scrollback instead of the alternate screen.
    ///
    /// Useful for embedding in shell prompts, pipelines, or TUI panels.
    #[arg(short, long)]
    inline: bool,
}

/// Resolve the CLI color argument into a [`Theme`].
fn resolve_theme(arg: &str) -> Theme {
    if arg.eq_ignore_ascii_case("auto") {
        let (r, g, b) = query_terminal_fg().unwrap_or((220, 110, 40));
        return Theme::from_color(r, g, b);
    }
    if let Some(preset) = Theme::preset(&arg.to_lowercase()) {
        return preset;
    }
    let (r, g, b) = parse_hex_color(arg).unwrap_or((220, 110, 40));
    Theme::from_color(r, g, b)
}

fn run() -> Result<(), ScampiiError> {
    let cli = Cli::parse();

    // Auto-detect must happen BEFORE crossterm takes over stdin.
    let theme = resolve_theme(&cli.color);
    let protocol = cli.protocol.unwrap_or_else(detect_protocol);
    let scale = cli.scale.max(1); // clamp to at least 1

    let raw_stdout = stdout();
    let mut stdout = BufWriter::new(raw_stdout.lock());
    terminal::enable_raw_mode()?;

    let _guard = if cli.inline {
        execute!(stdout, cursor::Hide)?;
        RawModeGuard::inline()
    } else {
        execute!(
            stdout,
            terminal::EnterAlternateScreen,
            cursor::Hide,
            terminal::Clear(terminal::ClearType::All),
        )?;
        RawModeGuard::alternate_screen()
    };

    let mut halfblock = Renderer::new();
    let mut iterm = ItermRenderer::new();
    let mut kitty = KittyRenderer::new();
    let mut sixel = SixelRenderer::new();
    let mut frame_idx = 0;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        match protocol {
            Protocol::Iterm => {
                let _ = iterm.draw(&mut stdout, &FRAMES[frame_idx], &theme, scale);
            }
            Protocol::Kitty => {
                let _ = kitty.draw(&mut stdout, &FRAMES[frame_idx], &theme, scale);
            }
            Protocol::Sixel => {
                let _ = sixel.draw(&mut stdout, &FRAMES[frame_idx], &theme, scale);
            }
            Protocol::Halfblock => {
                let _ = halfblock.draw(&mut stdout, &FRAMES[frame_idx], &theme);
            }
        }

        frame_idx = (frame_idx + 1) % FRAMES.len();

        if crossterm::event::poll(Duration::from_millis(FRAME_DELAY_MS))? {
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
