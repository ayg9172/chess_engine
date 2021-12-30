mod chess;
use chess::fen;
use chess::move_api::MoveAPI;
use std::time::Instant;

fn _perft_up_to(fen: &str, depth: u64) {
    println!("=================");
    for d in 1..depth + 1 {
        println!("{}", perft(fen, d));
    }
}

fn perft(fen: &str, depth: u64) -> u64 {
    let mut move_api = MoveAPI::new(fen);

    let now = Instant::now();

    let (out, dur) = move_api.perft(depth);
    println!(
        "All:{}, PseudoGen:{}",
        now.elapsed().as_secs_f64(),
        dur.as_secs_f64()
    );
    out
}

fn _all_perft() {
    _perft_up_to(fen::STARTING_BOARD, 6);
    _perft_up_to(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0",
        5,
    );
    _perft_up_to("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 0", 7);
    _perft_up_to(
        "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1 ",
        4,
    );
    _perft_up_to(
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ",
        5,
    );
    _perft_up_to(
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10  ",
        5,
    );
    _perft_up_to("n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 6", 5);
}

#[rustfmt::skip]
fn main() {
    _all_perft();
}
