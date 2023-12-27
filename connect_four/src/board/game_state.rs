use super::player_color::PlayerColor;

pub enum GameState {
    Draw,
    Win(PlayerColor),
}
