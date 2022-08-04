use std::collections::HashSet;


use crate::chess::crd::Crd;
use crate::chess::board::Board;

pub struct Pawn {
    player: usize,
    two_cells: bool,
    first_move: bool,
    
}

impl Pawn {
    pub fn create(player: usize) -> Self{
        Self{
            player,
            two_cells: false,
            first_move: true,
            
        }
    }

    pub fn get_player(&self) -> usize {
        self.player
    }

    pub fn get_moves(&self, crd: &Crd, board: &Board, cover: bool) -> HashSet<Crd> {
        let mut moves: HashSet<Crd> = HashSet::new();
        
        if !cover {
            moves.extend(self.possible_moves(crd, board));
            moves.extend(self.en_passant(crd, board));
        }
        moves.extend(self.possible_capture(crd, board, cover));
        

        moves
    }

    pub fn possible_moves(&self, crd: &Crd, board: &Board) -> HashSet<Crd> {
        let mut moves: HashSet<Crd> = HashSet::new();
        let direction = [1, -1]; //black, white



        let c = Crd::create(
            crd.x() + direction[(self.player % 2) ],
            crd.y());

        if let Some(c) = c {
            if !board.is_piece(&c) {
                moves.insert(c);
                if self.first_move {
                    let c = Crd::create(
                        crd.x() + 2*direction[(self.player % 2) ], 
                        crd.y());
                    if let Some(c) = c {
                        if !board.is_piece(&c) {
                            moves.insert(c);
                        }
                    }
                }
            }
        }
        
        moves
        
    }

    pub fn possible_capture(&self, crd: &Crd, board: &Board, cover: bool) -> HashSet<Crd> {
        let mut moves: HashSet<Crd> = HashSet::new();
        let direction = [1, -1]; //black, white
        
        for b in direction {
            let c = Crd::create(
                crd.x() + direction[(self.player % 2) ],
                crd.y() + b);
            if let Some(c) = c {
                if board.is_enemy_piece(&c, self.player) || 
                (cover && board.is_player_piece(&c, self.player)) {
                    moves.insert(c);
                }
            }
            
        }

        moves 
    }

    pub fn en_passant(&self, crd: &Crd, board: &Board) -> HashSet<Crd> {
        let mut moves: HashSet<Crd> = HashSet::new();
        let direction = [1, -1]; //black, white
        for b in direction {
            let crd_1 = Crd::create(
                crd.x(),
                crd.y() + b, 
            );
            if let Some(c) = crd_1 {
                let enemy_piece = board.get_enemy_piece(&c, self.player);
                if let Some(p) = enemy_piece {
                    if p.is_en_passant() {
                        let crd_2 = Crd::create(
                            crd.x() + direction[(self.player % 2)], 
                            crd.y() + b);
                        if let Some(c) = crd_2 {
                            if !board.is_piece(&c) {
                                moves.insert(c);
                            }
                        }
                    }
                }
            }
            

        }

        moves 
    }




    pub fn two_cells(&self) -> bool {
        self.two_cells
    }

    pub fn set_two_cells(&mut self, value: bool) {
        self.two_cells = value;
    }

    pub fn change_first_move(&mut self) {
        if self.first_move {
            self.first_move = false;
        }
    }

    pub fn change_two_calls(&mut self, location: &Crd, target: &Crd) {
        if (location.x() - target.x()).abs() == 2 {
            self.two_cells = true;
        } else {
            self.two_cells = false;
        }
        
    }

    pub fn first_move(&self) -> bool {
        self.first_move
    }
}