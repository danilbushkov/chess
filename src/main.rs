

mod chess;
mod console_game;


use chess::Chess;
use console_game::ConsoleGame;

fn main() {
    let chess = Chess::create();
    let game = ConsoleGame::create(chess);
    
    game.run();

}
