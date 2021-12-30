use super::{
    bitboard_util::{clear_bit, mask},
    board::Square,
};

#[allow(dead_code)]
pub const KING_ORIGIN: usize = Square::E8 as usize;

#[allow(dead_code)]
pub const SHORT_KING_LANDING: usize = Square::G8 as usize;

#[allow(dead_code)]
pub const LONG_KING_LANDING: usize = Square::C8 as usize;

#[allow(dead_code)]
pub const LONG_ROOK_ORIGIN: usize = Square::A8 as usize;

#[allow(dead_code)]
pub const LONG_ROOK_LANDING: usize = Square::D8 as usize;

pub const SHORT_ROOK_ORIGIN: usize = Square::H8 as usize;

#[allow(dead_code)]
pub const SHORT_ROOK_LANDING: usize = Square::F8 as usize;

#[allow(dead_code)]
const UNIVERSE: u64 = u64::MAX;

#[allow(dead_code)]
pub fn get_castle_masks() -> (u64, u64, u64, u64, u64, u64) {
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
