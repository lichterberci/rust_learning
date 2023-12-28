mod pve_game;

use pve_game::engine::Engine;

fn main() {
    
    let board_dim = (5, 5);

    let mut engine = Engine::new(board_dim.0, board_dim.1, 20);

    pve_game::start_game(board_dim.0, board_dim.1, &mut engine);

}
