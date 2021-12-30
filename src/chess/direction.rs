use super::position::Position;

/// Eight cardinal Directions
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    East,
    NorthEast,
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
}

/// Get the opposite cardinal direction
#[allow(dead_code)]
pub fn get_opposite(direction: Direction) -> Direction {
    match direction {
        Direction::East => Direction::West,
        Direction::NorthEast => Direction::SouthWest,
        Direction::North => Direction::South,
        Direction::NorthWest => Direction::SouthEast,
        Direction::West => Direction::East,
        Direction::SouthWest => Direction::NorthEast,
        Direction::South => Direction::North,
        Direction::SouthEast => Direction::NorthWest,
    }
}

/// (0, 0) is the NW corner
/// (7, 7) is the SW corner
/// White goes South->North
/// Black goes North->South
pub fn shift(direction: Direction) -> Position {
    match direction {
        Direction::East => Position::new(0, 1),
        Direction::NorthEast => Position::new(-1, 1),
        Direction::North => Position::new(-1, 0),
        Direction::NorthWest => Position::new(-1, -1),
        Direction::West => Position::new(0, -1),
        Direction::SouthWest => Position::new(1, -1),
        Direction::South => Position::new(1, 0),
        Direction::SouthEast => Position::new(1, 1),
    }
}

pub fn get_direction(position: &Position) -> Option<Direction> {
    let row = position.get_row();
    let col = position.get_col();
    let is_vertical = col == 0;
    let is_horizontal = row == 0;
    let is_diagonal = row.abs() == col.abs();
    let is_row_positive = row > 0;
    let is_col_positive = col > 0;
    if is_horizontal {
        return match is_col_positive {
            // Horizontal
            false => Option::Some(Direction::West),
            true => Option::Some(Direction::East),
        };
    } else if is_vertical {
        return match is_row_positive {
            // Vertical
            false => Option::Some(Direction::North),
            true => Option::Some(Direction::South),
        };
    } else if is_diagonal {
        return match (is_row_positive, is_col_positive) {
            (false, false) => Option::Some(Direction::NorthWest),
            (false, true) => Option::Some(Direction::NorthEast),
            (true, false) => Option::Some(Direction::SouthWest),
            (true, true) => Option::Some(Direction::SouthEast),
        };
    }
    Option::None
}
