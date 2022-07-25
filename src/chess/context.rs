

use crate::chess::board::Board;
use crate::chess::crd::Crd;
use crate::chess::state::State;
use crate::chess::piece::Piece;

pub struct Context {
    player: i8,
    board: Board,
    state: Option<State>,
    moves: Vec<Crd>,
    piece_crd: Option<Crd>,
    move_crd: Option<Crd>,
}


impl Context {
    pub fn create() -> Self {

        Self {
            player: 1,
            moves: Vec::new(),
            board: Board::create(),
            state: Some(State::SelectPieceState),
            piece_crd: Crd::default(),
            move_crd: Crd::default(),
        }
    }

    pub fn get_state(&mut self) -> Option<State> {
        self.state.take()
    }

    pub fn change_state(&mut self, state: State){
        self.state = Some(state);
    }


  //------------------------------------------  

    pub fn get_possible_moves() {

    }


  //------------------------------------------
    pub fn change_player(&mut self) {
        self.player = match self.player {
            1 => 2,
            2 => 1,
            _ => 1,
        };
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

    pub fn get_piece_crd(&self) -> &Option<Crd> {
        &self.piece_crd
    }

    pub fn set_piece_crd(&mut self, crd: Option<Crd>) {
        self.piece_crd = crd;
    }

    pub fn get_move_crd(&self) -> &Option<Crd> {
        &self.move_crd
    }

    pub fn set_move_crd(&mut self, crd: Option<Crd>) {
        self.move_crd = crd;
    }

    pub fn get_piece(&self) -> Option<&Box<Piece>> {
        self.board.get_piece(&self.get_piece_crd())
    }
    pub fn get_piece_by_crd(&self, crd: &Option<Crd>) -> Option<&Box<Piece>> {
        self.board.get_piece(crd)
    }

    pub fn check_moves(&self) -> bool {
        !self.moves.is_empty()
    }

    pub fn is_player_piece(&self) -> bool {
        match self.get_piece() {
            Some(piece) => match &**piece {
                Piece::None => false,
                piece => piece.get_player() == self.player,
            },
            None => false,
        }
        
    }

}