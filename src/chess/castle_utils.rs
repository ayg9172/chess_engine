use super::{
    bitboard_util::{clear_bit, mask},
    board::{Castle, Square},
    color::Color,
};

#[allow(dead_code)]
const BOARD_SIZE: u32 = 8;
pub struct CastleUtility {
    clear_short: u64,
    clear_long: u64,

    short_castle_result: u64,
    long_castle_result: u64,
}

impl CastleUtility {
    const KING_ORIGIN: usize = Square::E8 as usize;
    const SHORT_KING_LANDING: usize = Square::G8 as usize;
    const LONG_KING_LANDING: usize = Square::C8 as usize;
    const LONG_ROOK_ORIGIN: usize = Square::A8 as usize;
    const LONG_ROOK_LANDING: usize = Square::D8 as usize;
    const SHORT_ROOK_ORIGIN: usize = Square::H8 as usize;
    const SHORT_ROOK_LANDING: usize = Square::F8 as usize;

    const UNIVERSE: u64 = u64::MAX;

    pub fn new() -> Self {
        let masks = CastleUtility::get_castle_masks();
        CastleUtility {
            clear_short: masks.0,
            clear_long: masks.1,

            short_castle_result: masks.2,
            long_castle_result: masks.3,
        }
    }

    fn get_castle_masks() -> (u64, u64, u64, u64) {
        let short_rook_result = mask(CastleUtility::SHORT_ROOK_LANDING);
        let long_rook_result = mask(CastleUtility::LONG_ROOK_LANDING);
        let short_king_result = mask(CastleUtility::SHORT_KING_LANDING);
        let long_king_result = mask(CastleUtility::LONG_KING_LANDING);

        let mut clear_short = CastleUtility::UNIVERSE;
        let mut clear_long = CastleUtility::UNIVERSE;

        for i in CastleUtility::KING_ORIGIN..CastleUtility::SHORT_ROOK_ORIGIN + 1 {
            clear_short = clear_bit(clear_short, i);
        }

        for i in CastleUtility::LONG_ROOK_ORIGIN..CastleUtility::KING_ORIGIN + 1 {
            clear_long = clear_bit(clear_long, i);
        }

        let short_castle_result = short_king_result | short_rook_result;
        let long_castle_result = long_king_result | long_rook_result;

        (
            clear_short,
            clear_long,
            short_castle_result,
            long_castle_result,
        )
    }

    pub fn get_rook_origin(castle: Castle, color: Color) -> usize {
        match (color, castle) {
            (Color::Black, Castle::Short) => CastleUtility::SHORT_ROOK_ORIGIN,
            (Color::Black, Castle::Long) => CastleUtility::LONG_ROOK_ORIGIN,

            // rotate_right wraps the truncated bits to the beginning of the integer
            // (truncated bits in this case are always 1,
            // and we indeed want to fill with 1 at the beginning of the integer)
            (Color::White, Castle::Short) => {
                CastleUtility::SHORT_ROOK_ORIGIN + BOARD_SIZE as usize * 7
            }
            (Color::White, Castle::Long) => {
                CastleUtility::LONG_ROOK_ORIGIN + BOARD_SIZE as usize * 7
            }
        }
    }

    pub fn get_king_landing(castle: Castle, color: Color) -> usize {
        match (color, castle) {
            (Color::Black, Castle::Short) => CastleUtility::SHORT_KING_LANDING,
            (Color::Black, Castle::Long) => CastleUtility::LONG_KING_LANDING,

            // rotate_right wraps the truncated bits to the beginning of the integer
            // (truncated bits in this case are always 1,
            // and we indeed want to fill with 1 at the beginning of the integer)
            (Color::White, Castle::Short) => {
                CastleUtility::SHORT_KING_LANDING + BOARD_SIZE as usize * 7
            }
            (Color::White, Castle::Long) => {
                CastleUtility::LONG_KING_LANDING + BOARD_SIZE as usize * 7
            }
        }
    }

    pub fn get_clear_castle(&self, castle: Castle, color: Color) -> u64 {
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

    pub fn get_castle_result(&self, castle: Castle, color: Color) -> u64 {
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
}
