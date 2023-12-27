use connect_four::board::{Board, PlayerColor, GameState};

fn main() {
    
    let mut board = Board::new(4, 4);
    
    board.draw_to_console();
    
    println!("{:?}", board.get_state());
    println!();
    
    board.play_move(0, PlayerColor::Yellow);
    board.play_move(1, PlayerColor::Yellow);
    board.play_move(1, PlayerColor::Red);
    board.play_move(2, PlayerColor::Yellow);
    board.play_move(2, PlayerColor::Yellow);
    board.play_move(2, PlayerColor::Red);
    board.play_move(3, PlayerColor::Yellow);
    board.play_move(3, PlayerColor::Yellow);
    board.play_move(3, PlayerColor::Yellow);
    board.play_move(3, PlayerColor::Red);
    
    board.draw_to_console();
    println!("{:?}", board.get_state());


}
