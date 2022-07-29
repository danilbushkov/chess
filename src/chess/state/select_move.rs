
use crate::chess::state::State;
use crate::chess::code::Code;
use crate::chess::crd::Crd;
use crate::chess::context::Context;



impl State {


    pub fn select_move_actions(chess_context: &mut Context) {
        //chess_context.change_player();
    }
    
    pub fn select_move_handler(chess_context: &mut Context, crd: Option<Crd>) -> Code {
        
        if let None = crd  {
            chess_context.change_state(State::MoveState);
            return Code::IncorrectCrd;
        }

        if chess_context.is_player_piece(&crd) {
            let moves = chess_context.get_possible_moves(&crd);
            if moves.is_empty() {
                chess_context.change_state(State::SelectPieceState);
                return Code::NoneMoves;
            }
            chess_context.set_moves(moves);
            chess_context.set_piece_crd(crd);
            chess_context.change_state(State::MoveState);
            return Code::ReselectPiece;
        }

        if chess_context.check_possible_move(&crd) {
            if Self::move_piece(chess_context, &crd) {
                chess_context.change_player();
                chess_context.change_state(State::SelectPieceState);

            }
        }

        chess_context.change_state(State::SelectPieceState);
        Code::None
    }

    fn move_piece(chess_context: &mut Context, crd: &Option<Crd>) -> bool {
        if chess_context.en_passant(&crd) {
            return true;
        }
        if chess_context.move_piece(&crd) {
            return true;
        }
        if chess_context.capture(&crd) {
            return true;
        }
        return false;
    }


    

  
}