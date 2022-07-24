

use crate::chess::Chess;
use crate::chess::state::State;
use crate::chess::code::Code;
use crate::chess::crd::Crd;
use crate::chess::context::Context;



//SelectPieceState
impl State {
    

    pub fn select_piece_handler(chess_context: &mut Context, crd: Crd) -> Code {
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