mod select_piece;



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
    


    pub fn handler(&self, context: &mut Context, crd: Option<Crd>) -> Code {
        match self {
            State::SelectPieceState => State::select_piece_handler(context, crd),
            _ => Code::None,
        }

    }
}