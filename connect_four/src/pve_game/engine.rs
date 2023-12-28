use connect_four::board::{Board, GameState, PlayerColor};

#[derive(Debug, Clone)]
/// A minimax engine that can play the best moves in any situation.
pub struct Engine {}

impl Engine {
    pub fn new() -> Self { Self {  } }

    /// Plays an engine move.
    ///
    /// # Errors
    /// 
    /// This function will return an error if there are no legal moves.
    pub fn play_move(&self, board: &Board, color: PlayerColor) -> Result<usize, String> {
        if board.get_state() != GameState::Ongoing {
            return Err(String::from("Board is in a terminal state!"));
        }

        let mut board: Board = board.copy();

        let mut best_value = i32::MIN;
        let mut best_column = 0;

        let mut alpha = i32::MIN;
        let beta = i32::MAX;

        let other_color = if color == PlayerColor::Red {
            PlayerColor::Yellow
        } else {
            PlayerColor::Red
        };

        for column in board.get_available_columns() {
            board.make_move(column, color.clone());
            let value = -negamax(&mut board, -beta, -alpha, other_color.clone());
            board.unmake_move(column);

            if best_value < value {
                best_value = value;
                best_column = column;
            };

            alpha = alpha.max(best_value);
        }

        Ok(best_column)
    }
}

/// Implements the negamax algorithm for determining the best move
fn negamax(board: &mut Board, beta: i32, alpha: i32, color: PlayerColor) -> i32 {
    let mut alpha = alpha;

    let current_state = board.get_state();

    if current_state != GameState::Ongoing {
        let eval = match current_state {
            GameState::Draw => 0,
            GameState::Win(win_color) => {
                if win_color == PlayerColor::Red {
                    i32::MAX
                } else {
                    i32::MIN
                }
            }
            GameState::Ongoing => 0,
        };

        let color_multiplier = if color == PlayerColor::Red { 1 } else { -1 };

        return color_multiplier * eval;
    }

    let mut best_value = i32::MIN;

    let other_color = if color == PlayerColor::Red {
        PlayerColor::Yellow
    } else {
        PlayerColor::Red
    };

    for column in board.get_available_columns() {
        board.make_move(column, color);
        let value = -negamax(board, -beta, -alpha, other_color.clone());
        board.unmake_move(column);

        best_value = best_value.max(value);

        alpha = alpha.max(best_value);

        if alpha >= beta {
            break;
        }
    }

    best_value
}
