use super::player_color::PlayerColor;

#[derive(PartialEq, Eq, Debug)]
pub enum GameState {
    Ongoing,
    Draw,
    Win(PlayerColor),
}
