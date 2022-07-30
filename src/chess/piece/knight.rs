use std::collections::HashSet;


use crate::chess::crd::Crd;
use crate::chess::board::Board;


pub struct Knight {
    player: usize,
    
}

impl Knight {
    pub fn create(player: usize) -> Self{
        Self{
            player,
            
        }
    }

    pub fn get_player(&self) -> usize {
        self.player
    }


    pub fn get_moves(&self, crd: &Crd, board: &Board, cover: bool) -> HashSet<Crd> {
        let mut moves: HashSet<Crd> = HashSet::new();
        let direction_1 = [1,-1];
        let direction_2 = [2,-2];
        
        for a in direction_1 {
            for b in direction_2 {
                let crd_1 = Crd::create(crd.x() + a, crd.y() + b);
                let crd_2 = Crd::create(crd.x() + b, crd.y() + a);
                let crds = [crd_1, crd_2];

                for crd in crds {
                    if let Some(c) = crd {
                        if !board.is_piece(&c) || 
                            board.is_enemy_piece(&c, self.player) || 
                            cover {
                            moves.insert(c);
                        }
                    }
                    // if !board.is_piece_or_border(&c) || board.is_enemy_piece(&c, self.player) {
                    //     moves.insert(c.unwrap().get_tuple());
                    // } 
                }
            }
        } 



        moves
    }
}