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
        let mut data = Vec::<Option<PlayerColor>>::new();

        data.reserve_exact(width * height);

        Board {
            width,
            height,
            data,
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


        GameState::Draw
    }
}
