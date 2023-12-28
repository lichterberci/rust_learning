use connect_four::board::{Board, PlayerColor};
use rand::{self, Rng};

pub type HashType = u128;

#[derive(Debug, Clone)]
pub struct ZobristHash {
    yellow_position_hashes: Vec<HashType>,
    red_position_hashes: Vec<HashType>,
}

impl ZobristHash {
    pub fn new(board_width: usize, board_height: usize) -> Self {
        Self {
            yellow_position_hashes: (0..(board_width * board_height))
                .map(|_| rand::thread_rng().gen::<HashType>())
                .collect(),
            red_position_hashes: (0..(board_width * board_height))
                .map(|_| rand::thread_rng().gen::<HashType>())
                .collect(),
        }
    }

    pub fn calculate_hash(&self, board: &Board) -> HashType {
        let yellow_hash = (0..self.yellow_position_hashes.len())
            .filter(|idx| {
                board
                    .get_at_index(*idx)
                    .is_some_and(|color| color.is_some_and(|color| color == PlayerColor::Yellow))
            })
            .map(|idx| self.yellow_position_hashes[idx])
            .fold(0, |acc, x| acc ^ x);

        let red_hash = (0..self.yellow_position_hashes.len())
            .filter(|idx| {
                board
                    .get_at_index(*idx)
                    .is_some_and(|color| color.is_some_and(|color| color == PlayerColor::Red))
            })
            .map(|idx| self.red_position_hashes[idx])
            .fold(0, |acc, x| acc ^ x);

        yellow_hash ^ red_hash
    }
}
