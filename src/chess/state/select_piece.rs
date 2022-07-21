

use crate::chess::Chess;
use crate::chess::code::Code;


pub struct SelectPieceState<'a> {
    chess: &'a Chess<'a>,
}


impl<'a> SelectPieceState<'a> {
    pub fn create(chess: &'a Chess) -> Self {
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