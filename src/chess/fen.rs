/// White Pieces
pub const W_PAWN: char = 'P';
pub const W_KNIGHT: char = 'N';
pub const W_BISHOP: char = 'B';
pub const W_ROOK: char = 'R';
pub const W_QUEEN: char = 'Q';
pub const W_KING: char = 'K';

/// Black Pieces
pub const B_PAWN: char = 'p';
pub const B_KNIGHT: char = 'n';
pub const B_BISHOP: char = 'b';
pub const B_ROOK: char = 'r';
pub const B_QUEEN: char = 'q';
pub const B_KING: char = 'k';

/// Row Terminator
pub const ROW_END: char = '/';

/// Turn Indicator
pub const W_TURN: char = 'w';
pub const B_TURN: char = 'b';

/// Castle Indicators
pub const W_CASTLE_LONG: char = 'Q';
pub const W_CASTLE_SHORT: char = 'K';
pub const B_CASTLE_LONG: char = 'q';
pub const B_CASTLE_SHORT: char = 'k';

/// Indicator for Missing Fields
pub const NONE: char = '-';

/// Fen String for the Starting Board
pub const STARTING_BOARD: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
