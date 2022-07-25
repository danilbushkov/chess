

use crate::chess::state::State;
use crate::chess::code::Code;
use crate::chess::crd::Crd;
use crate::chess::context::Context;



//SelectPieceState
impl State {
    



    pub fn select_piece_handler(chess_context: &mut Context, crd: Option<Crd>) -> Code {
        
        if let None = crd  {
            chess_context.change_state(State::SelectPieceState);
            return Code::IncorrectCrd;
        }

        if !chess_context.is_player_piece() {
            chess_context.change_state(State::SelectPieceState);
            return Code::NonePiece;
        }

        chess_context.set_move_crd(crd);

        chess_context.get_possible_moves();
        if !chess_context.check_moves() {
            chess_context.change_state(State::SelectPieceState);
            return Code::NoneMoves;
        }

        chess_context.change_state(State::SelectPieceState);
        Code::None
    }

  
}