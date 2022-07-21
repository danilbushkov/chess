

use crate::chess::piece::Piece;

pub struct Board {
    board: [[Piece; 8]; 8],
}


impl Board {

    pub fn create() -> Self {
        Self {
            board: [[Piece::None; 8]; 8],
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
                self.board[i][j] = Piece::create(*item);
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

}