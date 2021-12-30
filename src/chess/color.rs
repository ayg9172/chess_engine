#[derive(Copy, Clone, PartialEq, Debug)]

/// Represents piece colors in Chess
pub enum Color {
    White,
    Black,
}

impl Color {
    //! Get the opponent's color
    //! White -> Black
    //! Black -> White
    pub fn get_opposite(self) -> Color {
        if self == Color::White {
            Color::Black
        } else {
            Color::White
        }
    }
}
