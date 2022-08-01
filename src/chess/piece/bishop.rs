use std::collections::HashSet;


use crate::chess::crd::Crd;
use crate::chess::board::Board;
use crate::chess::piece::Piece;

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


    pub fn get_moves(&self, crd: &Crd, board: &Board, skip: Option<&Crd>) -> HashSet<Crd> {
        let mut moves: HashSet<Crd> = HashSet::new();
        let direction = [(1, 1),(1, -1),(-1, 1),(-1, -1)];

        for d in &direction {
            Piece::get_trajectory(&mut moves, crd, board, d, self.player, skip);

        }
        moves

    }
}