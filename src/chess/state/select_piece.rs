

use crate::chess::state::State;
use crate::chess::code::Code;
use crate::chess::crd::Crd;
use crate::chess::context::Context;



//SelectPieceState
impl State {
    
    pub fn select_piece_actions(_chess_context: &mut Context) {
        //chess_context.clear_moves();
    }



    pub fn select_piece_handler(chess_context: &mut Context, crd: Crd) -> Code {

        if !chess_context.is_player_piece(&crd) {
            chess_context.change_state(State::SelectPieceState);
            return Code::NonePiece;
        }

        if chess_context.check_possible_moves(&crd) {
            chess_context.set_player_crd(crd);
            chess_context.change_state(State::MoveState);
            return Code::None;
        }

        
        chess_context.change_state(State::SelectPieceState);
        return Code::NoneMoves;
        
        
    }

  
}