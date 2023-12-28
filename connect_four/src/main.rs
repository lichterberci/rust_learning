mod pve_game;

use pve_game::engine::Engine;

fn main() {
    
    let engine = Engine::new();

    pve_game::start_game(5, 6, &engine);

}
