mod select_piece;
mod select_move;


use crate::chess::code::Code;
use crate::chess::crd::Crd;
use crate::chess::context::Context;

pub enum State {
    SelectPieceState,
    MoveState,
    CheckState,
    ResultState,
}

impl State {
    pub fn handler(&self, context: &mut Context, crd: Crd) -> Code {
        match self {
            State::SelectPieceState => State::select_piece_handler(context, crd),
            State::MoveState => State::select_move_handler(context, crd),
            _ => Code::NoneState,
        }

    }

    pub fn action_on_change(&self, context: &mut Context) {
        match self {
            State::SelectPieceState => State::select_piece_actions(context),
            State::MoveState => State::select_move_actions(context),
            _ => (),
        }
    }
}