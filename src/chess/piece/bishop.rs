
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


    pub fn get_moves(&self, crd: &Crd, board: &Board) -> Vec<Crd> {
        vec![]
    }
}