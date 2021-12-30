use std::collections::HashMap;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    row: i8,
    col: i8,
}

const BOARD_SIZE: i8 = 8;

impl Position {
    pub fn new(row: i8, col: i8) -> Position {
        Position { row, col }
    }

    pub fn index(id: usize) -> Position {
        Position::new(
            (id / BOARD_SIZE as usize) as i8,
            (id % BOARD_SIZE as usize) as i8,
        )
    }

    ///
    ///
    ///
    pub fn is_in_range(&self) -> bool {
        let not_negative: bool = self.row >= 0 && self.col >= 0;
        let not_exceeds: bool = self.row < BOARD_SIZE && self.col < BOARD_SIZE;
        not_exceeds && not_negative
    }

    ///
    ///
    ///
    pub fn get_row(&self) -> i8 {
        self.row
    }

    ///
    ///
    ///
    pub fn get_col(&self) -> i8 {
        self.col
    }

    ///
    ///
    ///
    pub fn to_index(&self) -> usize {
        (BOARD_SIZE * self.row + self.col) as usize
    }

    ///
    ///
    ///
    pub fn to_string(&self) -> String {
        position_to_algebraic(self)
    }
}

fn get_notation_map() -> HashMap<char, char> {
    // todo memoize this
    let mut notation_map = HashMap::new();
    let numbers = "12345678".chars(); // iterator
    let algebraic_cols: Vec<char> = "abcdefgh".chars().collect(); // Vec

    for (i, ch) in numbers.enumerate() {
        notation_map.insert(algebraic_cols[i], ch);
        notation_map.insert(ch, algebraic_cols[i]);
    }
    notation_map
}

pub fn position_to_algebraic(position: &Position) -> String {
    let convert = get_notation_map();

    let mut out = String::new();

    let col_num = (b'0' + position.get_col() as u8 + 1) as char;
    let col_char: char = *convert.get(&col_num).unwrap(); // potentially bad
    let row_char = (b'0' + BOARD_SIZE as u8 - position.get_row() as u8) as char;

    out.push(col_char);
    out.push(row_char);

    out
}

pub fn algebraic_to_position(algebraic: &String) -> Position {
    println!("{}", algebraic);
    let convert = get_notation_map();
    let col_char = algebraic.chars().next().unwrap();
    let col_digit = convert.get(&col_char).unwrap().to_digit(10).unwrap();
    let col_for_pos = col_digit - 1;

    let row_digit = algebraic.chars().nth(1).unwrap().to_digit(10).unwrap();
    let row_for_pos = 8 - row_digit;

    Position::new(row_for_pos as i8, col_for_pos as i8)
}

impl Add for Position {
    type Output = Self;
    fn add(self, other: Position) -> Position {
        Position {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}
impl Sub for Position {
    type Output = Self;
    fn sub(self, other: Position) -> Position {
        Position {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}
impl Mul for Position {
    type Output = Self;
    fn mul(self, other: Position) -> Position {
        Position {
            row: self.row * other.row,
            col: self.col * other.col,
        }
    }
}
impl AddAssign for Position {
    fn add_assign(&mut self, other: Position) {
        self.row += other.row;
        self.col += other.col;
    }
}
impl SubAssign for Position {
    fn sub_assign(&mut self, other: Position) {
        self.row -= other.row;
        self.col -= other.col;
    }
}
impl MulAssign for Position {
    fn mul_assign(&mut self, other: Position) {
        self.row *= other.row;
        self.col *= other.col;
    }
}
