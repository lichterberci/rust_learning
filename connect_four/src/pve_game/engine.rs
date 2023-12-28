use connect_four::board::{Board, GameState, PlayerColor};

const RED_WIN_EVAL: i32 = 100;
const YELLOW_WIN_EVAL: i32 = -100;

#[derive(Debug, Clone)]
/// A minimax engine that can play the best moves in any situation.
pub struct Engine {
    pub max_depth: usize,
}

impl Engine {
    pub fn new() -> Self {
        Self { max_depth: 15 }
    }

    /// Plays an engine move.
    ///
    /// # Errors
    ///
    /// This function will return an error if there are no legal moves.
    pub fn get_best_move(&self, board: &Board, color: PlayerColor) -> Result<usize, String> {
        if board.get_state() != GameState::Ongoing {
            return Err(String::from("Board is in a terminal state!"));
        }

        let mut board: Board = board.copy();

        let mut best_value = if color == PlayerColor::Red {
            -10000
        } else {
            10000
        };
        let mut best_column = 0;

        let mut alpha = -10000;
        let mut beta = 10000;

        let other_color = if color == PlayerColor::Red {
            PlayerColor::Yellow
        } else {
            PlayerColor::Red
        };

        for column in board.get_available_columns() {
            board.make_move(column, color.clone());
            let value = self.minimax(&mut board, alpha, beta, other_color.clone(), 1);
            board.unmake_move(column);

            println!("\tcolumn {column} --> {value}");

            if color == PlayerColor::Red {
                if value > best_value {
                    best_value = value;
                    best_column = column;
                }
                alpha = alpha.max(best_value);
            } else {
                if value < best_value {
                    best_value = value;
                    best_column = column;
                }
                beta = beta.min(best_value);
            }
        }

        Ok(best_column)
    }

    /// Implements the negamax algorithm for determining the best move
    fn minimax(
        &self,
        board: &mut Board,
        alpha: i32,
        beta: i32,
        color: PlayerColor,
        depth: usize,
    ) -> i32 {
        let mut alpha = alpha;
        let mut beta = beta;

        let current_state = board.get_state();

        if current_state != GameState::Ongoing {
            return match current_state {
                GameState::Win(win_color) => {
                    if win_color == PlayerColor::Red {
                        RED_WIN_EVAL
                    } else {
                        YELLOW_WIN_EVAL
                    }
                }
                GameState::Draw | GameState::Ongoing => 0,
            };
        }

        if depth >= self.max_depth {
            return self.static_eval(board);
        }

        let mut best_value = if color == PlayerColor::Red {
            -10000
        } else {
            10000
        };

        let other_color = if color == PlayerColor::Red {
            PlayerColor::Yellow
        } else {
            PlayerColor::Red
        };

        for column in board.get_available_columns() {
            board.make_move(column, color);
            let value = self.minimax(board, alpha, beta, other_color.clone(), depth + 1);
            board.unmake_move(column);

            if color == PlayerColor::Red {
                best_value = best_value.max(value);
                alpha = alpha.max(best_value);
            } else {
                best_value = best_value.min(value);
                beta = beta.min(best_value);
            }

            if alpha >= beta {
                break;
            }
        }

        best_value
    }

    fn static_eval(&self, board: &Board) -> i32 {
        board.get_longest_streak_for_color(&PlayerColor::Red) as i32
            - board.get_longest_streak_for_color(&PlayerColor::Yellow) as i32
    }
}
