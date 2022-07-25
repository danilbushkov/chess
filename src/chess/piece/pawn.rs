
use crate::chess::crd::Crd;
use crate::chess::board::Board;

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

    pub fn get_moves(&self, crd: &Crd, board: &Board) -> Vec<Crd> {
        let mut moves = vec![];
        let direction = [1, -1]; //black, white

        let crd = Crd::create(
            crd.x() + direction[(self.player % 2) as usize],
            crd.y());
        
        match crd {
            Some(c) => {
                
            }
            _=>(),
        }
        

        // if self.first_move {
        //     let crd = Crd::create(
        //         crd.x() + 2*direction[(self.player % 2) as usize], 
        //         crd.y());
        //     if Board::check_borders(&crd) {
                
        //     }
        // }


        moves
    }
}