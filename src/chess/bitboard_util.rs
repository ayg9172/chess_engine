#[rustfmt::skip]
/// Obtained using _get_mask_array
/// These are powers of two so the binary string only contains 
/// a single one. This is useful to mask out individual bits
const MASKS : [u64; 64] = [9223372036854775808, 4611686018427387904, 
2305843009213693952, 1152921504606846976, 576460752303423488, 
288230376151711744, 144115188075855872, 72057594037927936, 36028797018963968,
18014398509481984, 9007199254740992, 4503599627370496, 2251799813685248, 
1125899906842624, 562949953421312, 281474976710656, 140737488355328, 
70368744177664, 35184372088832, 17592186044416, 8796093022208, 4398046511104,
2199023255552, 1099511627776, 549755813888, 274877906944, 137438953472, 
68719476736, 34359738368, 17179869184, 8589934592, 4294967296, 2147483648, 
1073741824, 536870912, 268435456, 134217728, 67108864, 33554432, 16777216, 
8388608, 4194304, 2097152, 1048576, 524288, 262144, 131072, 65536, 32768,
16384, 8192, 4096, 2048, 1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1];

/// Generate individual bit masks
/// Run it to obtain the MASKS constant
fn _get_mask_array() -> [u64; 64] {
    let mut masks: [u64; 64] = [0; 64];
    let two: u64 = 2;
    for i in 0..64 {
        masks[63 - i] = two.pow(i as u32);
    }
    masks
}

/// Get individual bit mask with a 1 at the given index
pub fn mask(index: usize) -> u64 {
    MASKS[index]
}

/// Get the individual bit at the given index in a bitboard
pub fn get_bit(bitboard: u64, index: usize) -> u64 {
    mask(index) & bitboard
}

/// Put the individual bit at the given index in a bitboard
pub fn put_bit(bitboard: u64, index: usize) -> u64 {
    mask(index) | bitboard
}

/// Clear the individual bit at the given index in a bitboard
pub fn clear_bit(bitboard: u64, index: usize) -> u64 {
    (!mask(index)) & bitboard
}

/// Nice string representation of the bitboard for debugging
#[allow(dead_code)]
pub fn bitboard_to_string(board: u64) -> String {
    let mut out = String::new();
    let st: String = format!("{:b}", board);
    let length: usize = st.len();
    let pad_amount = 64 - length;
    for i in 0..(pad_amount) {
        // pad with zeroes
        out.push('0');
        if (i + 1) % 8 == 0 {
            out.push('\n');
        }
    }
    for (i, bit) in st.chars().enumerate() {
        out.push(bit);
        if (pad_amount + i + 1) % 8 == 0 {
            out.push('\n');
        }
    }
    out.pop(); // remove trailing newline
    out
}
