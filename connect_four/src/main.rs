mod pve_game;

use pve_game::engine::Engine;

fn main() {
    
    let engine = Engine {
        max_depth: 13,
    };

    pve_game::start_game(5, 5, &engine);

}
