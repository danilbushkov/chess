
mod piece;
mod state;
mod board;
pub mod code;
pub mod crd;


use crate::chess::code::Code;
use crate::chess::state::State;
use crate::chess::board::Board;
use crate::chess::crd::Crd;
use crate::chess::piece::Piece;




pub struct Chess {
    player: i8,
    board: Board,
    state: State,
    moves: Vec<Crd>,
    input_crd: Crd,
}

impl Chess {

    pub fn create() -> Self{
        Self {
            player: 1,
            moves: Vec::new(),
            board: Board::create(),
            state: State::None,
            input_crd: Crd::default(),
        }
    }


    pub fn change_state(&mut self,state: State){
        self.state = state;
    }


    pub fn handler(&mut self, crd: Crd) -> Code {
        if !Board::check_borders(&crd) {
            return Code::IncorrectCrd;
        }
        self.input_crd = crd;
        self.state.handler()
    }

    pub fn get_board_i8(&self) -> [[i8; 8]; 8] {
        self.board.get_board()
    }

    pub fn init(&mut self){
        self.board.init();
    }

    pub fn get_moves(&mut self, moves: Vec<Crd>) {
        self.moves = moves;
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_crd(&self) -> &Crd {
        &self.input_crd
    }

    pub fn get_piece(&self) -> &Piece {
        self.board.get_piece(&self.input_crd)
    }

}