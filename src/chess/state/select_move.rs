
use crate::chess::state::State;
use crate::chess::code::Code;
use crate::chess::crd::Crd;
use crate::chess::context::Context;



impl State {


    pub fn select_move_actions(_chess_context: &mut Context) {
        //chess_context.change_player();
    }
    
    pub fn select_move_handler(chess_context: &mut Context, crd: Crd) -> Code {

        if chess_context.is_player_piece(&crd) {
            if chess_context.check_possible_moves(&crd) {
                chess_context.set_player_crd(crd);
                chess_context.change_state(State::MoveState);
                return Code::ReselectPiece;
            }
            chess_context.change_state(State::SelectPieceState);
            return Code::NoneMoves;
        }

        if chess_context.check_possible_move(&crd) {
            if Self::move_piece(chess_context, &crd) {

                // if chess_context.is_check() {
                //     if chess_context.is_mate() {

                //     }
                // }
                
                chess_context.change_player();
                chess_context.change_possible_moves();
                chess_context.change_state(State::SelectPieceState);

            }
        }

        chess_context.change_state(State::SelectPieceState);
        Code::None
    }

    fn move_piece(chess_context: &mut Context, crd: &Crd) -> bool {
        if chess_context.castling(&crd) {
            return true;
        }
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