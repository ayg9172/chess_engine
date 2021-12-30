use super::color::Color;
use super::fen;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub fn fen_to_piece(fen: char) -> (Piece, Color) {
    let color = if fen.is_uppercase() {
        Color::White
    } else {
        Color::Black
    };
    match fen.to_ascii_lowercase() {
        fen::B_PAWN => (Piece::Pawn, color),
        fen::B_KNIGHT => (Piece::Knight, color),
        fen::B_BISHOP => (Piece::Bishop, color),
        fen::B_ROOK => (Piece::Rook, color),
        fen::B_QUEEN => (Piece::Queen, color),
        _ => (Piece::King, color),
    }
}

pub fn piece_to_fen(piece: Piece, color: Color) -> char {
    match (piece, color) {
        (Piece::Pawn, Color::White) => fen::W_PAWN,
        (Piece::Knight, Color::White) => fen::W_KNIGHT,
        (Piece::Bishop, Color::White) => fen::W_BISHOP,
        (Piece::Rook, Color::White) => fen::W_ROOK,
        (Piece::Queen, Color::White) => fen::W_QUEEN,
        (Piece::King, Color::White) => fen::W_KING,
        (Piece::Pawn, Color::Black) => fen::B_PAWN,
        (Piece::Knight, Color::Black) => fen::B_KNIGHT,
        (Piece::Bishop, Color::Black) => fen::B_BISHOP,
        (Piece::Rook, Color::Black) => fen::B_ROOK,
        (Piece::Queen, Color::Black) => fen::B_QUEEN,
        (Piece::King, Color::Black) => fen::B_KING,
    }
}
