

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

}