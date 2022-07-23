
mod piece;
mod state;
mod board;
mod algorithm;
pub mod code;
pub mod crd;



use crate::chess::code::Code;
use crate::chess::state::State;
use crate::chess::board::Board;
use crate::chess::crd::Crd;
use crate::chess::piece::Piece;




pub struct Chess<'a> {
    player: i8,
    board: Board,
    state: State<'a>,
    moves: Vec<Crd>,
    piece_crd: Crd,
    move_crd: Crd,
}

impl<'a> Chess<'a> {

    pub fn create() -> Self{
        Self {
            player: 1,
            moves: Vec::new(),
            board: Board::create(),
            state: State::None,
            piece_crd: Crd::default(),
            move_crd: Crd::default(),
        }
    }


    pub fn change_state(&mut self,state: State<'a>){
        self.state = state;
    }


    pub fn handler(&mut self, crd: Crd) -> Code {
        
        self.state.handler(crd)
    }

    pub fn change_player(&mut self) {
        self.player = match self.player {
            1 => 2,
            2 => 1,
            _ => 1,
        };
    }




    pub fn check_borders(&self, crd: &Crd) -> bool {
        Board::check_borders(crd)
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

    pub fn get_piece_crd(&self) -> &Crd {
        &self.piece_crd
    }

    pub fn set_piece_crd(&mut self, crd: Crd) {
        self.piece_crd = crd;
    }

    pub fn get_move_crd(&self) -> &Crd {
        &self.move_crd
    }

    pub fn set_move_crd(&mut self, crd: Crd) {
        self.move_crd = crd;
    }

    pub fn get_piece(&self) -> &Box<Piece> {
        self.board.get_piece(&self.get_piece_crd())
    }
    pub fn get_piece_by_crd(&self, crd: &Crd) -> &Box<Piece> {
        self.board.get_piece(crd)
    }

    pub fn is_player_piece(&self, piece: &Box<Piece>) -> bool {
        match &**piece {
            Piece::None => false,
            piece => piece.get_player() == self.player,
        }
    }

}