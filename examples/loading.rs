//! Polished loading animation — scampii on the left, status text on the right.
//!
//! Run with: cargo run --release --example loading

use std::io::{stdout, BufWriter, Write};
use std::sync::mpsc;
use std::time::Duration;

use scampii::{detect_protocol, Hue, Protocol, Renderer, Theme, FRAMES};

const STEPS: &[(&str, u64)] = &[
    ("Deveining the shrimp", 1200),
    ("Heating the butter", 1400),
    ("Mincing the garlic", 1000),
    ("Deglazing with white wine", 1600),
    ("Tossing in the linguine", 1200),
    ("Squeezing fresh lemon", 800),
    ("Garnishing with parsley", 600),
];

const SPINNER: &[&str] = &["\u{25cb}", "\u{25d4}", "\u{25d1}", "\u{25d5}"];

fn do_work(tx: mpsc::Sender<&'static str>) {
    for &(step, ms) in STEPS {
        tx.send(step).unwrap();
        std::thread::sleep(Duration::from_millis(ms));
    }
    tx.send("__done__").unwrap();
}

fn goto(out: &mut impl Write, row: usize, col: usize) {
    write!(out, "\x1b8").unwrap();
    if row > 0 {
        write!(out, "\x1b[{}B", row).unwrap();
    }
    write!(out, "\x1b[{}G\x1b[K", col).unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || do_work(tx));

    let mut theme = Theme::preset("ocean").unwrap();
    theme.set_color(Hue::Antenna, 60, 120, 180);
    theme.set_color(Hue::Leg, 50, 110, 170);

    let protocol = detect_protocol();
    let use_image = matches!(
        protocol,
        Protocol::Iterm | Protocol::Kitty | Protocol::Sixel
    );

    // The shrimp image at scale 1 is ~4 columns, halfblock is 24 columns.
    let text_col: usize = if use_image { 9 } else { 26 };
    let shrimp_offset: usize = 3;
    let total_rows: usize = shrimp_offset + if use_image { 8 } else { 14 };

    let mut out = BufWriter::new(stdout().lock());
    write!(out, "\x1b[?25l").unwrap();
    for _ in 0..total_rows {
        writeln!(out).unwrap();
    }
    write!(out, "\x1b[{}A\x1b7", total_rows).unwrap();
    out.flush().unwrap();

    // Use Animation for the shrimp, but render it at a specific position.
    let mut iterm = scampii::ItermRenderer::new();
    let mut halfblock = Renderer::new();
    let mut frame_idx: usize = 0;
    let mut tick: usize = 0;

    let mut completed: Vec<&str> = Vec::new();
    let mut current: &str = "";
    let mut done = false;

    while !done {
        while let Ok(msg) = rx.try_recv() {
            if msg == "__done__" {
                done = true;
            } else {
                if !current.is_empty() {
                    completed.push(current);
                }
                current = msg;
            }
        }

        // -- Shrimp on the left --
        write!(out, "\x1b8").unwrap();
        if shrimp_offset > 0 {
            write!(out, "\x1b[{}B", shrimp_offset).unwrap();
        }
        if use_image {
            write!(out, "\x1b[1G").unwrap();
            let _ = iterm.draw(&mut out, &FRAMES[frame_idx], &theme, 2);
        } else {
            let mut tmp = Vec::new();
            let _ = halfblock.draw(&mut tmp, &FRAMES[frame_idx], &theme);
            for (i, line) in String::from_utf8_lossy(&tmp).lines().enumerate() {
                write!(out, "\x1b8").unwrap();
                if shrimp_offset + i > 0 {
                    write!(out, "\x1b[{}B", shrimp_offset + i).unwrap();
                }
                write!(out, "\x1b[1G{line}").unwrap();
            }
        }

        // -- Title + tasks on the right --
        goto(&mut out, shrimp_offset, text_col);
        write!(out, "\x1b[1;38;2;255;180;60mCOOKING SHRIMP SCAMPII\x1b[0m").unwrap();

        let visible = if completed.len() >= 2 {
            &completed[completed.len() - 2..]
        } else {
            &completed[..]
        };

        let task_row = shrimp_offset + 1;
        for i in 0..3 {
            goto(&mut out, task_row + i, text_col);
            if i < 2 {
                let idx = if visible.len() >= 2 {
                    Some(i)
                } else if visible.len() == 1 && i == 1 {
                    Some(0)
                } else {
                    None
                };
                if let Some(idx) = idx {
                    write!(
                        out,
                        "\x1b[2m\x1b[32m\u{2713}\x1b[0m\x1b[2m {}\x1b[0m",
                        visible[idx]
                    )
                    .unwrap();
                }
            } else if !current.is_empty() {
                let s = SPINNER[tick % SPINNER.len()];
                write!(out, "\x1b[38;2;100;180;255m{s}\x1b[0m {current}").unwrap();
            }
        }

        out.flush().unwrap();
        frame_idx = (frame_idx + 1) % FRAMES.len();
        tick += 1;
        std::thread::sleep(Duration::from_millis(100));
    }

    // Clean exit.
    write!(out, "\x1b8").unwrap();
    for _ in 0..total_rows {
        write!(out, "\x1b[1G\x1b[K\n").unwrap();
    }
    write!(out, "\x1b8").unwrap();
    writeln!(out, "\x1b[1;32m\u{2713} Done\x1b[0m").unwrap();
    write!(out, "\x1b[?25h").unwrap();
    out.flush().unwrap();
}
