mod select_piece;


use crate::chess::state::select_piece::SelectPieceState;
use crate::chess::Chess;
use crate::chess::code::Code;
use crate::chess::crd::Crd;

pub enum State<'a> {
    SelectPieceState(SelectPieceState<'a>),
    MoveState,
    CheckState,
    ResultState,
    None,
}

impl<'a> State<'a> {
    pub fn begin_state(chess: &'a Chess) -> Self {
        Self::SelectPieceState(SelectPieceState::create(&chess))
    }

    pub fn handler(&self, crd: Crd) -> Code {
        match self {
            State::SelectPieceState(s) => s.handler(crd),
            _ => Code::None,
        }

    }
}