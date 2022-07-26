

use crate::chess::piece::Piece;
use crate::chess::crd::Crd;

pub struct Board {
    board: Vec<Vec<Option<Box<Piece>>>>,
}


impl Board {
    
    pub fn create() -> Self {
        let mut board: Vec<Vec<Option<Box<Piece>>>> = Vec::with_capacity(8);
        for i in 0..8 {
            board.push(Vec::with_capacity(8));
            for _ in 0..8 {
                board[i].push(None);
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
                board[i][j] = match item {
                    Some(i) => i.get_code(),
                    None => 0,
                }
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
                self.board[i][j] = match *item {
                    0 => None,
                    other => Some(Box::new(Piece::create(other))),
                }
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

    

    pub fn get_piece(&self, crd: &Option<Crd>) -> Option<&Box<Piece>> {

        match crd {
            Some(c) => self.board[c.x() as usize][c.y() as usize].as_ref(),
            None => None,
        }
        
        
    }

    pub fn move_piece(&mut self, location: &Crd, target: &Crd) {
        let tmp = self.board[location.x() as usize][location.y() as usize].take();
        self.board[target.x() as usize][target.y() as usize] = tmp;
    }   

    pub fn capture(&mut self, location: &Crd, target: &Crd) {
        self.board[target.x() as usize][target.y() as usize] 
            = self.board[location.x() as usize][location.y() as usize].take();
    }
    

    pub fn is_piece(&self, crd: &Option<Crd>) -> bool {
        match self.get_piece(crd) {
            Some(p) => match **p {
                Piece::None => false,
                _ => true,
            },
            None => false,
        }
    }


    pub fn is_piece_or_border(&self, crd: &Option<Crd>) -> bool {
        match self.get_piece(crd) {
            Some(p) => match **p {
                Piece::None => false,
                _ => true,
            },
            None => true,
        }
    }

    pub fn is_enemy_piece(&self, crd: &Option<Crd>, current_player: i8) -> bool {
        match self.get_piece(crd) {
            Some(p) => match **p {
                Piece::None => false,
                ref other => other.get_player() == [1, 2][(current_player % 2) as usize],
            },
            None => false,
        }

    }
}