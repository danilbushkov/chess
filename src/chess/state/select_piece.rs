

use crate::chess::Chess;
use crate::chess::code::Code;
use crate::chess::crd::Crd;


pub struct SelectPieceState;


impl SelectPieceState {
    pub fn create() -> Self {
        Self{}
    }

    pub fn handler(&self, crd: Crd) -> Code {
        // if !chess.check_borders(&crd) {
        //     return Code::IncorrectCrd;
        // }
        // chess.set_move_crd(crd);
        // if !chess.is_player_piece() {
        //     return Code::NoPiece;
        // }


        Code::None
    }

  
}