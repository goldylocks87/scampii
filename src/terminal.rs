//! Terminal utilities: raw-mode RAII guard and OSC foreground color query.
//!
//! The [`RawModeGuard`] ensures raw mode and the alternate screen are always
//! cleaned up, even on panics. The [`query_terminal_fg`] function probes the
//! terminal for its current foreground color via the OSC 10 escape sequence.

use std::io::{stdout, Read, Write};

use crossterm::{cursor, execute, terminal};

use crate::color::parse_osc_color;

// ---------------------------------------------------------------------------
// Raw-mode RAII guard
// ---------------------------------------------------------------------------

/// RAII guard that restores the terminal on drop.
///
/// When this value is dropped it:
/// 1. Shows the cursor.
/// 2. Leaves the alternate screen.
/// 3. Disables raw mode.
///
/// This runs on all exit paths including panics and early `?` returns.
#[derive(Debug)]
pub struct RawModeGuard {
    _private: (),
}

impl RawModeGuard {
    /// Create a guard for the standard alternate-screen mode.
    pub fn alternate_screen() -> Self {
        Self { _private: () }
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = execute!(stdout(), cursor::Show, terminal::LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}

// ---------------------------------------------------------------------------
// Termios wrapper
// ---------------------------------------------------------------------------

/// Saved POSIX termios state for a file descriptor.
///
/// Wraps the unsafe `libc::tcgetattr` / `tcsetattr` pair in a safe API
/// that restores the original state on request.
struct SavedTermios {
    fd: i32,
    orig: libc::termios,
}

impl SavedTermios {
    /// Save the current termios state for `fd`.
    ///
    /// Returns `None` if `tcgetattr` fails.
    fn save(fd: i32) -> Option<Self> {
        // SAFETY: zeroed termios is a valid initial state for tcgetattr to
        // populate, and we check the return value before using it.
        let orig = unsafe {
            let mut t = std::mem::zeroed::<libc::termios>();
            if libc::tcgetattr(fd, &mut t) != 0 {
                return None;
            }
            t
        };
        Some(Self { fd, orig })
    }

    /// Switch the file descriptor to a minimal raw mode suitable for reading
    /// an OSC response with a 200 ms timeout.
    fn enter_raw(&self) -> Result<(), std::io::Error> {
        // SAFETY: we only modify a copy of a valid termios struct and apply
        // it to the same fd it came from.
        unsafe {
            let mut raw = self.orig;
            libc::cfmakeraw(&mut raw);
            raw.c_cc[libc::VMIN] = 0;
            raw.c_cc[libc::VTIME] = 2; // 200 ms timeout in deciseconds
            if libc::tcsetattr(self.fd, libc::TCSANOW, &raw) != 0 {
                return Err(std::io::Error::last_os_error());
            }
        }
        Ok(())
    }

    /// Restore the saved termios state.
    fn restore(&self) -> Result<(), std::io::Error> {
        // SAFETY: restoring the original termios state we saved earlier.
        unsafe {
            if libc::tcsetattr(self.fd, libc::TCSANOW, &self.orig) != 0 {
                return Err(std::io::Error::last_os_error());
            }
        }
        Ok(())
    }
}

impl Drop for SavedTermios {
    fn drop(&mut self) {
        // Best-effort restore on all exit paths (including early returns/panics).
        let _ = self.restore();
    }
}

// ---------------------------------------------------------------------------
// Foreground color query
// ---------------------------------------------------------------------------

/// Query the terminal's foreground color via OSC 10.
///
/// Opens `/dev/tty` directly to avoid conflicting with crossterm's stdin
/// reader. **Must be called before** crossterm enters raw mode.
///
/// Returns `None` if the terminal does not respond or the response cannot be
/// parsed.
pub fn query_terminal_fg() -> Option<(u8, u8, u8)> {
    use std::os::unix::io::AsRawFd;

    let mut tty = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/tty")
        .ok()?;

    let fd = tty.as_raw_fd();
    let saved = SavedTermios::save(fd)?;
    saved.enter_raw().ok()?;

    // Send OSC 10 query (BEL terminator for widest compatibility)
    tty.write_all(b"\x1b]10;?\x07").ok()?;
    tty.flush().ok()?;

    // Read response
    let mut buf = [0u8; 64];
    let mut len = 0;
    while len < buf.len() {
        match tty.read(&mut buf[len..]) {
            Ok(0) => break,
            Ok(n) => {
                len += n;
                if buf[..len].contains(&0x07) || buf[..len].windows(2).any(|w| w == b"\x1b\\") {
                    break;
                }
            }
            Err(_) => break,
        }
    }

    // `saved` restores termios on drop (including early returns above).
    drop(saved);

    let response = std::str::from_utf8(&buf[..len]).ok()?;
    parse_osc_color(response)
}
