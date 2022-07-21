

use crate::chess::Chess;
use crate::chess::code::Code;


pub struct SelectPieceState {
    chess: &Chess,
}


impl SelectPieceState {
    pub fn create(chess: &Chess) -> Self {
        Self {
            chess,
        }
    }

    pub fn handler(&self) -> Code {
        self.get_moves();

        Code::None
    }

    pub fn get_moves(&self) {
        //self.chess.get_moves();
    }
}