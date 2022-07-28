use std::collections::HashSet;


use crate::chess::crd::Crd;
use crate::chess::board::Board;


pub struct Queen {
    player: i8,
}

impl Queen {
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
        let direction = [(1, 0),(0, 1),(-1, 0),(0, -1),(1, 1),(1, -1),(-1, 1),(-1, -1)];

        for &(ref a, ref b) in &direction {
            let mut search = true;
            while search {
                let crd = Crd::create(crd.x() + (*a as i8), crd.y() + (*b as i8));
                
                search = !board.is_piece_or_border(&crd);
                if search || board.is_enemy_piece(&crd, self.player) {
                    moves.insert(crd.unwrap().get_tuple());
                }
            }

        }


        moves
    }
}