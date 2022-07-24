mod select_piece;


//use crate::chess::state::State;
use crate::chess::Chess;
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
    // pub fn begin_state(chess: &mut Chess) -> Self {
    //     //Self::SelectPieceState(SelectPieceState::create(chess))
    // }

    pub fn handler(&self, context: &mut Context, crd: Crd) -> Code {
        match self {
            State::SelectPieceState => State::select_piece_handler(context, crd),
            _ => Code::None,
        }

    }
}