mod select_piece;


use crate::chess::state::select_piece::SelectPieceState;
use crate::chess::Chess;
use crate::chess::code::Code;

pub enum State {
    SelectPieceState(SelectPieceState),
    MoveState,
    CheckState,
    ResultState,
    None,
}

impl State {
    pub fn begin_state(chess: &Chess) -> Self {
        Self::SelectPieceState(SelectPieceState::create(&chess))
    }

    pub fn handler(&self) -> Code {
        match self {
            State::SelectPieceState(s) => s.handler(),
            _ => Code::None,
        }

    }
}