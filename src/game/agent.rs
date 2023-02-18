
use crate::chess::cmove::Move;

struct RandomAgent {

}


struct TomatoAgent {

}

pub trait Agent {
    fn getAgentMove() -> Move;
}


impl Agent for RandomAgent {
    fn getAgentMove() -> Move {
        //Move::new(Square::E2, Square::E4, Piece::new("c"), None )
        todo!();
    }
}

