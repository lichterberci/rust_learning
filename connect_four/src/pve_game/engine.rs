use connect_four::board::{Board, PlayerColor};

#[derive(Debug, Clone)]
pub struct Engine {}

impl Engine {
    pub fn play_move(&self, board: &Board, color: PlayerColor) -> Result<i32, String> {
        if board.get_state() != GameState::Ongoing {
            return Err(String::from("Board is in a terminal state!"));
        }

        let mut board = board.clone();

        let mut best_value = i32::MIN;
        let mut best_column = 0;

        let mut alpha = i32::MIN;
        let mut beta = i32::MAX;

        let other_color = if color == PlayerColor::Red {
            PlayerColor::Yellow
        } else {
            PlayerColor::Red
        };

        for column in board.get_available_columns() {
            board.make_move(column, color);
            let value = negamax(&mut board, -beta, -alpha, other_color);
            board.unmake_move(column);
        }

        Ok(best_column)
    }
}

fn negamax(board: &mut Board, beta: i32, alpha: i32, color: PlayerColor) -> i32 {

}
