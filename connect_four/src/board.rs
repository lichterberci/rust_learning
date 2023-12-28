mod game_state;
mod player_color;

use std::fmt::Display;

use nu_ansi_term;

pub use game_state::GameState;
pub use player_color::PlayerColor;

pub type CellType = Option<PlayerColor>;

const WINNING_STREAK_LENGTH: i32 = 4;
/// Represents a board with the coins tossed into it.
///
/// The data is indexed so that the 0 column is the leftmost one
/// and the 0 row is the bottom most one.
pub struct Board {
    pub width: usize,
    pub height: usize,
    data: Vec<CellType>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data_as_string = self.get_display_string();
        write!(f, "{}", data_as_string)
    }
}


impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            width,
            height,
            data: vec![None; width * height],
        }
    }

    fn get_at_pos(&self, column: usize, row: usize) -> &CellType {
        &self.data[row * self.width + column]
    }

    fn set_at_pos(&mut self, column: usize, row: usize, value: CellType) {
        self.data[row * self.width + column] = value;
    }

    pub fn can_play_move(&self, column: usize) -> bool {
        self.get_at_pos(column, self.height - 1).is_none()
    }

    pub fn make_move(&mut self, column: usize, player_color: PlayerColor) -> bool {
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

    pub fn unmake_move(&mut self, column: usize) {
        for i in (0..self.height).rev() {
            if self.get_at_pos(column, i).is_some() {
                self.set_at_pos(column, i, None);
                break;
            }
        }
    }

    pub fn get_state(&self) -> GameState {
        // checking the rows
        for row in 0..self.height {
            let mut last_cell: CellType = None;
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
        for col_of_diagonal_start in -(self.width as i64)..(self.width * 2) as i64 {
            let mut last_cell: CellType = None;
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
        println!("{}", self.get_display_string());
    }

    pub fn get_available_columns(&self) -> Vec<usize> {
        (0..self.width).filter(|c| self.can_play_move(*c)).collect()
    }

    pub fn copy(&self) -> Board {
        Board {
            width: self.width,
            height: self.height,
            data: self
                .data
                .iter()
                .copied()
                .collect::<Vec<Option<PlayerColor>>>(),
        }
    }

    pub fn get_display_string(&self) -> String {
        let mut result = String::new();
    
        let row_outline = (0..=self.width).map(|_| "+").collect::<Vec<_>>().join("-");
    
        result.push_str(&row_outline);
        result.push('\n');
    
        for row in (0..self.height).rev() {
            let mut displayed_row = String::from("|");
    
            for col in 0..self.width {
                let red_coin = &format!("{}", nu_ansi_term::Color::Red.paint("O"));
                let yellow_coin = format!("{}", nu_ansi_term::Color::Yellow.paint("O"));
    
                displayed_row.push_str(match self.get_at_pos(col, row) {
                    Some(color) => match color {
                        PlayerColor::Red => &red_coin,
                        PlayerColor::Yellow => &yellow_coin,
                    },
                    None => " ",
                });
                displayed_row += "|";
            }
    
            result.push_str(&displayed_row);
            result.push('\n');
            result.push_str(&row_outline);
            result.push('\n');
        }
    
        result
    }

    pub fn get_longest_streak_for_color(&self, color: &PlayerColor) -> usize {

        let mut result = 0;

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

                        if current_streak >= result && current_color == *color {
                            result = current_streak;
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

                        if current_streak >= result && current_color == *color {
                            result = current_streak;
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
        for col_of_diagonal_start in -(self.width as i64)..(self.width * 2) as i64 {
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

                        if current_streak >= result && current_color == *color {
                            result = current_streak;
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

                        if current_streak >= result && current_color == *color {
                            result = current_streak;
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

        result
    }

    pub fn get_at_index(&self, index: usize) -> Option<CellType> {
        if index >= self.width * self.height {
            return None;
        }

        return Some(self.data[index]);
    }

    pub fn index_to_column(&self, index: i32) -> usize {
        index % self.width
    }

}
