use std::collections::HashSet;

use crate::chess::piece::Piece;
use crate::chess::crd::Crd;

pub struct Board {
    board: Vec<Vec<Option<Box<Piece>>>>,
    //0 - white, 1 - black
    pieces: [HashSet<(usize, usize)>; 2],
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
        
        let pieces = [HashSet::with_capacity(16), HashSet::with_capacity(16)];

        Self {
            board,
            pieces,
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
        self.pieces[0].clear();
        self.pieces[1].clear();
        for (i, arr) in board_i8.iter().enumerate() {
            for (j, item) in arr.iter().enumerate() {
                self.board[i][j] = match *item {
                    0 => None,
                    other => {
                        let piece = Some(
                            Box::new(
                                Piece::create(other)
                            )
                        );
                        if let Some(ref p) = piece {
                            self.pieces[(p.get_player()/2) as usize].insert((i, j));
                        }


                        piece
                    },
                }; 
                
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

        if let Some(ref t) = tmp {
            if t.get_player() > 0 {
                self.pieces[(t.get_player()/2) as usize].remove(&location.get_tuple());
                self.pieces[(t.get_player()/2) as usize].insert(target.get_tuple());
            }
            
        }
        self.board[target.x() as usize][target.y() as usize] = tmp;

    }   

    pub fn remove_piece(&mut self, target: &Crd) {
        let piece = self.board[target.x() as usize][target.y() as usize].take();

        if let Some(ref t) = piece {
            if t.get_player() > 0 {
                self.pieces[(t.get_player()/2) as usize].remove(&target.get_tuple());
            } 
        }
    }

    pub fn capture(&mut self, location: &Crd, target: &Crd) {
        let m_piece = self.board[location.x() as usize][location.y() as usize].as_ref();
        let r_piece = self.board[target.x() as usize][target.y() as usize].as_ref();

        if let Some(ref m) = m_piece {
            if let Some(ref r) = r_piece {
                if m.get_player() > 0 && r.get_player() > 0 {

                    self.pieces[(r.get_player()/2) as usize].remove(&target.get_tuple());

                    self.pieces[(m.get_player()/2) as usize].remove(&location.get_tuple());
                    self.pieces[(m.get_player()/2) as usize].insert(target.get_tuple());
                } 
            }  
        }

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
        if let None = crd {
            return true;
        }
        self.is_piece(crd)
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

    pub fn get_enemy_piece(&self, crd: &Option<Crd>, current_player: i8) -> Option<&Box<Piece>> {
        match crd {
            Some(c) => {
                match self.board[c.x() as usize][c.y() as usize] {
                    Some(ref p) => {
                        if p.get_player() == [1, 2][(current_player % 2) as usize] {
                            return Some(p);
                        }
                        None
                    },
                    None => None, 
                }
            },
            None => None,
        }
    }
}