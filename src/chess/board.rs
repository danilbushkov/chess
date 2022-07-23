

use crate::chess::piece::Piece;
use crate::chess::crd::Crd;

pub struct Board {
    board: Vec<Vec<Box<Piece>>>,
}


impl Board {
    
    pub fn create() -> Self {
        let mut board: Vec<Vec<Box<Piece>>> = Vec::with_capacity(8);
        for i in 0..8 {
            board.push(Vec::with_capacity(8));
            for _ in 0..8 {
                board[i].push(Box::new(Piece::None));
            }
        }

        Self {
            board,
        }
    }

    pub fn get_board(&self) -> [[i8; 8]; 8] {
        let mut board: [[i8; 8]; 8] = [[0; 8]; 8];

        for (i, arr) in self.board.iter().enumerate() {
            for (j, item) in arr.iter().enumerate() {
                board[i][j] = item.get_code();
            }
        }

        board
    }

    pub fn init(&mut self){
        self.from(self.start_board());
    }


    pub fn from(&mut self, board_i8: [[i8; 8]; 8]) {
        for (i, arr) in board_i8.iter().enumerate() {
            for (j, item) in arr.iter().enumerate() {
                self.board[i][j] = Box::new(Piece::create(*item));
            } 
        }
    }

    fn start_board(&self) -> [[i8; 8]; 8] {
        [
            [4, 6, 8,10, 12,8, 6, 4],
            [2, 2, 2, 2, 2, 2, 2, 2],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1, 1, 1, 1],
            [3, 5, 7, 9, 11,7, 5, 3],
        ]
    }

    pub fn check_borders(crd: &Crd) -> bool{
        (crd.x() >= 0 && crd.x() < 8) && (crd.y() >= 0 && crd.y() < 8)
    }

    pub fn get_piece(&self, crd: &Crd) -> &Box<Piece> {

        &self.board[crd.x() as usize][crd.y() as usize]
        
    }

    pub fn swap(a: &Crd, b: &Crd) {
        
    }
    

}