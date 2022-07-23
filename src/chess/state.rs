mod select_piece;


use crate::chess::state::select_piece::SelectPieceState;
use crate::chess::Chess;
use crate::chess::code::Code;
use crate::chess::crd::Crd;

pub enum State {
    SelectPieceState(SelectPieceState),
    MoveState,
    CheckState,
    ResultState,
    None,
}

impl State {
    // pub fn begin_state(chess: &mut Chess) -> Self {
    //     //Self::SelectPieceState(SelectPieceState::create(chess))
    // }

    pub fn handler(&mut self, crd: Crd) -> Code {
        match self {
            State::SelectPieceState(s) => s.handler(crd),
            _ => Code::None,
        }

    }
}