
#[derive(Clone, PartialEq, Eq, Debug, Copy)]
pub enum PlayerColor {
    Red,
    Yellow,
}

impl std::fmt::Display for PlayerColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self == &PlayerColor::Red { "red" } else { "yellow" })
    }
}
