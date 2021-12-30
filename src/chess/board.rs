/// TODO: Abstract Wrapping(UNIVERSE) - bool
/// TODO: Make King move gen more efficient?
/// TODO: Convert to templates?
/// TODO: Better encapsulation
/// TODO: revisit dead code format specifiers
use super::bitboard_util::{clear_bit, get_bit, mask, put_bit};
use super::color::Color;
use super::fen;
use super::piece::fen_to_piece;
use super::piece::piece_to_fen;
use super::piece::Piece;
use super::position::position_to_algebraic;
use super::position::{self, Position};

/// Constants for printing the board
const H_BOUNDARY: &str = "---------------------------------";
const H_BAR: &str = "|---|---|---|---|---|---|---|---|";
const LEFT: &str = "| ";
const RIGHT: &str = " ";
const END: &str = "|";
const EMPTY: char = ' ';

#[allow(dead_code)]
const BOARD_SIZE: u8 = 8;
const SQUARE_COUNT: u32 = 64;
const NEWLINE: char = '\n';

/// Human-readable indexing of bitboards 
/// using a C-like enum
/// E.g. A8->0, B8->1, ..., H1->63
#[allow(dead_code)] 
#[rustfmt::skip]
pub enum Square {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
}

/// Represents castling sides in Chess
/// Short is King-side, Long is Queen-side
pub enum Castle {
    Short,
    Long,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Board {
    pub pawns: u64,
    pub knights: u64,
    pub bishops: u64,
    pub rooks: u64,
    pub queens: u64,
    pub kings: u64,

    pub white_pieces: u64,
    pub black_pieces: u64,

    pub ep_target: u64,

    pub halfmove_clock: u16,
    pub fullmove_clock: u16,

    pub turn: Color,

    pub is_white_castle_short: bool,
    pub is_white_castle_long: bool,
    pub is_black_castle_short: bool,
    pub is_black_castle_long: bool,
}

impl Board {
    ///
    ///
    ///
    pub fn new(fen_string: &str) -> Board {
        // todo: add error checks!
        let split: Vec<&str> = fen_string.split_whitespace().collect();

        let mut pawns: u64 = 0;
        let mut knights: u64 = 0;
        let mut bishops: u64 = 0;
        let mut rooks: u64 = 0;
        let mut queens: u64 = 0;
        let mut kings: u64 = 0;
        let mut white_pieces: u64 = 0;
        let mut black_pieces: u64 = 0;
        let mut ep_target: u64 = 0;

        let mut row = 0;
        let mut col = 0;
        for ch in split[0].chars() {
            if ch == fen::ROW_END {
                row += 1;
                col = 0;
            } else if ch.to_digit(10) != None {
                col += ch.to_digit(10).unwrap() as i8;
            } else {
                let result = fen_to_piece(ch);
                let piece: Piece = result.0;
                let color: Color = result.1;

                match piece {
                    Piece::Pawn => pawns = put_bit(pawns, Position::new(row, col).to_index()),
                    Piece::Knight => knights = put_bit(knights, Position::new(row, col).to_index()),
                    Piece::Bishop => bishops = put_bit(bishops, Position::new(row, col).to_index()),
                    Piece::Rook => rooks = put_bit(rooks, Position::new(row, col).to_index()),
                    Piece::Queen => queens = put_bit(queens, Position::new(row, col).to_index()),
                    Piece::King => kings = put_bit(kings, Position::new(row, col).to_index()),
                };

                match color {
                    Color::White => {
                        white_pieces = put_bit(white_pieces, Position::new(row, col).to_index())
                    }
                    Color::Black => {
                        black_pieces = put_bit(black_pieces, Position::new(row, col).to_index())
                    }
                };
                col += 1;
            }
        }

        let turn: Color;
        if split[1] == fen::B_TURN.to_string() {
            turn = Color::Black;
        } else {
            turn = Color::White;
        }

        // Castling Arguments
        let mut is_white_castle_short = false;
        let mut is_white_castle_long = false;
        let mut is_black_castle_short = false;
        let mut is_black_castle_long = false;
        for ch in split[2].chars() {
            match ch {
                fen::W_CASTLE_SHORT => is_white_castle_short = true,
                fen::W_CASTLE_LONG => is_white_castle_long = true,
                fen::B_CASTLE_SHORT => is_black_castle_short = true,
                fen::B_CASTLE_LONG => is_black_castle_long = true,
                fen::NONE => break,
                _ => break,
            }
        }

        // En Passant Argument
        if split[3] != fen::NONE.to_string() {
            let target_pos = position::algebraic_to_position(&split[3].to_string());
            ep_target = mask(target_pos.to_index());
        }

        let fullmove_clock = split[4].parse::<u16>().unwrap();
        let halfmove_clock = split[5].parse::<u16>().unwrap();

        Board {
            pawns,
            knights,
            bishops,
            rooks,
            queens,
            kings,
            white_pieces,
            black_pieces,
            ep_target,
            fullmove_clock,
            halfmove_clock,
            turn,
            is_white_castle_short,
            is_white_castle_long,
            is_black_castle_short,
            is_black_castle_long,
        }
    }

    ///
    ///
    ///
    pub fn get_color_piece_board(&self, piece: Piece, color: Color) -> u64 {
        match (piece, color) {
            (Piece::Pawn, Color::White) => (self.pawns & self.white_pieces),
            (Piece::Knight, Color::White) => (self.knights & self.white_pieces),
            (Piece::Bishop, Color::White) => (self.bishops & self.white_pieces),
            (Piece::Rook, Color::White) => (self.rooks & self.white_pieces),
            (Piece::Queen, Color::White) => (self.queens & self.white_pieces),
            (Piece::King, Color::White) => (self.kings & self.white_pieces),
            (Piece::Pawn, Color::Black) => (self.pawns & self.black_pieces),
            (Piece::Knight, Color::Black) => (self.knights & self.black_pieces),
            (Piece::Bishop, Color::Black) => (self.bishops & self.black_pieces),
            (Piece::Rook, Color::Black) => (self.rooks & self.black_pieces),
            (Piece::Queen, Color::Black) => (self.queens & self.black_pieces),
            (Piece::King, Color::Black) => (self.kings & self.black_pieces),
        }
    }

    ///
    ///
    ///
    pub fn get_piece_board(&self, piece: Piece) -> u64 {
        match piece {
            Piece::Pawn => self.pawns,
            Piece::Knight => self.knights,
            Piece::Bishop => self.bishops,
            Piece::Rook => self.rooks,
            Piece::Queen => self.queens,
            Piece::King => self.kings,
        }
    }

    ///
    ///
    ///
    pub fn set_color_board(&mut self, color: Color, bitboard: u64) {
        match color {
            Color::White => self.white_pieces = bitboard,
            Color::Black => self.black_pieces = bitboard,
        };
    }

    ///
    ///
    ///
    pub fn set_piece_board(&mut self, piece: Piece, bitboard: u64) {
        match piece {
            Piece::Pawn => self.pawns = bitboard,
            Piece::Knight => self.knights = bitboard,
            Piece::Bishop => self.bishops = bitboard,
            Piece::Rook => self.rooks = bitboard,
            Piece::Queen => self.queens = bitboard,
            Piece::King => self.kings = bitboard,
        };
    }

    ///
    ///
    ///
    pub fn get_pieces(&self, color: Color) -> u64 {
        match color {
            Color::White => self.white_pieces,
            Color::Black => self.black_pieces,
        }
    }

    ///
    ///
    ///
    pub fn get_castle(&self, castle_length: Castle, color: Color) -> bool {
        match (castle_length, color) {
            (Castle::Short, Color::White) => self.is_white_castle_short,
            (Castle::Long, Color::White) => self.is_white_castle_long,
            (Castle::Short, Color::Black) => self.is_black_castle_short,
            (Castle::Long, Color::Black) => self.is_black_castle_long,
        }
    }

    ///
    ///
    ///
    pub fn set_castle(&mut self, castle_length: Castle, color: Color, value: bool) {
        match (castle_length, color) {
            (Castle::Short, Color::White) => self.is_white_castle_short = value,
            (Castle::Long, Color::White) => self.is_white_castle_long = value,
            (Castle::Short, Color::Black) => self.is_black_castle_short = value,
            (Castle::Long, Color::Black) => self.is_black_castle_long = value,
        }
    }

    #[allow(dead_code)]
    ///
    ///
    ///     
    pub fn get_fen(self: &Board) -> String {
        let square_array = self.get_char_representation();
        let mut out = String::new();
        for row in 0..8 {
            let mut count: u8 = 0;
            for col in 0..8 {
                if square_array[row][col] != EMPTY {
                    if count != 0 {
                        out.push((count + b'0') as char);
                    }
                    out.push(square_array[row][col]);
                    count = 0;
                } else {
                    count += 1;
                }
            }
            if count != 0 {
                out.push((count + b'0') as char);
            }
            out.push(fen::ROW_END);
        }
        out.pop(); // remove trailing row separator
        out.push(EMPTY);

        let turn = self.turn;
        if turn == Color::White {
            out.push(fen::W_TURN);
        } else {
            out.push(fen::B_TURN);
        }

        out.push(EMPTY);
        let castle_bools = [
            self.is_white_castle_short,
            self.is_white_castle_long,
            self.is_black_castle_short,
            self.is_black_castle_long,
        ];
        let castle_chars = vec![
            fen::W_CASTLE_SHORT,
            fen::W_CASTLE_LONG,
            fen::B_CASTLE_SHORT,
            fen::B_CASTLE_LONG,
        ];

        let mut can_castle = false;
        for i in 0..4 {
            if castle_bools[i] {
                can_castle = true;
                out.push(castle_chars[i]);
            }
        }
        if !can_castle {
            out.push(fen::NONE);
        }
        out.push(EMPTY);

        let position = self.ep_target;
        if position == 0 {
            out += &fen::NONE.to_string();
        } else {
            out += &position_to_algebraic(&Position::index(position.leading_zeros() as usize));
        }

        out.push(EMPTY);

        let fm: String = self.fullmove_clock.to_string();
        out.push_str(&fm);

        out.push(EMPTY);

        let hm: String = self.halfmove_clock.to_string();
        out.push_str(&hm);
        out
    }

    ///
    ///
    ///
    pub fn get_char_representation(&self) -> Vec<Vec<char>> {
        let mut out = vec![vec![EMPTY; 8]; 8];

        let piece_boards = [
            self.pawns,
            self.knights,
            self.bishops,
            self.rooks,
            self.queens,
            self.kings,
        ];
        let piece_types = [
            Piece::Pawn,
            Piece::Knight,
            Piece::Bishop,
            Piece::Rook,
            Piece::Queen,
            Piece::King,
        ];
        for i in 0..6 {
            let mut bitboard = piece_boards[i];
            let p_type = piece_types[i];

            // WARNING: Little Endian
            let mut leading = bitboard.leading_zeros();
            while leading < SQUARE_COUNT {
                let position = Position::index(leading as usize);

                let is_white = get_bit(bitboard & self.white_pieces, leading as usize) != 0;
                let is_black = get_bit(bitboard & self.black_pieces, leading as usize) != 0;

                if is_white {
                    out[position.get_row() as usize][position.get_col() as usize] =
                        piece_to_fen(p_type, Color::White);
                } else if is_black {
                    out[position.get_row() as usize][position.get_col() as usize] =
                        piece_to_fen(p_type, Color::Black);
                } else {
                    out[position.get_row() as usize][position.get_col() as usize] = '!';
                }
                bitboard = clear_bit(bitboard, leading as usize);
                leading = bitboard.leading_zeros();
            }
        }
        out
    }

    ///
    ///
    ///
    pub fn get_state(&self) -> [[(Option<Piece>, Option<Color>); 8]; 8] {
        let mut out = [[(None, None); 8]; 8];

        let piece_boards = [
            self.pawns,
            self.knights,
            self.bishops,
            self.rooks,
            self.queens,
            self.kings,
        ];
        let piece_types = [
            Piece::Pawn,
            Piece::Knight,
            Piece::Bishop,
            Piece::Rook,
            Piece::Queen,
            Piece::King,
        ];

        for i in 0..6 {
            let mut bitboard = piece_boards[i];
            let p_type = piece_types[i];

            // WARNING: Little Endian
            let mut leading = bitboard.leading_zeros();
            while leading < SQUARE_COUNT {
                let position = Position::index(leading as usize);

                let is_white = get_bit(bitboard & self.white_pieces, leading as usize) != 0;

                if is_white {
                    out[position.get_row() as usize][position.get_col() as usize] =
                        (Some(p_type), Some(Color::White));
                } else {
                    out[position.get_row() as usize][position.get_col() as usize] =
                        (Some(p_type), Some(Color::Black));
                }
                bitboard = clear_bit(bitboard, leading as usize);
                leading = bitboard.leading_zeros();
            }
        }
        out
    }

    /// Print board representation
    /// @param board - the board to print
    pub fn to_string(&self) -> String {
        let mut out = self.get_fen();
        out.push(NEWLINE);
        let square_array = self.get_char_representation();
        out += H_BOUNDARY;
        out.push(NEWLINE);

        for r in 0..8 {
            if r != 0 {
                out += H_BAR;
                out.push(NEWLINE);
            }
            for c in 0..8 {
                out += LEFT;
                out.push(square_array[r][c]);
                out += RIGHT;
            }
            out += END;
            out.push(NEWLINE);
        }

        out += H_BOUNDARY;
        out
    }

    #[allow(dead_code)]
    pub fn dbg(&self) {
        println!("{}", self.to_string());
    }
}
