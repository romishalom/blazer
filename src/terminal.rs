//! Terminal utilities: querying the size of the terminal window and
//! detecting whether output likely supports color.

/// Returns the current terminal size as `(columns, rows)`, if it can be
/// determined.
///
/// Returns `None` when stdout is not a terminal (e.g. piped to a file) or
/// when the size cannot be queried on the current platform, after falling
/// back to the `COLUMNS`/`LINES` environment variables.
///
/// # Examples
///
/// ```no_run
/// use blazer::terminal::size;
///
/// if let Some((cols, rows)) = size() {
///     println!("Terminal is {}x{}", cols, rows);
/// }
/// ```
pub fn size() -> Option<(u16, u16)> {
    #[cfg(unix)]
    {
        unix_size()
    }
    #[cfg(windows)]
    {
        windows_size()
    }
    #[cfg(not(any(unix, windows)))]
    {
        env_fallback()
    }
}

#[cfg(unix)]
fn unix_size() -> Option<(u16, u16)> {
    use std::mem::MaybeUninit;

    #[repr(C)]
    struct Winsize {
        ws_row: libc::c_ushort,
        ws_col: libc::c_ushort,
        ws_xpixel: libc::c_ushort,
        ws_ypixel: libc::c_ushort,
    }

    unsafe {
        let mut ws: MaybeUninit<Winsize> = MaybeUninit::uninit();
        let res = libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, ws.as_mut_ptr());
        if res == 0 {
            let ws = ws.assume_init();
            if ws.ws_col > 0 && ws.ws_row > 0 {
                return Some((ws.ws_col, ws.ws_row));
            }
        }
    }
    env_fallback()
}

#[cfg(windows)]
fn windows_size() -> Option<(u16, u16)> {
    // Minimal, manual FFI bindings to avoid pulling in a heavy dependency.
    #[repr(C)]
    struct Coord {
        x: i16,
        y: i16,
    }

    #[repr(C)]
    struct SmallRect {
        left: i16,
        top: i16,
        right: i16,
        bottom: i16,
    }

    #[repr(C)]
    struct ConsoleScreenBufferInfo {
        size: Coord,
        cursor_position: Coord,
        attributes: u16,
        window: SmallRect,
        maximum_window_size: Coord,
    }

    #[link(name = "kernel32")]
    extern "system" {
        fn GetStdHandle(n_std_handle: i32) -> isize;
        fn GetConsoleScreenBufferInfo(
            h_console_output: isize,
            lp_console_screen_buffer_info: *mut ConsoleScreenBufferInfo,
        ) -> i32;
    }

    const STD_OUTPUT_HANDLE: i32 = -11;

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle == 0 || handle == -1 {
            return env_fallback();
        }
        let mut info: ConsoleScreenBufferInfo = std::mem::zeroed();
        if GetConsoleScreenBufferInfo(handle, &mut info) != 0 {
            let cols = (info.window.right - info.window.left + 1).max(0) as u16;
            let rows = (info.window.bottom - info.window.top + 1).max(0) as u16;
            if cols > 0 && rows > 0 {
                return Some((cols, rows));
            }
        }
    }
    env_fallback()
}

/// Falls back to the `COLUMNS` and `LINES` environment variables.
#[allow(dead_code)]
fn env_fallback() -> Option<(u16, u16)> {
    let cols = std::env::var("COLUMNS").ok()?.parse().ok()?;
    let rows = std::env::var("LINES").ok()?.parse().ok()?;
    Some((cols, rows))
}

/// Returns `true` if the current environment likely supports ANSI color codes.
///
/// This checks the `NO_COLOR` and `TERM` environment variables as a basic
/// heuristic (respecting the [NO_COLOR](https://no-color.org) convention).
/// It does not perform a full TTY check.
///
/// # Examples
///
/// ```
/// use blazer::terminal::supports_color;
///
/// if supports_color() {
///     println!("Colors are probably supported!");
/// }
/// ```
pub fn supports_color() -> bool {
    if std::env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if let Ok(term) = std::env::var("TERM") {
        if term == "dumb" {
            return false;
        }
    }
    true
}