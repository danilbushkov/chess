use std::collections::HashSet;


use crate::chess::crd::Crd;
use crate::chess::board::Board;
use crate::chess::piece::Piece;



pub struct Rook {
    player: usize,
    first_move: bool,
}

impl Rook {
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
        let mut moves: HashSet<Crd> = HashSet::new();
        let direction = [(1, 0),(0, 1),(-1, 0),(0, -1)];

        for d in &direction {
            Piece::get_trajectory(&mut moves, crd, board, d, self.player, cover);

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