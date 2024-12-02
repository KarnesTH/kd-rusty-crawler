//! Terminal utility functions for cross-platform terminal size detection.

/// Gets the current terminal size (width, height) in characters.
///
/// # Platform-specific implementations
/// - Unix: Uses TIOCGWINSZ ioctl
/// - Windows: Uses GetConsoleScreenBufferInfo
///
/// # Returns
/// - Ok((width, height)): Terminal dimensions in characters
/// - Err: If terminal size cannot be determined
///
/// # Errors
/// - System call failures
/// - Invalid terminal dimensions (0 width or height)
#[cfg(any(unix, windows))]
pub fn get_terminal_size() -> Result<(u16, u16), std::io::Error> {
    #[cfg(unix)]
    {
        use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
        let mut ws = winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        if unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut ws) } == -1 {
            return Err(std::io::Error::last_os_error());
        }

        if ws.ws_col == 0 || ws.ws_row == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Invalid terminal size",
            ));
        }

        Ok((ws.ws_col, ws.ws_row))
    }

    #[cfg(windows)]
    {
        use winapi::um::errhandlingapi::GetLastError;
        use winapi::um::winbase::{GetStdHandle, STD_OUTPUT_HANDLE};
        use winapi::um::wincon::{GetConsoleScreenBufferInfo, CONSOLE_SCREEN_BUFFER_INFO};

        let handle = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
        if handle.is_null() {
            return Err(std::io::Error::last_os_error());
        }

        let mut info = CONSOLE_SCREEN_BUFFER_INFO::default();

        if unsafe { GetConsoleScreenBufferInfo(handle, &mut info) } == 0 {
            return Err(std::io::Error::last_os_error());
        }

        let width = (info.srWindow.Right - info.srWindow.Left + 1) as u16;
        let height = (info.srWindow.Bottom - info.srWindow.Top + 1) as u16;

        if width == 0 || height == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Invalid terminal size",
            ));
        }

        Ok((width, height))
    }
}

/// Gets the terminal size with a fallback to default values.
///
/// # Returns
/// - (width, height): Terminal dimensions in characters
/// - Defaults to (80, 24) if terminal size cannot be determined
pub fn get_terminal_size_or_default() -> (u16, u16) {
    get_terminal_size().unwrap_or((80, 24))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_size() {
        let (width, height) = get_terminal_size_or_default();
        assert!(width > 0);
        assert!(height > 0);
    }
}
