

use crate::chess::state::State;
use crate::chess::code::Code;
use crate::chess::crd::Crd;
use crate::chess::context::Context;



//SelectPieceState
impl State {
    
    pub fn select_piece_actions(chess_context: &mut Context) {
        chess_context.clear_moves();
    }



    pub fn select_piece_handler(chess_context: &mut Context, crd: Option<Crd>) -> Code {
        
        if let None = crd  {
            chess_context.change_state(State::SelectPieceState);
            return Code::IncorrectCrd;
        }

        
        if !chess_context.is_player_piece(&crd) {
            chess_context.change_state(State::SelectPieceState);
            return Code::NonePiece;
        }

        let moves = chess_context.get_possible_moves(&crd);
        if moves.is_empty() {
            chess_context.change_state(State::SelectPieceState);
            return Code::NoneMoves;
        }

        chess_context.set_moves(moves);
        chess_context.set_piece_crd(crd);
        chess_context.change_state(State::MoveState);
        Code::None
    }

  
}