

mod console_game;


use chess::chess::Chess;
use console_game::ConsoleGame;
use chess::chess::state::State;

fn main() {
    let mut chess = Chess::create();
    let mut game = ConsoleGame::create(chess);
    
    game.run();

}
