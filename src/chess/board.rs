

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
}