use std::collections::HashSet;

use crate::chess::crd::Crd;
use crate::chess::board::Board;


pub struct King {
    player: usize,
    first_move: bool,
   
}

impl King {
    pub fn create(player: usize) -> Self{
        Self{
            player,
            first_move: true,
            
        }
    }

    pub fn get_player(&self) -> usize {
        self.player
    }


    pub fn get_moves(&self, crd: &Crd, board: &Board, cover: bool) -> HashSet<Crd> {
        
        if cover {
            self.attacks(crd, board)
        } else {
            self.limited_move(crd, board)
        }
        
    }




    pub fn attacks(&self, crd: &Crd, board: &Board) -> HashSet<Crd> {
        let mut moves: HashSet<Crd> = HashSet::new();
        let direction = [1, -1, 0];
        for a in direction {
            for b in direction {
                if let Some(move_crd) = Crd::create(crd.x()+a, crd.x()+b) {
                    if move_crd != *crd {
                        moves.insert(move_crd);
                    }
                }
                
            }
        }
        moves
    }


    pub fn limited_move(&self, crd: &Crd, board: &Board) -> HashSet<Crd> {
        let mut moves: HashSet<Crd> = HashSet::new();
        let direction = [1, -1, 0];

        let enemy_attacks = board.get_enemy_attacks(self.player);

        for a in direction {
            for b in direction {
                if let Some(move_crd) = Crd::create(crd.x()+a, crd.x()+b) {
                    if !board.is_player_piece(&move_crd, self.player) {
                        if !enemy_attacks.contains(&move_crd) {
                            moves.insert(move_crd);
                        }
                    }
                }
                
            }
        }
        if !enemy_attacks.contains(&crd) && self.first_move {
            moves.extend(self.castling(crd, board, enemy_attacks));
        }
        moves
    }

    pub fn castling(&self, crd: &Crd, board: &Board, enemy_attacks: HashSet<Crd>) -> HashSet<Crd> {
        let mut moves: HashSet<Crd> = HashSet::new();

        
        let x = [7, 0][self.player/2];

        for (y, d) in [(0, 1), (7, -1)] {
            if let Some(rook_crd) = Crd::create(x, y) {
                if let Some(piece) = board.get_player_piece(&rook_crd, self.player) {
                    if piece.is_rook() && piece.first_move() {
                        
                        let mut tmp = Crd::create(rook_crd.x(), rook_crd.y()+d);
                        let mut castling = true;
                        for _ in 1..(rook_crd.y() - crd.y()).abs() {
                            if let Some(ref c) = tmp {
                                if enemy_attacks.contains(&c) || board.is_piece(&c) {
                                    castling = false;
                                    break;
                                }
                                tmp = Crd::create(c.x(), c.y()+d);
                            }
                            
                        }
                        if castling {
                            if let Some(c) = Crd::create(rook_crd.x(), rook_crd.y()+d) {
                                moves.insert(c);
                            }
                        }

                    }
                }
            }
            
        }

        moves
    }


    pub fn change_first_move(&mut self) {
        if self.first_move {
            self.first_move = false;
        }
    }


    pub fn first_move(&self) -> bool {
        self.first_move
    }
}