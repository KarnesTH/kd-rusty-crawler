use kd_rusty_crawler::UI;

fn main() {
    let ui = UI::new();

    loop {
        ui.draw_menu();

        match ui.get_input().as_str() {
            "1" => {
                println!("Starting new game...");
            }
            "2" => {
                println!("Loading game...");
            }
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid input! Please select a number between 1 and 3.");
            }
        }
    }
}
