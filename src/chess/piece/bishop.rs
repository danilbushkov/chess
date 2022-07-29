use std::collections::HashSet;


use crate::chess::crd::Crd;
use crate::chess::board::Board;


pub struct Bishop {
    player: i8,
    

}

impl Bishop {
    pub fn create(player: i8) -> Self{
        Self{
            player,
            
        }
    }

    pub fn get_player(&self) -> i8 {
        self.player
    }


    pub fn get_moves(&self, crd: &Crd, board: &Board) -> HashSet<(usize, usize)> {
        let mut moves: HashSet<(usize, usize)> = HashSet::new();
        let direction = [(1, 1),(1, -1),(-1, 1),(-1, -1)];

        for &(ref a, ref b) in &direction {
            let mut search = true;
            let mut crd = crd.clone();
            while search {
                match Crd::create(crd.x() + (*a as i8), crd.y() + (*b as i8)) {
                    Some(c) => {
                        if board.is_piece_by_crd(&c) {
                            search = false;
                            if board.is_enemy_piece_by_crd(&c, self.player) {
                                moves.insert(c.get_tuple());
                            }
                        } else {
                            moves.insert(c.get_tuple());
                            crd = c;
                        }
                    },
                    None => {
                        search = false;
                    },
                }
            }

        }
        moves

    }
}