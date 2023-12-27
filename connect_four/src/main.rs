use connect_four::board::{Board, PlayerColor};

fn main() {

    let mut board = Board::new(4, 5);

    board.draw_to_console();

    println!();
    
    board.play_move(0, PlayerColor::Red);
    
    board.draw_to_console();

    println!();
    
    board.play_move(0, PlayerColor::Yellow);
    
    board.draw_to_console();

    println!();

}
