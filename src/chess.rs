//  white/black
// 0 - empty cell
// 1/2 - pawn
// 3/4 - rook
// 5/6 - knight
// 7/8 - bishop
// 9/10 - queen
// 11/12 - king

// player: 
// 1 - white
// 2 - black


mod piece;
mod state;
mod board;
pub mod code;
pub mod crd;


use crate::chess::code::Code;
use crate::chess::state::State;
use crate::chess::board::Board;
use crate::chess::crd::Crd;




pub struct Chess {
    player: i8,
    board: Board,
    state: State,
    moves: Vec<Crd>,

}

impl Chess {

    pub fn create() -> Self{
        Self {
            player: 1,
            moves: Vec::new(),
            board: Board::create(),
            state: State::SelectPieceState,
        }
    }


    pub fn change_state(&mut self,state: State){
        self.state = state;
    }


    pub fn handler(&self, crd: Crd) -> Code {

        Code::None
    }

    

}