use crate::chess::direction::shift;

use super::{
    bitboard_util::mask,
    board::{Board, Castle},
    castle_utils::CastleUtility,
    cmove::Move,
    color::Color,
    direction::Direction,
    piece::Piece,
};

pub struct MoveExecutor {
    /// reference to the board
    board: Box<Board>,
    previous_boards: Vec<Board>,
    castle_utility: CastleUtility,
}

const CAPTURE_TYPES: [Piece; 5] = [
    Piece::Pawn,
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
];

impl MoveExecutor {
    pub fn new(board: Board) -> MoveExecutor {
        MoveExecutor {
            board: Box::new(board),
            previous_boards: Vec::new(),

            castle_utility: CastleUtility::new(),
        }
    }

    pub fn exec_move(&mut self, mut cmove: Move) {
        self.previous_boards.push(*self.board.clone());

        let color = self.board.turn;
        let e_color = color.get_opposite();
        let mut friendly_pieces = self.board.get_pieces(color);
        let mut enemy_pieces = self.board.get_pieces(e_color);
        let mut piece_board = self.board.get_piece_board(cmove.piece);

        if cmove.piece == Piece::King {
            let is_legal_short = self.board.get_castle(Castle::Short, color);
            let is_legal_long = self.board.get_castle(Castle::Long, color);

            let s = CastleUtility::get_king_landing(Castle::Short, color);
            let l = CastleUtility::get_king_landing(Castle::Long, color);

            // TODO: halve the code here :)

            if is_legal_short && cmove.end.to_index() == s {
                let clear_mask = self.castle_utility.get_clear_castle(Castle::Short, color);
                let mut rook_board = self.board.get_piece_board(Piece::Rook);

                // clear pieces on castling squares in relevant bitboards
                friendly_pieces &= clear_mask;
                piece_board &= clear_mask;
                rook_board &= clear_mask;

                // put castle result on friendly bitboard
                let castle_mask = self.castle_utility.get_castle_result(Castle::Short, color);

                friendly_pieces |= castle_mask;

                // place rook on new location
                let rook_mask = castle_mask & !mask(cmove.end.to_index());
                rook_board |= rook_mask;

                self.board.set_piece_board(Piece::Rook, rook_board);
            }

            // TODO: halve the code here :)
            if is_legal_long && cmove.end.to_index() == l {
                let clear_mask = self.castle_utility.get_clear_castle(Castle::Long, color);
                let mut rook_board = self.board.get_piece_board(Piece::Rook);

                // clear pieces on castling squares in relevant bitboards
                friendly_pieces &= clear_mask;
                piece_board &= clear_mask;
                rook_board &= clear_mask;

                // put castle result on friendly bitboard
                let castle_mask = self.castle_utility.get_castle_result(Castle::Long, color);
                friendly_pieces |= castle_mask;

                // place rook on new location
                let rook_mask = castle_mask & !mask(cmove.end.to_index());
                rook_board |= rook_mask;

                self.board.set_piece_board(Piece::Rook, rook_board);
            }
            self.board.set_castle(Castle::Short, color, false);
            self.board.set_castle(Castle::Long, color, false);
        }

        if cmove.piece == Piece::Rook {
            // Check for rook loss of castle privileges
            if cmove.start.to_index() == CastleUtility::get_rook_origin(Castle::Short, color) {
                self.board.set_castle(Castle::Short, color, false);
            } else if cmove.start.to_index() == CastleUtility::get_rook_origin(Castle::Long, color)
            {
                self.board.set_castle(Castle::Long, color, false);
            }
        }

        let capture_mask = mask(cmove.end.to_index()) & enemy_pieces;
        if capture_mask != 0 {
            for capture_type in CAPTURE_TYPES {
                let is_capture =
                    capture_mask & self.board.get_color_piece_board(capture_type, e_color) != 0;
                if is_capture {
                    // remove captured piece from piece type bitboard
                    let mut captured_piece_board = self.board.get_piece_board(capture_type);
                    captured_piece_board &= !mask(cmove.end.to_index());

                    if capture_type == cmove.piece {
                        // TODO: clean this up
                        // this is a hacky way of putting everything together
                        piece_board &= !mask(cmove.end.to_index());
                    }

                    // if rook is captured, other side obviously cant castle with it :)
                    if cmove.end.to_index()
                        == CastleUtility::get_rook_origin(Castle::Short, e_color)
                    {
                        self.board.set_castle(Castle::Short, e_color, false);
                    } else if cmove.end.to_index()
                        == CastleUtility::get_rook_origin(Castle::Long, e_color)
                    {
                        self.board.set_castle(Castle::Long, e_color, false);
                    }

                    self.board
                        .set_piece_board(capture_type, captured_piece_board);
                    // remove captured piece from enemy bitboard
                    enemy_pieces &= !mask(cmove.end.to_index());
                }
            }
        };

        let mut pep = 0; // TODO: clarify variable name
        if cmove.piece == Piece::Pawn {
            // Adjust destination square if pawn captures by EP
            let mut is_ep = self.board.ep_target & mask(cmove.end.to_index()) != 0;

            // ensure pawns are on same row
            is_ep = is_ep
                && (self.board.ep_target.leading_zeros() as i32 - cmove.start.to_index() as i32)
                    .abs()
                    == 1;
            if is_ep {
                let vector = if color == Color::White {
                    shift(Direction::North)
                } else {
                    shift(Direction::South)
                };
                // adjust landing square, this step must be done after capture check
                cmove.end += vector;
            }

            // Check for promotion
            if cmove.promotion != None {
                let promotion_type = cmove.promotion.unwrap();
                //piece_board &= !mask(cmove.end.to_index());?????
                piece_board &= !mask(cmove.start.to_index());

                let mut new_type_board = self.board.get_piece_board(promotion_type);
                new_type_board |= mask(cmove.end.to_index());
                self.board.set_piece_board(promotion_type, new_type_board);
            }

            // add self as ep target if diff of rows is 2
            if (cmove.start.get_row() - cmove.end.get_row()).abs() == 2 {
                pep = mask(cmove.end.to_index());
            }
        }
        self.board.ep_target = 0 | pep; // moving resets ep target

        // move piece
        piece_board &= !mask(cmove.start.to_index());

        // only add piece at target if no promotion
        if cmove.promotion == None || cmove.piece != Piece::Pawn {
            piece_board |= mask(cmove.end.to_index());
        }

        friendly_pieces &= !mask(cmove.start.to_index());
        friendly_pieces |= mask(cmove.end.to_index());

        self.board.set_color_board(color, friendly_pieces);
        self.board.set_color_board(e_color, enemy_pieces);
        self.board.set_piece_board(cmove.piece, piece_board);

        self.board.turn = self.board.turn.get_opposite();
    }

    pub fn undo_move(&mut self) {
        self.board = Box::new(self.previous_boards.pop().unwrap());
    }

    pub fn get_board_ref(&self) -> &Board {
        return self.board.as_ref();
    }
}
