mod libs;
use libs::{game_manager::GameManager, helper::clear_console};

// Game loop:
//  1. load stack cards
//  2. load hand cards
//
//  3. give the best card to play
//  4. read played cards
//  5. go to 3.

fn main() {
    clear_console();
    let mut gm = GameManager::new_game();
    gm.load_start_info();
    gm.start_game_loop();
}
