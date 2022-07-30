use std::collections::HashSet;


use crate::chess::crd::Crd;
use crate::chess::board::Board;


pub struct Bishop {
    player: usize,
    

}

impl Bishop {
    pub fn create(player: usize) -> Self{
        Self{
            player,
            
        }
    }

    pub fn get_player(&self) -> usize {
        self.player
    }


    pub fn get_moves(&self, crd: &Crd, board: &Board) -> HashSet<Crd> {
        let mut moves: HashSet<Crd> = HashSet::new();
        let direction = [(1, 1),(1, -1),(-1, 1),(-1, -1)];

        for (a, b) in &direction {
            let mut search = true;
            let mut crd = crd.clone();
            while search {
                match Crd::create(crd.x() + *a, crd.y() + *b) {
                    Some(c) => {
                        if board.is_piece(&c) {
                            search = false;
                            if board.is_enemy_piece(&c, self.player) {
                                moves.insert(c);
                            }
                        } else {
                            moves.insert(c.clone());
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