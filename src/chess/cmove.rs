use super::piece::{Piece, self};
use super::position::Position;

#[derive(Copy, Clone, PartialEq)]
/// Represents a Chess move
/// Start and End are represented with a Position class
/// piece: the moving piece
/// promotion: optionally the moving piece's new type
pub struct Move {
    pub start: Position,
    pub end: Position,
    pub piece: Piece, // redundant but useful
    pub promotion: Option<Piece>,
}

/// Possible promotion types
const PROMOTION_PIECES: [Piece; 4] = [Piece::Queen, Piece::Knight, Piece::Rook, Piece::Bishop];


impl Move {
    /// Constructor
    pub fn new(start: Position, end: Position, piece: Piece, promotion: Option<Piece>) -> Move {
        Move {
            start,
            end,
            promotion,
            piece,
        }
    }

    /// Return a Vec of 4 moves with given start and end positions
    /// (One for each promotion type)
    pub fn make_promotions(start: Position, end: Position) -> Vec<Move> {
        let mut out = Vec::new();
        for piece in PROMOTION_PIECES {
            out.push(Move::new(start, end, Piece::Pawn, Some(piece)));
        }

        out
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        return format!("{:?}", self.piece)
            + ":"
            + &self.start.to_string()
            + "->"
            + &self.end.to_string();
    }
    #[allow(dead_code)]
    pub fn dbg(&self) {
        println!("{}", self.to_string());
    }
}
