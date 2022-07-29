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

    pub fn get_board(&self) -> [[usize; 8]; 8] {
        let mut board: [[usize; 8]; 8] = [[0; 8]; 8];

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


    pub fn from(&mut self, board_isize: [[usize; 8]; 8]) {
        self.pieces[0].clear();
        self.pieces[1].clear();
        for (i, arr) in board_isize.iter().enumerate() {
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

    fn start_board(&self) -> [[usize; 8]; 8] {
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

    

    pub fn get_piece(&self, crd: &Crd) -> Option<&Box<Piece>> {
        self.get_ref(crd.get_tuple())
    }

    // pub fn get_piece_by_crd(&self, crd: &Crd) -> Option<&Box<Piece>> {

    //     self.board[crd.x() ][crd.y() ].as_ref()
        
    // }

    // pub fn is_enemy_piece_by_crd(&self, crd: &Crd, current_player: isize) -> bool {
    //     match self.get_piece_by_crd(crd) {
    //         Some(p) => match **p {
    //             Piece::None => false,
    //             ref other => other.get_player() == [1, 2][(current_player % 2) ],
    //         },
    //         None => false,
    //     }
    // }

    // pub fn get_piece_mut(&mut self, crd: &Crd) -> Option<&mut Box<Piece>> {

    //     match crd {
    //         Some(c) => self.board[c.x() ][c.y() ].as_mut(),
    //         None => None,
    //     }
        
        
    // }

    // pub fn pawn_two_cells(&mut self, location: &Crd, target: &Crd) {
        
        
    // }


    pub fn move_piece(&mut self, location: &Crd, target: &Crd) -> bool {
        
        let mut tmp = self.take(location.get_tuple());

        if let Some(t) = &mut tmp {
            if t.get_player() > 0 {
                
                t.change_first_move();
                t.change_two_calls(location, target);

                self.pieces[(t.get_player()/2)].remove(&location.get_tuple());
                self.pieces[(t.get_player()/2)].insert(target.get_tuple());
                self.set(target.get_tuple(), tmp);
                return true;
            }
            
        }
        
        //self.board[target.x() ][target.y() ] = tmp;

        false
    }   

    pub fn remove_piece(&mut self, target: &Crd) -> bool {
        let piece = self.take(target.get_tuple());

        if let Some(ref t) = piece {
            if t.get_player() > 0 {
                self.pieces[(t.get_player()/2) ].remove(&target.get_tuple());
                return true;
            } 
        }
        false
    }

    pub fn capture(&mut self, location: &Crd, target: &Crd) -> bool {

        if let Some(ref m) = self.get_ref(location.get_tuple()) {
            if let Some(ref r) = self.get_ref(target.get_tuple()) {
                let player = m.get_player();
                let enemy = r.get_player();
                if player == 0 || enemy == 0 {
                    return false;
                }

                
                let mut tmp = self.take(location.get_tuple());
                if let Some(p) = &mut tmp {
                    p.change_first_move();
                    self.set(target.get_tuple(), tmp);
                
                    self.pieces[enemy/2].remove(&target.get_tuple());
                    self.pieces[player/2].remove(&location.get_tuple());
                    self.pieces[player/2].insert(target.get_tuple());
                    return true;
                }
            }  
        }
        false
    }
    

    pub fn is_piece(&self, crd: &Crd) -> bool {
        match self.get_piece(crd) {
            Some(p) => match **p {
                Piece::None => false,
                _ => true,
            },
            None => false,
        }
    }
    pub fn is_piece_by_crd(&self, crd: &Crd) -> bool {
        match self.get_piece(crd) {
            Some(p) => match **p {
                Piece::None => false,
                _ => true,
            },
            None => false,
        }
    }


    // pub fn is_piece_or_border(&self, crd: &Crd) -> bool {
    //     if let None = crd {
    //         return true;
    //     }
    //     self.is_piece(crd)
    // }

    pub fn is_enemy_piece(&self, crd: &Crd, current_player: usize) -> bool {
        match self.get_piece(crd) {
            Some(p) => match **p {
                Piece::None => false,
                ref other => other.get_player() == [1, 2][(current_player % 2)],
            },
            None => false,
        }

    }

    pub fn get_enemy_piece(&self, crd: &Crd, current_player: usize) -> Option<&Box<Piece>> {
        match self.get_ref(crd.get_tuple()) {
            Some(ref p) => {
                if p.get_player() == [1, 2][(current_player % 2)] {
                    return Some(p);
                }
                None
            },
            None => None, 
        }   
    }

    pub fn get_player_piece(&self, crd: &Crd, current_player: usize) -> Option<&Box<Piece>> {
        match self.get_ref(crd.get_tuple()) {
            Some(ref p) => {
                if p.get_player() == [1, 2][(current_player % 2)] {
                    return Some(p);
                }
                None
            },
            None => None, 
        }   
    }

    pub fn is_player_piece(&self, crd: &Crd, current_player: usize) -> bool {
        match self.get_piece(crd) {
            Some(p) => match **p {
                Piece::None => false,
                ref other => other.get_player() == current_player,
            },
            None => false,
        }
        
    }

    pub fn get_ref(&self, (x, y): (usize, usize)) -> Option<&Box<Piece>> {
        self.board[x][y].as_ref()
    }

    pub fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut Box<Piece>> {
        self.board[x][y].as_mut()
    }
    pub fn take(&mut self, (x, y): (usize, usize)) -> Option<Box<Piece>> {
        self.board[x][y].take()
    }

    pub fn set(&mut self, (x, y): (usize, usize), value: Option<Box<Piece>>) {
        self.board[x][y] = value;
    }
    
}