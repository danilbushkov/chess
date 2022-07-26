
use crate::chess::crd::Crd;
use crate::chess::board::Board;


pub struct Knight {
    player: i8,
}

impl Knight {
    pub fn create(player: i8) -> Self{
        Self{
            player,
        }
    }

    pub fn get_player(&self) -> i8 {
        self.player
    }


    pub fn get_moves(&self, crd: &Crd, board: &Board) -> Vec<Crd> {
        let mut moves: Vec<Crd> = vec![];
        let direction_1 = [1,-1];
        let direction_2 = [2,-2];
        
        for a in direction_1 {
            for b in direction_2 {
                let crd_1 = Crd::create(crd.x() + (a as i8), crd.y() + (b as i8));
                let crd_2 = Crd::create(crd.x() + (b as i8), crd.y() + (a as i8));
                let crds = [crd_1, crd_2];

                for crd in crds {
                    if !board.is_piece_or_border(&crd) || board.is_enemy_piece(&crd, self.player) {
                        moves.push(crd.unwrap());
                    } 
                }
            }
        } 


        moves
    }
}