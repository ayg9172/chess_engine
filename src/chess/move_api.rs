use super::{
    board::Board, cmove::Move, color::Color, move_executor::MoveExecutor,
    move_generator::MoveGenerator, piece::Piece,
};
use std::time::{Duration, Instant};

pub struct MoveAPI {
    move_generator: MoveGenerator,
    move_executor: MoveExecutor,
}

impl MoveAPI {
    pub fn new(fen: &str) -> Self {
        let board = Board::new(fen);

        MoveAPI {
            move_generator: MoveGenerator::new(),
            move_executor: MoveExecutor::new(board),
        }
    }

    pub fn get_board_ref(&self) -> &Board {
        self.move_executor.get_board_ref()
    }

    pub fn get_state(&self) -> [[(Option<Piece>, Option<Color>); 8]; 8] {
        return self.get_board_ref().get_state();
    }

    pub fn get_legal_moves(&mut self) -> Vec<Move> {
        // TODO: make this run at most once per turn
        let mut out = Vec::new();
        let pseudolegal: Vec<Move> = self
            .move_generator
            .get_moves(self.get_board_ref());
        let attack_color = self.get_board_ref().turn.get_opposite();

        for m in pseudolegal {
            self.move_executor.exec_move(m);
            let is_check = self.is_check(attack_color);
            self.move_executor.undo_move();

            // TODO: better check check system
            if !is_check {
                out.push(m);
            }
        }
        out
    }

    pub fn exec_move(&mut self, m: Move) {
        self.move_executor.exec_move(m);
    }

    pub fn undo_move(&mut self) {
        self.move_executor.undo_move();
    }

    fn is_check(&self, attack_color: Color) -> bool {
        self.move_generator
            .is_attacked_king(self.get_board_ref(), attack_color)
    }

    pub fn is_checkmate(&mut self) -> bool {
        let attack_color = self.get_board_ref().turn.get_opposite();
        self.get_legal_moves().is_empty() && self.is_check(attack_color)
    }

    pub fn is_stalemate(&mut self) -> bool {
        let attack_color = self.get_board_ref().turn.get_opposite();
        self.get_legal_moves().is_empty() && !self.is_check(attack_color)
    }

    pub fn get_turn_color(&self) -> Color {
        return self.get_board_ref().turn;
    }

    pub fn perft(&mut self, depth: u64) -> (u64, Duration) {
        if depth == 0 {
            return (1, Duration::new(0, 0));
        }

        let pseudolegal: Vec<Move> = self
            .move_generator
            .get_moves(self.get_board_ref());

        let attack_color = self.get_board_ref().turn.get_opposite();

        let mut out = 0;

        let mut dur = Duration::new(0, 0);
        for m in pseudolegal {
            self.move_executor.exec_move(m);

            let now = Instant::now();
            let is_check = self.is_check(attack_color);
            dur += now.elapsed();

            if !is_check {
                let rval = self.perft(depth - 1);
                out += rval.0;
                dur += rval.1;
            }

            self.move_executor.undo_move();
        }
        (out, dur)
    }

    #[allow(dead_code)]
    pub fn perft_divide(&mut self, depth: u64) {
        let ms = self.get_legal_moves();
        for m in ms {
            self.exec_move(m);
            println!(
                "{} :: {} ___ {}",
                m.to_string(),
                self.perft(depth - 1).0,
                self.get_board_ref().get_fen()
            );
            self.undo_move();
        }
    }
}
