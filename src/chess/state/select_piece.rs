

use crate::chess::Chess;
use crate::chess::code::Code;
use crate::chess::crd::Crd;


pub struct SelectPieceState<'a> {
    chess: &'a Chess<'a>,
}


impl<'a> SelectPieceState<'a> {
    pub fn create(chess: &'a Chess) -> Self {
        Self {
            chess,
        }
    }

    pub fn handler(&self, crd: Crd) -> Code {
        if !self.chess.check_borders(&crd) {
            return Code::IncorrectCrd;
        }
        let piece = self.chess.get_piece_by_crd(&crd);
        if !self.chess.is_player_piece(piece) {
            return Code::NoPiece;
        }
        

        Code::None
    }

  
}