use crate::chess::direction::shift;

use super::{
    bitboard_util::{clear_bit, mask},
    board::{Board, Castle, Square},
    cmove::Move,
    color::Color,
    direction::Direction,
    piece::Piece,
};

const KING_ORIGIN: usize = Square::E8 as usize;
const SHORT_KING_LANDING: usize = Square::G8 as usize;
const LONG_KING_LANDING: usize = Square::C8 as usize;

const LONG_ROOK_ORIGIN: usize = Square::A8 as usize;
const SHORT_ROOK_ORIGIN: usize = Square::H8 as usize;
const SHORT_ROOK_LANDING: usize = Square::F8 as usize;
const LONG_ROOK_LANDING: usize = Square::D8 as usize;

const UNIVERSE: u64 = u64::MAX;

pub struct MoveExecutor {
    /// reference to the board
    board: Box<Board>,
    previous_boards: Vec<Board>,

    clear_short: u64,
    clear_long: u64,

    short_castle_result: u64,
    long_castle_result: u64,

    pub debug_capture_count: u64,
}

const BOARD_SIZE: u32 = 8;
const CAPTURE_TYPES: [Piece; 5] = [
    Piece::Pawn,
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
];

impl MoveExecutor {
    pub fn new(board: Board) -> MoveExecutor {
        let masks = MoveExecutor::get_castle_masks();
        MoveExecutor {
            board: Box::new(board),
            previous_boards: Vec::new(),

            clear_short: masks.0,
            clear_long: masks.1,
            short_castle_result: masks.2,
            long_castle_result: masks.3,

            debug_capture_count: 0,
        }
    }

    pub fn get_board_as_mut(&mut self) -> &mut Board {
        self.board.as_mut()
    }

    fn get_castle_masks() -> (u64, u64, u64, u64, u64, u64) {
        let short_rook_result = mask(SHORT_ROOK_LANDING);
        let long_rook_result = mask(LONG_ROOK_LANDING);
        let short_king_result = mask(SHORT_KING_LANDING);
        let long_king_result = mask(LONG_KING_LANDING);

        let mut clear_short = UNIVERSE;
        let mut clear_long = UNIVERSE;

        for i in KING_ORIGIN..SHORT_ROOK_ORIGIN + 1 {
            clear_short = clear_bit(clear_short, i);
        }

        for i in LONG_ROOK_ORIGIN..KING_ORIGIN + 1 {
            clear_long = clear_bit(clear_long, i);
        }

        let short_castle_origin = mask(KING_ORIGIN) | mask(SHORT_ROOK_ORIGIN);
        let long_castle_origin = mask(KING_ORIGIN) | mask(LONG_ROOK_ORIGIN);
        let short_castle_result = short_king_result | short_rook_result;
        let long_castle_result = long_king_result | long_rook_result;

        (
            clear_short,
            clear_long,
            short_castle_origin,
            long_castle_origin,
            short_castle_result,
            long_castle_result,
        )
    }

    fn get_rook_origin(castle: Castle, color: Color) -> usize {
        match (color, castle) {
            (Color::Black, Castle::Short) => SHORT_ROOK_ORIGIN,
            (Color::Black, Castle::Long) => LONG_ROOK_ORIGIN,

            // rotate_right wraps the truncated bits to the beginning of the integer
            // (truncated bits in this case are always 1,
            // and we indeed want to fill with 1 at the beginning of the integer)
            (Color::White, Castle::Short) => SHORT_ROOK_ORIGIN + BOARD_SIZE as usize * 7,
            (Color::White, Castle::Long) => LONG_ROOK_ORIGIN + BOARD_SIZE as usize * 7,
        }
    }

    fn get_king_landing(castle: Castle, color: Color) -> usize {
        match (color, castle) {
            (Color::Black, Castle::Short) => SHORT_KING_LANDING,
            (Color::Black, Castle::Long) => LONG_KING_LANDING,

            // rotate_right wraps the truncated bits to the beginning of the integer
            // (truncated bits in this case are always 1,
            // and we indeed want to fill with 1 at the beginning of the integer)
            (Color::White, Castle::Short) => SHORT_KING_LANDING + BOARD_SIZE as usize * 7,
            (Color::White, Castle::Long) => LONG_KING_LANDING + BOARD_SIZE as usize * 7,
        }
    }

    fn get_clear_castle(&self, castle: Castle, color: Color) -> u64 {
        match (color, castle) {
            (Color::Black, Castle::Short) => self.clear_short,
            (Color::Black, Castle::Long) => self.clear_long,

            // rotate_right wraps the truncated bits to the beginning of the integer
            // (truncated bits in this case are always 1,
            // and we indeed want to fill with 1 at the beginning of the integer)
            (Color::White, Castle::Short) => self.clear_short.rotate_right(BOARD_SIZE * 7),
            (Color::White, Castle::Long) => self.clear_long.rotate_right(BOARD_SIZE * 7),
        }
    }

    fn get_castle_result(&self, castle: Castle, color: Color) -> u64 {
        match (color, castle) {
            (Color::Black, Castle::Short) => self.short_castle_result,
            (Color::Black, Castle::Long) => self.long_castle_result,

            // rotate_right wraps the truncated bits to the beginning of the integer
            // (truncated bits in this case are always 1,
            // and we indeed want to fill with 1 at the beginning of the integer)
            (Color::White, Castle::Short) => self.short_castle_result.rotate_right(BOARD_SIZE * 7),
            (Color::White, Castle::Long) => self.long_castle_result.rotate_right(BOARD_SIZE * 7),
        }
    }

    #[allow(clippy::identity_op)]
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

            let s = MoveExecutor::get_king_landing(Castle::Short, color);
            let l = MoveExecutor::get_king_landing(Castle::Long, color);

            // TODO: halve the code here :)

            if is_legal_short && cmove.end.to_index() == s {
                let clear_mask = self.get_clear_castle(Castle::Short, color);
                let mut rook_board = self.board.get_piece_board(Piece::Rook);

                // clear pieces on castling squares in relevant bitboards
                friendly_pieces &= clear_mask;
                piece_board &= clear_mask;
                rook_board &= clear_mask;

                // put castle result on friendly bitboard
                let castle_mask = self.get_castle_result(Castle::Short, color);

                friendly_pieces |= castle_mask;

                // place rook on new location
                let rook_mask = castle_mask & !mask(cmove.end.to_index());
                rook_board |= rook_mask;

                self.board.set_piece_board(Piece::Rook, rook_board);
            }

            // TODO: halve the code here :)
            if is_legal_long && cmove.end.to_index() == l {
                let clear_mask = self.get_clear_castle(Castle::Long, color);
                let mut rook_board = self.board.get_piece_board(Piece::Rook);

                // clear pieces on castling squares in relevant bitboards
                friendly_pieces &= clear_mask;
                piece_board &= clear_mask;
                rook_board &= clear_mask;

                // put castle result on friendly bitboard
                let castle_mask = self.get_castle_result(Castle::Long, color);
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
            if cmove.start.to_index() == MoveExecutor::get_rook_origin(Castle::Short, color) {
                self.board.set_castle(Castle::Short, color, false);
            } else if cmove.start.to_index() == MoveExecutor::get_rook_origin(Castle::Long, color) {
                self.board.set_castle(Castle::Long, color, false);
            }
        }

        let capture_mask = mask(cmove.end.to_index()) & enemy_pieces;
        if capture_mask != 0 {
            self.debug_capture_count += 1;
            for capture_type in CAPTURE_TYPES {
                let is_capture = capture_mask & self.board.get_color_piece_board(capture_type, e_color) != 0;
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
                    if cmove.end.to_index() == MoveExecutor::get_rook_origin(Castle::Short, e_color)
                    {
                        self.board.set_castle(Castle::Short, e_color, false);
                    } else if cmove.end.to_index()
                        == MoveExecutor::get_rook_origin(Castle::Long, e_color)
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
        self.board.as_ref()
    }
}
