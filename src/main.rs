use kd_rusty_crawler::{Game, UI};

fn main() {
    let ui = UI::new();
    let mut game = None;
    ui.hide_cursor();

    loop {
        ui.clear_screen();
        if let Some(ref mut g) = game {
            ui.clear_screen();
            ui.draw_game(g);
            let input = ui.get_game_input();
            ui.clear_screen();

            match input.as_str() {
                "q" => {
                    game = None;
                }
                "i" => {
                    println!("Inventory");
                }
                _ => {}
            }
        } else {
            ui.clear_screen();
            ui.draw_menu();
            let input = ui.get_menu_input();
            ui.clear_screen();

            match input.as_str() {
                "1" => {
                    let player_name = ui.get_player_name();
                    game = Some(Game::new(player_name));
                }
                "2" => {
                    println!("Loading game...");
                }
                "3" => {
                    ui.clear_screen();
                    ui.show_cursor();
                    println!("Goodbye!");
                    break;
                }
                _ => {
                    println!("Invalid input! Please select a number between 1 and 3.");
                }
            }
        }
    }
    ui.show_cursor();
}
