
use crate::chess::crd::Crd;

#[derive(Copy, Clone)]
pub struct Pawn {
    player: i8,
    two_moves: bool,
    first_move: bool,
}

impl Pawn {
    pub fn create(player: i8) -> Self{
        Self{
            player,
            two_moves: false,
            first_move: true,
        }
    }

    pub fn get_player(&self) -> i8 {
        self.player
    }

    pub fn get_moves(&self, crd: &Crd) -> Vec<Crd> {

        vec![]
    }
}