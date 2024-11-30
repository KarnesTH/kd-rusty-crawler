#[cfg(unix)]
fn get_terminal_size() -> (u16, u16) {
    use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
    let mut ws = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    if unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut ws) } == 0 {
        (ws.ws_col, ws.ws_row)
    } else {
        (80, 24)
    }
}

#[cfg(windows)]
fn get_terminal_size() -> (u16, u16) {
    use winapi::um::winbase::GetStdHandle;
    use winapi::um::winbase::STD_OUTPUT_HANDLE;
    use winapi::um::wincon::{GetConsoleScreenBufferInfo, CONSOLE_SCREEN_BUFFER_INFO};

    let handle = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
    let mut info = CONSOLE_SCREEN_BUFFER_INFO::default();

    if unsafe { GetConsoleScreenBufferInfo(handle, &mut info) } != 0 {
        let width = (info.srWindow.Right - info.srWindow.Left + 1) as u16;
        let height = (info.srWindow.Bottom - info.srWindow.Top + 1) as u16;
        (width, height)
    } else {
        (80, 24)
    }
}

pub struct UI {
    width: u16,
    height: u16,
}

impl UI {
    pub fn new() -> Self {
        let (width, height) = get_terminal_size();
        UI { width, height }
    }

    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[H");
    }

    pub fn draw_menu(&self) {
        self.clear_screen();

        let horizontal_line = "─".repeat(self.width as usize - 2);

        println!("┌{}┐", horizontal_line);

        let empty_line = format!("│{}│", " ".repeat(self.width as usize - 2));

        let vertical_padding = (self.height - 10) / 2;

        for _ in 0..vertical_padding {
            println!("{}", empty_line);
        }

        self.draw_title();
        println!("{}", empty_line);
        self.draw_menu_options();

        for _ in 0..2 {
            println!("{}", empty_line);
        }

        println!("└{}┘", horizontal_line);
    }

    fn draw_title(&self) {
        let title = vec![
            r" ____  _   _ ____ _______   __",
            r"|  _ \| | | / ___|_   _\ \ / /",
            r"| |_) | | | \___ \ | |  \ V / ",
            r"|  _ <| |_| |___) || |   | |  ",
            r"|_| \_\\___/|____/ |_|   |_|  ",
        ];

        let max_title_width = title.iter().map(|line| line.len()).max().unwrap_or(0);
        let left_padding = (self.width as usize - max_title_width - 2) / 2;

        for line in title {
            println!(
                "│{}{}{}│",
                " ".repeat(left_padding),
                line,
                " ".repeat(self.width as usize - left_padding - line.len() - 2)
            );
        }

        self.draw_centered_text("C R A W L E R");
        self.draw_centered_text(&format!("v{}", env!("CARGO_PKG_VERSION")));
    }

    fn draw_menu_options(&self) {
        self.draw_centered_text("1. New Game");
        self.draw_centered_text("2. Load Game");
        self.draw_centered_text("3. Exit");
    }

    fn draw_centered_text(&self, text: &str) {
        let padding = (self.width as usize - text.len() - 2) / 2;
        println!(
            "│{}{}{}│",
            " ".repeat(padding),
            text,
            " ".repeat(self.width as usize - padding - text.len() - 2)
        );
    }

    pub fn get_input(&self) -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}
