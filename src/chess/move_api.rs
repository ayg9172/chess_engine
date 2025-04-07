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

    pub fn get_pseudo_moves_color(&self, color: Color) -> Vec<Move> {
        self.move_generator.get_moves_color(self.get_board_ref(), color)
    }

    pub fn get_board_ref(&self) -> &Board {
        self.move_executor.get_board_ref()
    }

    pub fn get_state(&self) -> [[(Option<Piece>, Option<Color>); 8]; 8] {
        return self.get_board_ref().get_state();
    }

    pub fn get_piece_move_count(&self, color: Color) -> u32 {
        self.move_generator.get_move_count_color(self.get_board_ref(), color)
    }

    pub fn get_legal_moves(&mut self) -> Vec<Move> {
        // TODO: make this run at most once per turn
        let mut out = Vec::new();
        let pseudolegal: Vec<Move> = self.move_generator.get_moves(self.get_board_ref());
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

    pub fn get_evaluation(&self) -> f64 {

        let w_score = self.get_board_ref().get_material_score(Color::White);
        let b_score = self.get_board_ref().get_material_score(Color::Black);
        let w_dev = self.get_pseudo_moves_color(Color::White).len() as f64;
        let b_dev = self.get_pseudo_moves_color(Color::Black).len() as f64;

        let w_queen = self.move_generator.get_piece_moves(self.get_board_ref(), Piece::Queen, Color::White).len() as f64;
        let b_queen = self.move_generator.get_piece_moves(self.get_board_ref(), Piece::Queen, Color::Black).len() as f64;

        let overextend_w = w_queen;
        let overextend_b = b_queen;

        w_score - b_score + (w_dev - b_dev) * 0.001 + 0.001 * b_queen / w_queen
    }

    pub fn perft(&mut self, depth: u64) -> (u64, Duration) {
        if depth == 1 {

            let mut dur = Duration::new(0, 0);

            let now = Instant::now();

            // Yes this is not pretty, the function is in-lined
            // so we can accurately time pseudolegal moves
            // with less effort
            let count = {
                let mut out = Vec::new();
                let pseudolegal: Vec<Move> = self.move_generator.get_moves(self.get_board_ref());
                dur += now.elapsed();
                let attack_color = self.get_board_ref().turn.get_opposite();

                for m in pseudolegal {
                    self.move_executor.exec_move(m);
                    let is_check = self.is_check(attack_color);
                    self.move_executor.undo_move();

                    if !is_check {
                        out.push(m);
                    }
                }
                out
            }
            .len() as u64;

            return (count, dur);
        }

        let now = Instant::now();
        let mut dur = Duration::new(0, 0);

        let pseudolegal: Vec<Move> = self.move_generator.get_moves(self.get_board_ref());
        dur += now.elapsed();

        let attack_color = self.get_board_ref().turn.get_opposite();

        let mut out = 0;

        for m in pseudolegal {
            self.move_executor.exec_move(m);

            let is_check = self.is_check(attack_color);

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
