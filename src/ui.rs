//! User Interface module for terminal-based rendering and interaction.

use crate::get_terminal_size;
use std::io::Write;

/// Main UI structure handling terminal rendering and user interaction.
pub struct UI {
    width: u16,
    height: u16,
}

/// Available content types for the UI system.
pub enum Content {
    /// Main menu display with title and options
    MainMenu,
    /// Empty content, showing only the frame
    Empty,
}

impl UI {
    /// Creates a new UI instance with current terminal dimensions.
    ///
    /// # Panics
    /// When terminal size cannot be determined.
    pub fn new() -> Self {
        let (width, height) = get_terminal_size().unwrap();
        UI { width, height }
    }

    /// Draws the initial frame that remains constant throughout the application.
    ///
    /// Creates a box using Unicode box-drawing characters that fills the terminal.
    pub fn draw_frame(&self) {
        let horizontal_line = "─".repeat(self.width as usize - 2);

        println!("┌{}┐", horizontal_line);

        let empty_line = format!("│{}│", " ".repeat(self.width as usize - 2));
        for _ in 1..self.height - 1 {
            println!("{}", empty_line);
        }

        print!("└{}┘", horizontal_line);
        std::io::stdout().flush().unwrap();
    }

    /// Updates the content within the frame based on the provided content type.
    ///
    /// # Arguments
    /// * `content` - The type of content to display
    pub fn update_content(&self, content: Content) {
        match content {
            Content::MainMenu => self.draw_main_menu(),
            Content::Empty => {
                for row in 2..self.height - 1 {
                    print!(
                        "\x1B[{};{}H│{}│",
                        row,
                        1,
                        " ".repeat(self.width as usize - 2)
                    );
                }
                std::io::stdout().flush().unwrap();
            }
        }
        std::io::stdout().flush().unwrap();
    }

    /// Shows a centered dialog message for a specified duration.
    ///
    /// # Arguments
    /// * `message` - The message to display in the dialog
    pub fn show_dialog(&self, message: &str) {
        let col = (self.width as usize / 2) - (message.len() / 2);
        let row = self.height / 2;

        print!("\x1B[{};{}H{}", row, col, message);
        std::io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    /// Draws the main menu content with ASCII art title and options.
    ///
    /// Displays:
    /// - ASCII art title
    /// - Version number
    /// - Menu options
    fn draw_main_menu(&self) {
        let title = [
            r" ____  _   _ ____ _______   __",
            r"|  _ \| | | / ___|_   _\ \ / /",
            r"| |_) | | | \___ \ | |  \ V / ",
            r"|  _ <| |_| |___) || |   | |  ",
            r"|_| \_\\___/|____/ |_|   |_|  ",
        ];

        let content_start_row = self.height / 2 - 5;

        for (i, line) in title.iter().enumerate() {
            let padding = (self.width as usize - line.len()) / 2;
            print!("\x1B[{};{}H", content_start_row + i as u16, 1);
            print!(
                "│{}{}{}│",
                " ".repeat(padding),
                line,
                " ".repeat(self.width as usize - padding - line.len() - 2)
            );
        }

        let version = format!("v{}", env!("CARGO_PKG_VERSION"));

        let menu_items = [
            "C R A W L E R",
            &version,
            "",
            "1. New Game",
            "2. Load Game",
            "3. Exit",
        ];

        for (i, item) in menu_items.iter().enumerate() {
            let padding = (self.width as usize - item.len()) / 2;
            print!(
                "\x1B[{};{}H",
                content_start_row + title.len() as u16 + i as u16,
                1
            );
            print!(
                "│{}{}{}│",
                " ".repeat(padding),
                item,
                " ".repeat(self.width as usize - padding - item.len() - 2)
            );
        }

        let input_row = self.height - 2;
        let input_col = self.width / 2;
        print!("\x1B[{};{}H> ", input_row, input_col);
    }

    /// Gets user input from the current cursor position.
    ///
    /// # Returns
    /// A trimmed string containing the user's input.
    pub fn get_input(&self) -> String {
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}

impl Drop for UI {
    /// Ensures proper cleanup of terminal state on UI destruction.
    ///
    /// - Waits briefly to show final messages
    /// - Clears the screen
    /// - Restores cursor visibility
    fn drop(&mut self) {
        std::thread::sleep(std::time::Duration::from_secs(1));
        print!("\x1B[2J\x1B[H");
        print!("\x1B[?25h");
        std::io::stdout().flush().unwrap();
    }
}

impl Default for UI {
    /// Provides default initialization for UI struct.
    fn default() -> Self {
        Self::new()
    }
}
