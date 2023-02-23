

use crate::chess::cmove::Move;

use crate::chess::{piece, board};
use crate::chess::position::Position;
use crate::chess::{color::Color, move_api::MoveAPI, piece::Piece, fen};

#[allow(dead_code)]
pub const WHITE: &str = "White";

#[allow(dead_code)]
pub const BLACK: &str = "Black";

#[allow(dead_code)]
pub const DRAW: &str = "Stalemate";

#[allow(dead_code)]
pub const NOT_OVER: &str = "IsOngoing";

///
///
///
pub enum Message {
    Info(String),
    Error(String),
}

pub trait GameAPI {
    ///
    ///
    ///
    fn try_move(&mut self, m: Move) -> Message;

    ///
    ///
    ///
    fn try_undo(&mut self) -> Message;

    ///
    ///
    ///
    fn get_state(&self) -> [[(Option<Piece>, Option<Color>); 8]; 8];

    ///
    ///
    ///
    fn get_outcome(&mut self) -> Message;

    ///
    ///
    ///
    fn get_legal_moves(&mut self) -> Vec<Move>;
}

///
///
///
pub struct ChessGame {
    move_api: MoveAPI,
}

impl ChessGame {
    #[allow(dead_code)]
    pub fn new(move_api: MoveAPI) -> Self {
        ChessGame {
            move_api,
        }
    }

    pub fn dbg(&self) {
        println!("{}", self.move_api.get_board_ref().to_string())
    }

    pub fn get_fen(&self) -> String {
        // todo put functionality in move_api
        return self.move_api.get_board_ref().get_fen();
    }

    pub fn get_turn_color(&self) -> Color {
        self.move_api.get_turn_color()
    }

    pub fn get_piece(&self, position: Position) -> Option<Piece> {
        // todo move this logic to board
        let board = self.move_api
            .get_board_ref()
            .get_char_representation();


        

        let piece_fen = board[position.get_row() as usize][position.get_col() as usize];

        if piece_fen == board::EMPTY {
            return None;
        }

        return Some(piece::fen_to_piece(piece_fen).0);
    }
}

impl GameAPI for ChessGame {



    fn try_move(&mut self, m: Move) -> Message {
        let legal_moves = self.move_api.get_legal_moves();
        if legal_moves.contains(&m) {
            self.move_api.exec_move(m);
            Message::Info("Move Sucess".to_string())
        } else {
            Message::Error("Not a legal move".to_string())
        }
    }

    fn try_undo(&mut self) -> Message {
        self.move_api.undo_move();
        Message::Info("Successful Request".to_string())
    }

    fn get_state(&self) -> [[(Option<Piece>, Option<Color>); 8]; 8] {
        self.move_api.get_state()
    }

    fn get_outcome(&mut self) -> Message {
        let is_checkmate = self.move_api.is_checkmate();
        let is_stalemate = self.move_api.is_stalemate();

        if is_checkmate {
            return Message::Info(format!(
                "{:?}",
                self.move_api.get_turn_color().get_opposite()
            ));
        }

        if is_stalemate {
            return Message::Info("Stalemate".to_string());
        }

        Message::Info("IsOngoing".to_string())
    }

    fn get_legal_moves(&mut self) -> Vec<Move> {
        self.move_api.get_legal_moves()
    }
}