mod pve_game;

use pve_game::engine::Engine;

fn main() {
    
    let board_dim = (7, 6);

    let mut engine = Engine::new(board_dim.0, board_dim.1, 10);

    pve_game::start_game(board_dim.0, board_dim.1, &mut engine);

}
