use super::bitboard_util::{clear_bit, mask};
use super::move_table::MoveTable;

use super::board::{Board, Castle, Square};
use super::cmove::Move;
use super::color::Color;
use super::piece::Piece;
use super::position::{Position};
use std::num::Wrapping;

/// Legal Move Gen Idea
/// Prep: Check how many pieces check the King
///       Get check Attack lines
///       
///
/// 1. If more than 1 piece checks the King, generate King moves only [x]
/// 2. Ensure check legality of post-King moves [x]
/// 3. If King is checked, only legal moves are on the Attack Line, capturing checking piece, or by the King piece itself [x]
/// 4. Get pinned pieces, mask away moves if they are made by a pinned piece
///    in the direction of a not pinned move
/// 5. Check for checks discovered by EP

pub struct MoveGenerator {
    move_table: MoveTable,
}

const PIECE_TYPES: [Piece; 6] = [
    Piece::Pawn,
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
    Piece::King,
];

const UNIVERSE: u64 = u64::MAX;
const BOARD_SIZE: u32 = 8;
const W_PAWN_LAST_ROW: usize = 0;
const B_PAWN_LAST_ROW: usize = 7;
impl MoveGenerator {
    pub fn new() -> Self {
        MoveGenerator {
            move_table: MoveTable::new(),
        }
    }

    fn get_pawn_move_bitboard(&self, board: &Board, index: usize, color: Color) -> u64 {
        let mut move_board = self.move_table.get_pawn(color)[index];
        let attack_board = self.move_table.get_pawn_attack(color)[index];

        let friendly_pieces = board.get_pieces(color);
        let enemy_pieces = board.get_pieces(color.get_opposite());
        let all_pieces = friendly_pieces | enemy_pieces;

        let is_first_move = move_board.count_ones() == 2;

        // Remove move targets that have a piece on them
        move_board &= !all_pieces;

        // Clear potential double moves if there is only one move available
        // (since that implies a piece could be in the way of a single move)
        let is_blocked_double: u64 = (move_board.count_ones() != 2 && is_first_move) as u64;

        // remove_mask is the empty set if we do remove, and the universe if we do not
        let remove_mask = (Wrapping(u64::MAX) + Wrapping(is_blocked_double)).0;

        // if we do not remove, remove_mask is the universe
        // and fills in the 0s in fourth_rank_remove
        let fourth_rank_remove = (!self.move_table.get_fourth_rank()) | remove_mask;
        move_board &= fourth_rank_remove;

        // return value initialized here
        let mut out = move_board;

        // Add pawn captures
        out |= enemy_pieces & attack_board;
        let mut ep_target = board.ep_target;
        let ep_index = ep_target.leading_zeros();

        // Idea: En Passant is only possible when enemy pawn is adjacent
        //       if the enemy pawn is in the same row, and index diff is 1)
        let is_same_row = index as u32 / BOARD_SIZE == ep_index / BOARD_SIZE;
        let is_one_off = (index as i32 - ep_index as i32).abs() == 1;
        let is_adjacent = (is_same_row && is_one_off) as bool;

        // if is_adjacent = 0, ep_target is cleared by ANDing it with 0
        // if is_adjacent = 1, ep_target is ANDed with u64 max, (via overflow)
        // which keeps the ep target square
        ep_target &= (Wrapping(0) - Wrapping(is_adjacent as u64)).0;

        // add ep square as one of the moves if any
        out |= ep_target;

        out
    }

    fn get_knight_move_bitboard(&self, board: &Board, index: usize, color: Color) -> u64 {
        let friendly_pieces = board.get_pieces(color);
        let attack_board = self.move_table[Piece::Knight][index];
        (!friendly_pieces) & attack_board
    }

    fn get_king_move_bitboard(&self, board: &Board, index: usize, color: Color) -> u64 {
        let friendly_pieces = board.get_pieces(color);
        let enemy_pieces = board.get_pieces(color.get_opposite());

        // all pieces but the king TODO: better name
        let all_pieces = (friendly_pieces | enemy_pieces) & !board.get_color_piece_board(Piece::King, color);
        let mut attack_board = self.move_table[Piece::King][index];

        let mut castle_short = self.move_table.get_castle(Castle::Short, color);
        let mut castle_long = self.move_table.get_castle(Castle::Long, color);

        // zeroed out if castling is impossible
        castle_short &= (Wrapping(0) - Wrapping(board.get_castle(Castle::Short, color) as u64)).0;
        castle_long &= (Wrapping(0) - Wrapping(board.get_castle(Castle::Long, color) as u64)).0;

        // get intermediate castling squares
        // they must not be under check
        // after extensive debugging, we find these square must also have no pieces
        let mut short_safety = self
            .move_table
            .get_castle_safe_squares(Castle::Short, color);
        let mut long_safety = self.move_table.get_castle_safe_squares(Castle::Long, color);

        // If ANY of the intermediate squares are under attack, castle move mask is zeroed out
        let mut i = short_safety.leading_zeros();
        while i < 64 {
            // get move bitboard and map the destinations to Positions
            let x = self.is_attacked(board, color.get_opposite(), i as usize);
            let safety_mask = (Wrapping(UNIVERSE) + Wrapping(x as u64)).0;

            castle_short &= safety_mask;

            let piece_in_way = all_pieces & mask(i as usize) != 0;
            let p_mask = (Wrapping(UNIVERSE) + Wrapping(piece_in_way as u64)).0;
            // remove castle if there's a piece in the way
            castle_short &= p_mask;

            // Remove bit, so we can get the next one
            short_safety = clear_bit(short_safety, i as usize);
            i = short_safety.leading_zeros();
        }

        // TODO: clean this up, small edge case for long castle

        let in_way = if color == Color::White {
            Square::B1
        } else {
            Square::B8
        };

        let piece_in_way = all_pieces & mask(in_way as usize) != 0;
        let p_mask = (Wrapping(UNIVERSE) + Wrapping(piece_in_way as u64)).0;
        // remove castle if there's a piece in the way
        castle_long &= p_mask;

        let mut i = long_safety.leading_zeros();
        while i < 64 {
            // get move bitboard and map the destinations to Positions
            let x = self.is_attacked(board, color.get_opposite(), i as usize);
            let safety_mask = (Wrapping(UNIVERSE) + Wrapping(x as u64)).0;
            castle_long &= safety_mask;

            let piece_in_way = all_pieces & mask(i as usize) != 0;
            let p_mask = (Wrapping(UNIVERSE) + Wrapping(piece_in_way as u64)).0;
            // remove castle if there's a piece in the way
            castle_long &= p_mask;

            // Remove bit, so we can get the next one
            long_safety = clear_bit(long_safety, i as usize);
            i = long_safety.leading_zeros();
        }

        // ensure King is not attacked
        let mut adjacent = attack_board;
        while adjacent.count_ones() != 0 {
            let index = adjacent.leading_zeros();
            let x = self.is_attacked(board, color.get_opposite(), index as usize) as u64;
            let safety_mask = (Wrapping(0) - Wrapping(x as u64)).0;
            attack_board &= !(safety_mask & mask(index as usize));
            adjacent = clear_bit(adjacent, index as usize);
        }

        attack_board |= (castle_short) | (castle_long);

        (!friendly_pieces) & attack_board
    }

    /// Get moves for sliding pieces
    fn get_sliding_piece(&self, board: &Board, index: usize, piece: Piece, color: Color) -> u64 {
        let friendly_pieces = board.get_pieces(color);
        let enemy_pieces = board.get_pieces(color.get_opposite());

        // can improve efficiency by excluding the outer squares TODO
        let mut attack_board = self.move_table[piece][index];
        let mut blockers = (friendly_pieces | enemy_pieces) & attack_board;

        let mut i = blockers.leading_zeros();
        let rays = self.move_table.get_rays();
        while i < 64 {
            attack_board &= !rays[index as usize][i as usize];
            blockers = clear_bit(blockers, i as usize);
            i = blockers.leading_zeros();
        }
        (!friendly_pieces) & attack_board
    }

    /// Convert each 1 in bitboard to a corresponding position
    fn bitboard_to_positions(bitboard: u64) -> Vec<Position> {
        let mut out = Vec::new();
        let mut current_board = bitboard;
        let mut i = current_board.leading_zeros();
        while i < 64 {
            out.push(Position::index(i as usize));
            current_board = clear_bit(current_board, i as usize);
            i = current_board.leading_zeros();
        }
        out
    }

    fn get_piece_move_bitboard(
        &self,
        board: &Board,
        piece: Piece,
        index: usize,
        color: Color,
    ) -> u64 {
        match piece {
            Piece::Pawn => self.get_pawn_move_bitboard(board, index, color),
            Piece::Knight => self.get_knight_move_bitboard(board, index, color),
            Piece::Bishop => self.get_sliding_piece(board, index, Piece::Bishop, color),
            Piece::Rook => self.get_sliding_piece(board, index, Piece::Rook, color),
            Piece::Queen => self.get_sliding_piece(board, index, Piece::Queen, color),
            Piece::King => self.get_king_move_bitboard(board, index, color),
        } //// TODO:;
    }

    fn get_piece_moves(&self, board: &Board, piece: Piece, color: Color) -> Vec<Move> {
        let mut out = Vec::new();

        // todo: remove redundancy
        let last_row = match color {
            Color::White => W_PAWN_LAST_ROW,
            Color::Black => B_PAWN_LAST_ROW,
        } as i8;

        let mut pieces = board.get_color_piece_board(piece, color);
        let mut i = pieces.leading_zeros();
        while i < 64 {
            // get move bitboard and map the destinations to Positions
            let bitboard = self.get_piece_move_bitboard(board, piece, i as usize, color);
            let destinations = MoveGenerator::bitboard_to_positions(bitboard);
            for d in destinations {
                // todo: clean this up
                if piece == Piece::Pawn {
                    if d.get_row() == last_row {
                        out.extend(Move::make_promotions(Position::index(i as usize), d));
                    } else {
                        out.push(Move::new(
                            Position::index(i as usize),
                            d,
                            Piece::Pawn,
                            Option::None,
                        ));
                    }
                } else {
                    out.push(Move::new(Position::index(i as usize), d, piece, Option::None));
                }
            }
            // Remove pawn, so we can get the next one
            pieces = clear_bit(pieces, i as usize);
            i = pieces.leading_zeros();
        }
        out
    }

    fn is_attacked(&self, board: &Board, attacking_color: Color, index: usize) -> bool {
        let friendly: Color = attacking_color.get_opposite();

        let rook_mask = self.get_piece_move_bitboard(board, Piece::Rook, index, friendly);
        let bishop_mask = self.get_piece_move_bitboard(board, Piece::Bishop, index, friendly);
        let knight_mask = self.get_piece_move_bitboard(board, Piece::Knight, index, friendly);
        let king_mask = self.move_table[Piece::King][index];

        let pawn_mask = self.move_table.get_pawn_attack(friendly)[index];

        // e stands for enemy
        let e_queens =
            board.get_color_piece_board(Piece::Queen, attacking_color) & (rook_mask | bishop_mask);
        let e_rooks = board.get_color_piece_board(Piece::Rook, attacking_color) & (rook_mask);
        let e_bishops = board.get_color_piece_board(Piece::Bishop, attacking_color) & (bishop_mask);
        let e_knights = board.get_color_piece_board(Piece::Knight, attacking_color) & (knight_mask);
        let e_pawns = board.get_color_piece_board(Piece::Pawn, attacking_color) & (pawn_mask);
        let e_kings = board.get_color_piece_board(Piece::King, attacking_color) & (king_mask);

        e_kings != 0
            || e_queens != 0
            || e_rooks != 0
            || e_bishops != 0
            || e_knights != 0
            || e_pawns != 0
    }

    pub fn is_attacked_king(&self, board: &Board, attacking_color: Color) -> bool {
        let king_color = attacking_color.get_opposite();
        let king_board = board.get_color_piece_board(Piece::King, king_color);

        // todo add error check
        let king_position = MoveGenerator::bitboard_to_positions(king_board)[0];
        self.is_attacked(board, attacking_color, king_position.to_index())
    }

    // TODO fix board borrow
    pub fn get_moves(&self, board: &Board) -> Vec<Move> {
        let mut out = Vec::new();
        let pieces = PIECE_TYPES;
        for piece in pieces {
            // TODO: add Color to get_moves()
            out.extend(self.get_piece_moves(board, piece, board.turn));
        }

        out
    }
}
