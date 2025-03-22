pub mod utils;
pub mod game;
pub mod rayattacks;
use crate::game::*;

fn main() {
    // let game = Game::init();
    // let fenstr = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let fenstr = "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2"; 
    let game = Game::read_fen(&fenstr);
    println!("{}",game.board_rep());
    println!("Raw FEN: {}", fenstr);
    println!("Active Color: {:?}", game.active_color); 
    println!("Castling Rights: {:?},  {:04b}", game.castling_rights, game.castling_rights.bits()); 
    println!("En Passant Square: {:?}", game.en_passant);
    println!("Ply: {:?}", game.ply); 
    println!("Full Moves: {:?}", game.fullmoves); 
}
