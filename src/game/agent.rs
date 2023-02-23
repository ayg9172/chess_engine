
use std::{cmp::Ordering, iter::Map, collections::{HashMap, VecDeque}};

use rand::{rngs::ThreadRng, SeedableRng, Rng};

use crate::{chess::{cmove::Move, move_api::MoveAPI, board, color::Color, piece}, api::chess_api::{GameAPI, ChessGame}};

pub struct RandomAgent {
    random_generator: ThreadRng
}



const GOOD_MOVE_COUNT: usize = 2;
pub struct TomatoAgent {
    random_generator: ThreadRng,
    good_ideas:  Vec<VecDeque<Move>>,
    max_depth: usize,
    pub prunes: usize,
    pub count: usize,
}


struct ScoredMove {
    chess_move: Move,
    board_value: f64, 

    is_good_idea: f64,
    freedom_value: f64,
    aggression_value: f64,
    piece_value: f64, 
}

impl ScoredMove {
    // todo make this function easy to change by creating subroutines
    pub fn compareTo(&self, other: &ScoredMove) -> Ordering {
        match self.is_good_idea.partial_cmp(&other.is_good_idea) {
            Some(Ordering::Equal) => {},
            None => {},
            Some(ord) => return ord,
        }
        match self.board_value.partial_cmp(&other.board_value) {
            Some(Ordering::Equal) => {},
            None => {},
            Some(ord) => return ord,
        }
        match self.aggression_value.partial_cmp(&other.aggression_value) {
            Some(Ordering::Equal) => {},
            None => {},
            Some(ord) => return ord,
        }
        match self.freedom_value.partial_cmp(&other.freedom_value) {
            Some(Ordering::Equal) => {},
            None => {},
            Some(ord) => return ord,
        }

        return Ordering::Equal;
        // todo dont calculate freedom value unless necessary?
        //self.piece_value.partial_cmp(&other.piece_value).unwrap()
    }
}

const MIN: f64 = -100000f64;
const MAX: f64 = 100000f64;

impl TomatoAgent {
    fn evaluate_move(&mut self, move_api : &mut MoveAPI, alpha: f64, beta: f64, color: f64, depth: usize) -> (f64, Option<Move>) {
        if depth >= self.max_depth  {
            // todo remove hardcode
            return (-move_api.get_evaluation() * color, None);
        }

        let mut a = alpha;

        let mut moves = move_api.get_legal_moves();

        if moves.len() == 0 {
            return (MIN, None); // todo
        }
        moves = self.order_moves(move_api, moves, color, depth);

        let mut best_value = MIN;
        let mut best_move = moves[0];

        for chess_move in moves {
            move_api.exec_move(chess_move);
            let value = -self.evaluate_move(move_api, -beta, -a, -color, depth + 1).0;
            move_api.undo_move();
            self.count += 1;

            if value > best_value {
                best_value = value;
                a = f64::max(a, value);
                if a >= beta {
                    self.prunes += 1;
                    //println!("Cutoff!");
                    
                    self.good_ideas[depth].push_back(chess_move);
                    if self.good_ideas[depth].len() > GOOD_MOVE_COUNT {
                        self.good_ideas[depth].pop_front();
                    }
                    break;
                }
                best_move = chess_move;
            }
        }

        return (best_value, Some(best_move));
    }
    pub fn choose_best_move(&mut self, move_api : &mut MoveAPI, color: f64) -> Move {
        // todo remove hardcode
        let res = self.evaluate_move(move_api, MIN, MAX , 1f64, 0);
        println!("Prediction: {}", res.0);
        res.1.unwrap()
    }

    


    pub fn order_moves(&mut self, move_api: &mut MoveAPI, moves: Vec<Move>, color: f64, depth: usize) -> Vec<Move> {
        //todo func is currently working for 1=black hardcoded
        let sColor = if color < 0.0 {
            Color::White
        } else {
            Color::Black
        };

        let oColor = if color < 0.0 {
            Color::Black
        } else {
            Color::White
        };


        let mut ordered_moves = Vec::new();

        for chess_move in &moves {
            move_api.exec_move(chess_move.clone());
            let board_value = color * -move_api.get_evaluation(); // todo negative is hardcoded (not anymore?)
            let freedom_value = move_api.get_piece_move_count(sColor) as f64;
            let aggression_value = freedom_value - move_api.get_piece_move_count(oColor) as f64;
            let piece_value = piece::get_piece_dev_value(chess_move.piece) as f64;
            let is_good_idea = if self.good_ideas[depth].contains(chess_move) {
                1f64
            } else {
                0f64
            };

            ordered_moves.push(ScoredMove {
                chess_move: chess_move.clone(),
                is_good_idea,
                board_value,
                piece_value,
                aggression_value,
                freedom_value,
            });
            move_api.undo_move();
        }

        ordered_moves.sort_by(|a, b| b.compareTo(a));

        let mut out = Vec::new();
        for scored_move in &ordered_moves {
            out.push(scored_move.chess_move);
        }


        return out;
    }



    pub fn new(max_depth: usize)-> Self {
        let mut random_generator = rand::thread_rng();  

        let mut good_ideas = Vec::new();

        // todo reduce length of dequeue?
        for _ in 0..(max_depth+2) {
            good_ideas.push(VecDeque::new());
        }

        TomatoAgent {
            random_generator,
            good_ideas,
            max_depth,
            prunes: 0,
            count: 0,
        }
    }


    

}

impl Agent for TomatoAgent {
    fn getAgentMove(&mut self, board_logic: &mut ChessGame) -> Option<Move> {
        //Move::new(Square::E2, Square::E4, Piece::new("c"), None )

        let color = if board_logic.get_turn_color() == Color::White {
            1f64
        } else {
            -1f64
        };

        let possible_moves = board_logic.get_legal_moves();
        if possible_moves.len() == 0 {
            return None;
        }

        let fen = board_logic.get_fen();
        let mut move_api = MoveAPI::new(&fen);
        
        return Some(self.choose_best_move(&mut move_api, color));
        

    }

}

pub trait Agent {
    fn getAgentMove(&mut self, board_logic: &mut ChessGame) -> Option<Move>;
}

impl RandomAgent {
    pub fn new()-> Self {
        let mut random_generator = rand::thread_rng();  

        

        RandomAgent {
            random_generator,
        }
    }
}

impl Agent for RandomAgent {
    fn getAgentMove(&mut self, board_logic: &mut ChessGame) -> Option<Move> {
        //Move::new(Square::E2, Square::E4, Piece::new("c"), None )

        let possible_moves = board_logic.get_legal_moves();
        if possible_moves.len() == 0 {
            return None;
        }
        
        let chosen_index: usize = self.random_generator.gen::<usize>() % possible_moves.len();
        let option = possible_moves.get(chosen_index);

        match option {
            Some(m) => Some(m.clone()),
            None => None,
        }


    }
}




