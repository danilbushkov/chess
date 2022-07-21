mod select_piece;


use crate::chess::state::select_piece::SelectPieceState;
use crate::chess::Chess;
use crate::chess::code::Code;

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

    pub fn handler(&self) -> Code {
        match self {
            State::SelectPieceState(s) => s.handler(),
            _ => Code::None,
        }

    }
}