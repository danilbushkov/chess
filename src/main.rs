

mod chess;

use crate::chess::Chess;

fn main() {
    let mut game: Chess = Chess::new();
    game.init_board();
    game.print_board();
    game.run();

}
