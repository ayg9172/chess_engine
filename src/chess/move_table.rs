use super::bitboard_util::{mask, put_bit};
use super::board::{Castle, Square};
use super::color::Color;
use super::direction::{get_direction, shift, Direction};
use super::direction::{
    Direction::East, Direction::North, Direction::NorthEast, Direction::NorthWest,
    Direction::South, Direction::SouthEast, Direction::SouthWest, Direction::West,
};
use super::piece::Piece;
use super::position::Position;
use std::ops::Index;

const W_CASTLE_SHORT: usize = Square::G1 as usize;
const W_CASTLE_LONG: usize = Square::C1 as usize;
const B_CASTLE_SHORT: usize = Square::G8 as usize;
const B_CASTLE_LONG: usize = Square::C8 as usize;

fn sliding_helper(index: usize, directions: &Vec<Direction>, max_magnitude: i8) -> u64 {
    let mut bitboard: u64 = 0;
    let origin = Position::index(index);
    for direction in directions {
        let mut current = origin + shift(*direction);
        let mut i = 0;
        while current.is_in_range() && i < max_magnitude {
            bitboard = put_bit(bitboard, current.to_index());
            current += shift(*direction);
            i += 1;
        }
    }
    bitboard
}

fn jumping_helper(index: usize, jumps: &Vec<Position>) -> u64 {
    let mut bitboard: u64 = 0;
    let origin = Position::index(index);
    for jump in jumps {
        let current = origin + (*jump);
        if current.is_in_range() {
            bitboard = put_bit(bitboard, current.to_index());
        }
    }
    bitboard
}

fn get_sliding_all(directions: &Vec<Direction>, max_magnitude: i8) -> [u64; 64] {
    let mut masks: [u64; 64] = [0; 64];
    for i in 0..64 {
        masks[i] = sliding_helper(i, directions, max_magnitude);
    }
    masks
}

fn get_pawn_masks() -> [u64; 64] {
    let directions = vec![North];
    let mut masks = get_sliding_all(&directions, 1);

    let start = Square::A2 as usize;
    let end = Square::H2 as usize + 1;

    for i in start..end {
        masks[i] = sliding_helper(i, &directions, 2);
    }
    masks
}

fn get_pawn_attack_masks() -> [u64; 64] {
    let directions = vec![NorthWest, NorthEast];
    get_sliding_all(&directions, 1)
}

fn get_knight_masks() -> [u64; 64] {
    let jumps: Vec<Position> = vec![
        Position::new(1, 2),
        Position::new(1, -2),
        Position::new(2, 1),
        Position::new(-2, 1),
        Position::new(-1, 2),
        Position::new(-1, -2),
        Position::new(2, -1),
        Position::new(-2, -1),
    ];

    let mut masks: [u64; 64] = [0; 64];
    for i in 0..64 {
        masks[i] = jumping_helper(i, &jumps)
    }
    masks
}

fn get_bishop_masks() -> [u64; 64] {
    let directions = vec![NorthEast, SouthEast, SouthWest, NorthWest];
    get_sliding_all(&directions, 8)
}

fn get_rook_masks() -> [u64; 64] {
    let directions = vec![West, North, East, South];
    get_sliding_all(&directions, 8)
}

fn get_queen_masks() -> [u64; 64] {
    let directions = vec![
        East, NorthEast, North, NorthWest, West, SouthWest, South, SouthEast,
    ];

    get_sliding_all(&directions, 8)
}

fn get_king_masks() -> [u64; 64] {
    let directions = vec![
        East, NorthEast, North, NorthWest, West, SouthWest, South, SouthEast,
    ];
    get_sliding_all(&directions, 1)
}

fn get_ray_masks() -> [[u64; 64]; 64] {
    let mut rays = [[0; 64]; 64];

    for origin in 0..64 {
        for destination in 0..64 {
            let u: Position = Position::index(origin);
            let mut v: Position = Position::index(destination);
            let vector = v - u;
            let direction = get_direction(&vector);
            if direction == None {
                continue;
            }

            v += shift(direction.unwrap());
            while v.is_in_range() {
                rays[origin][destination] = put_bit(rays[origin][destination], v.to_index());
                v += shift(direction.unwrap());
            }
        }
    }
    rays
}

fn manual_mask(squares: Vec<usize>) -> u64 {
    let mut out: u64 = 0;
    for sq in squares {
        out = put_bit(out, sq);
    }
    out
}

pub struct MoveTable {
    pawn: [u64; 64],
    pawn_attack: [u64; 64],

    reverse_pawn: [u64; 64],
    reverse_pawn_attack: [u64; 64],

    knight: [u64; 64],
    rook: [u64; 64],
    bishop: [u64; 64],
    queen: [u64; 64],
    king: [u64; 64],

    fourth_ranks: u64,
    rays: [[u64; 64]; 64],

    w_castle_short: u64,
    w_castle_long: u64,
    b_castle_short: u64,
    b_castle_long: u64,

    w_castle_short_safety: u64,
    w_castle_long_safety: u64,
    b_castle_short_safety: u64,
    b_castle_long_safety: u64,
}

impl Index<Piece> for MoveTable {
    type Output = [u64; 64];
    fn index(&self, piece: Piece) -> &Self::Output {
        match piece {
            Piece::Pawn => &self.pawn,
            Piece::Knight => &self.knight,
            Piece::Bishop => &self.bishop,
            Piece::Rook => &self.rook,
            Piece::Queen => &self.queen,
            Piece::King => &self.king,
        }
    }
}

impl MoveTable {
    pub fn new() -> MoveTable {
        let pawn = get_pawn_masks();
        let pawn_attacks = get_pawn_attack_masks();
        let mut reverse_pawn = [0; 64];
        let mut reverse_pawn_attacks = [0; 64];

        for i in 0..64 {
            // add to list in reverse order
            reverse_pawn[63 - i] = pawn[i].reverse_bits();
            reverse_pawn_attacks[63 - i] = pawn_attacks[i].reverse_bits();
        }

        let mut fourth_ranks = 0;
        for i in Square::A4 as usize..Square::H4 as usize + 1 {
            fourth_ranks |= put_bit(fourth_ranks, i);
        }
        for i in Square::A5 as usize..Square::H5 as usize + 1 {
            fourth_ranks |= put_bit(fourth_ranks, i);
        }

        let ws = manual_mask(vec![W_CASTLE_SHORT, W_CASTLE_SHORT - 1, W_CASTLE_SHORT - 2]);
        let wl = manual_mask(vec![W_CASTLE_LONG, W_CASTLE_LONG + 1, W_CASTLE_LONG + 2]);
        let bs = manual_mask(vec![B_CASTLE_SHORT, B_CASTLE_SHORT - 1, B_CASTLE_SHORT - 2]);
        let bl = manual_mask(vec![B_CASTLE_LONG, B_CASTLE_LONG + 1, B_CASTLE_LONG + 2]);

        MoveTable {
            pawn,
            pawn_attack: pawn_attacks,

            reverse_pawn,
            reverse_pawn_attack: reverse_pawn_attacks,

            knight: get_knight_masks(),
            rook: get_rook_masks(),
            bishop: get_bishop_masks(),
            queen: get_queen_masks(),
            king: get_king_masks(),

            fourth_ranks,
            rays: get_ray_masks(),

            w_castle_short: mask(W_CASTLE_SHORT),
            w_castle_long: mask(W_CASTLE_LONG),
            b_castle_short: mask(B_CASTLE_SHORT),
            b_castle_long: mask(B_CASTLE_LONG),

            w_castle_short_safety: ws,
            w_castle_long_safety: wl,
            b_castle_short_safety: bs,
            b_castle_long_safety: bl,
        }
    }

    pub fn get_pawn(&self, color: Color) -> [u64; 64] {
        match color {
            Color::White => self.pawn,
            Color::Black => self.reverse_pawn,
        }
    }

    pub fn get_pawn_attack(&self, color: Color) -> [u64; 64] {
        match color {
            Color::White => self.pawn_attack,
            Color::Black => self.reverse_pawn_attack,
        }
    }

    pub fn get_fourth_rank(&self) -> u64 {
        self.fourth_ranks
    }

    pub fn get_rays(&self) -> [[u64; 64]; 64] {
        self.rays
    }

    pub fn get_castle(&self, castle_length: Castle, color: Color) -> u64 {
        match (castle_length, color) {
            (Castle::Short, Color::White) => self.w_castle_short,
            (Castle::Long, Color::White) => self.w_castle_long,
            (Castle::Short, Color::Black) => self.b_castle_short,
            (Castle::Long, Color::Black) => self.b_castle_long,
        }
    }

    pub fn get_castle_safe_squares(&self, castle_length: Castle, color: Color) -> u64 {
        match (castle_length, color) {
            (Castle::Short, Color::White) => self.w_castle_short_safety,
            (Castle::Long, Color::White) => self.w_castle_long_safety,
            (Castle::Short, Color::Black) => self.b_castle_short_safety,
            (Castle::Long, Color::Black) => self.b_castle_long_safety,
        }
    }
}
