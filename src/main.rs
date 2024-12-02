use kd_rusty_crawler::{ui::Content, Game, Map, Room, UI};

/// Application state representing either the main menu or active game.
///
/// Uses `Box<Game>` to reduce memory footprint of the enum variants.
enum AppState {
    /// Main menu state
    Menu,
    /// Active game state with boxed Game instance
    InGame(Box<Game>),
}

fn main() {
    let ui = UI::new();
    let mut app_state = AppState::Menu;

    ui.draw_frame();
    ui.update_content(Content::MainMenu);

    loop {
        let input = ui.get_input();

        match &mut app_state {
            AppState::Menu => match input.as_str() {
                "1" => {
                    ui.update_content(Content::Empty);
                    ui.show_dialog("Enter your hero's name: ");
                    let player_name = ui.get_input();

                    let mut map = Map::new(40, 15);
                    map.create_room(Room::new(40, 15));

                    app_state = AppState::InGame(Box::new(Game::new(player_name, map)));
                    ui.update_content(Content::Empty);
                    if let AppState::InGame(game) = &app_state {
                        ui.update_content(Content::Game(game));
                    }
                }
                "2" => {
                    ui.update_content(Content::Empty);
                    ui.show_dialog("Load game not implemented yet!");
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    ui.update_content(Content::MainMenu);
                }
                "3" | "q" => {
                    ui.update_content(Content::Empty);
                    ui.show_dialog("Thanks for playing!");
                    break;
                }
                _ => {
                    ui.show_dialog("Please select 1-3");
                    ui.update_content(Content::MainMenu);
                }
            },
            AppState::InGame(game) => match input.as_str() {
                "q" => {
                    app_state = AppState::Menu;
                    ui.update_content(Content::Empty);
                    ui.update_content(Content::MainMenu);
                }
                _ => {
                    game.update();
                    ui.update_content(Content::Game(game));
                }
            },
        };
    }
}
