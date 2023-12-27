mod game_state;
mod player_color;

pub use game_state::GameState;
pub use player_color::PlayerColor;

const WINNING_STREAK_LENGTH: i32 = 4;
/// Represents a board with the coins tossed into it.
///
/// The data is indexed so that the 0 column is the leftmost one
/// and the 0 row is the bottom most one.
pub struct Board {
    width: usize,
    height: usize,
    data: Vec<Option<PlayerColor>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            width,
            height,
            data: vec![None; width * height],
        }
    }

    fn get_at_pos(&self, column: usize, row: usize) -> &Option<PlayerColor> {
        &self.data[row * self.width + column]
    }

    fn set_at_pos(&mut self, column: usize, row: usize, value: Option<PlayerColor>) {
        self.data[row * self.width + column] = value;
    }

    pub fn can_play_move(&self, column: usize) -> bool {
        self.get_at_pos(column, self.height - 1).is_none()
    }

    pub fn play_move(&mut self, column: usize, player_color: PlayerColor) -> bool {
        if !self.can_play_move(column) {
            return false;
        }

        for i in 0..self.height {
            if self.get_at_pos(column, i).is_none() {
                self.set_at_pos(column, i, Some(player_color));
                break;
            }
        }

        true
    }

    pub fn get_state(&self) -> GameState {
        
        // checking the rows
        for row in 0..self.height {
            let mut last_cell: Option<PlayerColor> = None;
            let mut current_streak = 0;

            for col in 0..self.width {
                let current_cell = self.get_at_pos(col, row);

                if current_cell.is_none() {
                    current_streak += 1;
                    last_cell = match current_cell {
                        Some(color) => Some(color.clone()),
                        None => None,
                    };
                    continue;
                }

                let current_color = current_cell.clone().unwrap();

                match last_cell {
                    Some(ref last_color) if *last_color == current_color => {
                        current_streak += 1;

                        if current_streak >= WINNING_STREAK_LENGTH {
                            return GameState::Win(current_color);
                        }
                    }
                    _ => {
                        current_streak = 1;
                    }
                }

                last_cell = match current_cell {
                    Some(color) => Some(color.clone()),
                    None => None,
                };
            }
        }

        // checking columns
        for col in 0..self.width {
            let mut last_cell: Option<PlayerColor> = None;
            let mut current_streak = 0;

            for row in 0..self.height {
                let current_cell = self.get_at_pos(col, row);

                if current_cell.is_none() {
                    current_streak += 1;
                    last_cell = match current_cell {
                        Some(color) => Some(color.clone()),
                        None => None,
                    };
                    continue;
                }

                let current_color = current_cell.clone().unwrap();

                match last_cell {
                    Some(ref last_color) if *last_color == current_color => {
                        current_streak += 1;

                        if current_streak >= WINNING_STREAK_LENGTH {
                            return GameState::Win(current_color);
                        }
                    }
                    _ => {
                        current_streak = 1;
                    }
                }

                last_cell = match current_cell {
                    Some(color) => Some(color.clone()),
                    None => None,
                };
            }
        }

        // checking diagonals
        for col_of_diagonal_start in -(self.width as i64)..self.width as i64 {
            let mut last_cell: Option<PlayerColor> = None;
            let mut current_streak = 0;

            // diagonals from the bottom left to the top right

            for index_from_the_bottom in 0..self.width {
                let col = col_of_diagonal_start + index_from_the_bottom as i64;
                let row = index_from_the_bottom as i64;

                if col < 0 || col >= self.width as i64 || row < 0 || row >= self.height as i64 {
                    last_cell = None;
                    current_streak = 0;
                    continue;
                }

                let current_cell = self.get_at_pos(col as usize, row as usize);

                if current_cell.is_none() {
                    current_streak += 1;
                    last_cell = match current_cell {
                        Some(color) => Some(color.clone()),
                        None => None,
                    };
                    continue;
                }

                let current_color = current_cell.clone().unwrap();

                match last_cell {
                    Some(ref last_color) if *last_color == current_color => {
                        current_streak += 1;

                        if current_streak >= WINNING_STREAK_LENGTH {
                            return GameState::Win(current_color);
                        }
                    }
                    _ => {
                        current_streak = 1;
                    }
                }

                last_cell = match current_cell {
                    Some(color) => Some(color.clone()),
                    None => None,
                };
            }

            // diagonals from the bottom right to the top left

            last_cell = None;
            current_streak = 0;

            for index_from_the_bottom in 0..self.width {
                let col = col_of_diagonal_start - index_from_the_bottom as i64;
                let row = index_from_the_bottom as i64;

                if col < 0 || col >= self.width as i64 || row < 0 || row >= self.height as i64 {
                    last_cell = None;
                    current_streak = 0;
                    continue;
                }

                let current_cell = self.get_at_pos(col as usize, row as usize);

                if current_cell.is_none() {
                    current_streak += 1;
                    last_cell = match current_cell {
                        Some(color) => Some(color.clone()),
                        None => None,
                    };
                    continue;
                }

                let current_color = current_cell.clone().unwrap();

                match last_cell {
                    Some(ref last_color) if *last_color == current_color => {
                        current_streak += 1;

                        if current_streak >= WINNING_STREAK_LENGTH {
                            return GameState::Win(current_color);
                        }
                    }
                    _ => {
                        current_streak = 1;
                    }
                }

                last_cell = match current_cell {
                    Some(color) => Some(color.clone()),
                    None => None,
                };
            }
        }

        
        if self.data.iter().all(|cell| cell.is_some()) {
            GameState::Draw
        } else {
            GameState::Ongoing
        }
    }

    pub fn draw_to_console(&self) {
        let row_outline = (0..=self.width).map(|_| "+").collect::<Vec<_>>().join("-");

        println!("{}", row_outline);

        for row in (0..self.height).rev() {
            let mut displayed_row = String::from("|");

            for col in 0..self.width {
                displayed_row += match self.get_at_pos(col, row) {
                    Some(col) => match col {
                        PlayerColor::Red => "#",
                        PlayerColor::Yellow => "O",
                    },
                    None => " ",
                };
                displayed_row += "|";
            }

            println!("{}", displayed_row);
            println!("{}", row_outline);
        }
    }
}
