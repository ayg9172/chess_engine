use actix_web::{ web, App, HttpServer, HttpRequest, HttpResponse };
use actix_cors::Cors;
use serde::{Deserialize, Serialize};

use crate::{chess::{move_api::{MoveAPI, self}, position::Position, cmove::Move, fen, piece::{fen_to_piece, Piece, self}, color::Color}, game::agent::{RandomAgent, Agent, TomatoAgent}, api::chess_api::{GameAPI, ChessGame, Message::{Info, Error}}};


#[derive(Serialize, Deserialize, Debug)]
struct MoveRequest {
    fen: String,
    startRow: u32, 
    startCol: u32,
    endRow: u32, 
    endCol: u32,
    promotion: String,
    depth: u32,
}


#[derive(Serialize, Deserialize, Debug)]
struct MoveResponse {


    isLegal: bool,
    fen: String,
    isGameOver: bool,

    turn: String,

    startRow: u32, 
    startCol: u32,
    endRow: u32, 
    endCol: u32,  
    message: String,
}
impl MoveResponse {

    fn illegal_move(message: String) -> HttpResponse {
        let isLegal = false;
        let fen = String::new();
        let isGameOver = false;
        let turn = String::new();
        let startRow = 0;
        let startCol = 0;
        let endRow = 0;
        let endCol = 0;
    
        HttpResponse::Ok().json(MoveResponse { 
            isLegal, 
            fen, 
            isGameOver, 
            turn, 
            startRow, 
            startCol, 
            endRow, 
            endCol,
            message
        })
    }
    
}



// todo wrap this in result in case req fails
async fn try_move(data: web::Json<MoveRequest>, _req: HttpRequest) -> HttpResponse {
    println!("request got!");
    let start = Position::new(data.startRow as i8, data.startCol as i8);
    let end = Position::new(data.endRow as i8, data.endCol as i8);

    println!("Attempting: {} {}", start.to_string(), end.to_string());
    let mut move_api = MoveAPI::new(&data.fen);

    let mut game_api = ChessGame::new(move_api);
    game_api.dbg();
    let option = game_api.get_piece(start);

    let piece;
    match option {
        None => return MoveResponse::illegal_move(String::from("No piece selected")),
        Some(p) => piece = p,
    }

    let mut user_move = Move::new(
        start,  
        end,
        piece,
        None
    );

    let result = game_api.try_move(user_move);
    
    match result {
        Info(msg) => {
            println!("{}",msg);
            let mut agent = TomatoAgent::new(data.depth as usize);

            println!("Agent thinking...");
            let option = agent.getAgentMove(&mut game_api);

            let mut startRow = 0;
            let mut startCol = 0;
            let mut endRow = 0;
            let mut endCol = 0;
            let mut agentMoveString = String::from("");
            match option {
                Some(chess_move) => {
                    let st = &chess_move.to_string();
                    agentMoveString += st;

                    game_api.try_move(chess_move); 
                    startRow = chess_move.start.get_row() as u32;
                    startCol = chess_move.start.get_col() as u32;
                    endRow = chess_move.end.get_row() as u32;
                    endCol = chess_move.end.get_col() as u32;

                    println!("Agent played a move");
                },
                None => { 
                    println!("Agent has no moves");
                },
            };

            let isLegal = true;
            let fen = game_api.get_fen();

            let isGameOver = match game_api.get_outcome() {
                Info(state) => state == "IsOngoing",
                Error(msg) => false,
            };

            let turn = match game_api.get_turn_color() {
                Color::White => String::from("white"),
                Color::Black => String::from("black"),
            };

            println!("Considered {} moves and found {} reductions", agent.count, agent.prunes);
            return HttpResponse::Ok().json(MoveResponse {
                isLegal,
                fen,
                isGameOver,
            
                turn,
            
                startRow, 
                startCol,
                endRow, 
                endCol,  
                message: String::from("Success! ".to_owned() + &agentMoveString),
            });
        }

        Error(msg) => {
            return MoveResponse::illegal_move(msg)
        }
    }
}

#[actix_web::main] // or #[tokio::main]
pub async fn server() -> std::io::Result<()> {
    println!("Starting server");

    HttpServer::new(|| {

        let cors = Cors::permissive()
        .allowed_origin("http://localhost:5173").send_wildcard(); // bug in actix requires http protocol specification?

        App::new()
        .wrap(cors)
        .service(web::resource("/requestMove").route(web::post().to(try_move)))
    })
    .bind(("127.0.0.1", 3131))?
    .run()
    .await
}