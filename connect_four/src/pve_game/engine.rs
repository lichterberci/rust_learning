mod zobrist_hash;

use std::{collections::HashMap, ops::Sub};

use connect_four::board::{Board, GameState, PlayerColor};

use self::zobrist_hash::{HashType, ZobristHash};

const RED_WIN_EVAL: i32 = 100;
const YELLOW_WIN_EVAL: i32 = -100;

#[derive(Debug, Clone)]
/// A minimax engine that can play the best moves in any situation.
pub struct Engine {
    pub max_depth: usize,
    hash_generator: ZobristHash,
    transposition_table: HashMap<HashType, i32>,
}

impl Engine {
    pub fn new(board_width: usize, board_height: usize, max_depth: usize) -> Self {
        Self {
            max_depth,
            hash_generator: ZobristHash::new(board_width, board_height),
            transposition_table: HashMap::new(),
        }
    }

    /// Plays an engine move.
    ///
    /// # Errors
    ///
    /// This function will return an error if there are no legal moves.
    pub fn get_best_move(&mut self, board: &Board, color: PlayerColor) -> Result<usize, String> {
        if board.get_state() != GameState::Ongoing {
            return Err(String::from("Board is in a terminal state!"));
        }

        self.transposition_table.clear();

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

        self.transposition_table.clear();

        Ok(best_column)
    }

    /// Implements the negamax algorithm for determining the best move
    fn minimax(
        &mut self,
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

        let position_hash = self.hash_generator.calculate_hash(board);

        if self.transposition_table.contains_key(&position_hash) {
            return self.transposition_table[&position_hash];
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

        self.transposition_table.insert(position_hash, best_value);

        best_value
    }

    fn static_eval(&self, board: &Board) -> i32 {
        let mut result = 0;

        let longest_red_streak = board.get_longest_streak_for_color(&PlayerColor::Red) as i32;
        let longest_yellow_streak = board.get_longest_streak_for_color(&PlayerColor::Yellow) as i32;

        result += 100 * longest_red_streak;
        result -= 100 * longest_yellow_streak;

        let column_scores = (0..board.width)
            .map(|c| -c.abs_diff(board.width / 2))
            .collect::<Vec<_>>();

        let red_column_penalties = (0..board.width * board.heigth)
            .filter(|index| {
                board
                    .get_at_index(index)
                    .is_some_and(|cell| cell.is_some_and(|color| color == PlayerColor::Red))
            })
            .map(|index| board.index_to_column(index))
            .map(|column| column_scores[column])
            .sum();

        let yellow_column_penalties = (0..board.width * board.heigth)
            .filter(|index| {
                board
                    .get_at_index(index)
                    .is_some_and(|cell| cell.is_some_and(|color| color == PlayerColor::Yellow))
            })
            .map(|index| board.index_to_column(index))
            .map(|column| column_scores[column])
            .sum();

        result += red_column_penalties;
        result -= yellow_column_penalties;

        result
    }
}
