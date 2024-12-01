use crate::{Game, Player};
use std::io::Write;

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

    pub fn clear_screen(&self) {
        print!("\x1B[2J\x1B[H");
        std::io::stdout().flush().unwrap();
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

    pub fn draw_game(&self, game: &Game) {
        self.clear_screen();

        let map_width = ((self.width as usize - 6) * 2) / 3;
        let stats_width = (self.width as usize - 6) - map_width - 3;
        let stats_rows = 9;

        println!("┌{}┐", "─".repeat(self.width as usize - 2));

        println!(
            "│ ╔{}╗ ╔{}╗ │",
            "═".repeat(map_width),
            "═".repeat(stats_width)
        );

        let content_height = stats_rows.min(self.height as usize);
        for i in 0..content_height {
            print!("│ ║{}║ ║", " ".repeat(map_width));
            self.draw_stats_row(&game.player, i, stats_width);
            println!("║ │");
        }

        println!(
            "│ ╚{}╝ ╚{}╝ │",
            "═".repeat(map_width),
            "═".repeat(stats_width)
        );

        println!(
            "│ {}│",
            "Messages:".to_string().pad_right(self.width as usize - 3)
        );
        println!("│{}│", " ".repeat(self.width as usize - 2));

        println!(
            "│ {}│",
            "Command: _".to_string().pad_right(self.width as usize - 3)
        );

        println!("└{}┘", "─".repeat(self.width as usize - 2));
    }

    fn draw_map_row(&self, row: usize, width: usize) {
        print!("{}", ".".repeat(width));
    }

    fn draw_stats_row(&self, player: &Player, row: usize, width: usize) {
        let stats = match row {
            0 => " Stats:".to_string(),
            1 => format!("    HP: {}/100", player.health),
            2 => format!("    Level: {}", player.level),
            3 => format!(
                "    XP: {}/{}",
                player.experience, player.experience_to_next_level
            ),
            4 => format!("    ATK: {}", player.attack),
            5 => format!("    DEF: {}", player.defense),
            6 => " Equipment:".to_string(),
            7 => format!(
                "    Weapon: {}",
                player.equipped_weapon.as_ref().map_or("None", |w| &w.name)
            ),
            8 => format!(
                "    Armor: {}",
                player.equipped_armor.as_ref().map_or("None", |a| &a.name)
            ),
            _ => String::new(),
        };

        if !stats.is_empty() {
            print!("{}", stats.pad_right(width));
        } else {
            print!("{}", " ".repeat(width));
        }
    }

    pub fn show_cursor(&self) {
        print!("\x1B[?25h");
        std::io::stdout().flush().unwrap();
    }

    pub fn hide_cursor(&self) {
        print!("\x1B[?25l");
        std::io::stdout().flush().unwrap();
    }

    pub fn get_input_at(&self, row: u16, col: u16) -> String {
        print!("\x1B[{};{}H", row, col);
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    pub fn get_menu_input(&self) -> String {
        let input_row = self.height - 2;
        let input_col = (self.width / 2) as u16;

        self.get_input_at(input_row, input_col)
    }

    pub fn get_player_name(&self) -> String {
        self.clear_screen();
        self.draw_name_input_screen();

        let name_prompt = "Name: ";
        let total_width = name_prompt.len() + 20;
        let start_pos = (self.width as usize - total_width) / 2;
        let cursor_pos = start_pos + name_prompt.len() + 1;
        let input_row = self.height / 2 + 1;

        self.get_input_at(input_row, cursor_pos as u16)
    }

    fn draw_name_input_screen(&self) {
        let horizontal_line = "─".repeat(self.width as usize - 2);
        println!("┌{}┐", horizontal_line);

        let empty_line = format!("│{}│", " ".repeat(self.width as usize - 2));

        for _ in 0..self.height / 3 {
            println!("{}", empty_line);
        }

        self.draw_centered_text("What should your hero be called?");
        println!("{}", empty_line);
        self.draw_centered_text("Name: ____________________");

        for _ in 0..self.height / 3 {
            println!("{}", empty_line);
        }

        println!("└{}┘", horizontal_line);
    }

    pub fn get_game_input(&self) -> String {
        let input_row = self.height - 2;
        let input_col = (self.width - 4) as u16;

        self.get_input_at(input_row, input_col)
    }
}

trait PadString {
    fn pad_right(&self, width: usize) -> String;
}

impl PadString for String {
    fn pad_right(&self, width: usize) -> String {
        if self.len() >= width {
            self.clone()
        } else {
            format!("{}{}", self, " ".repeat(width - self.len()))
        }
    }
}
