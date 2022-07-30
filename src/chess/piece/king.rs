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


    pub fn get_moves(&self, crd: &Crd, board: &Board) -> HashSet<Crd> {
        HashSet::new()
    }

    pub fn change_first_move(&mut self) {
        if self.first_move {
            self.first_move = false;
        }
    }
}