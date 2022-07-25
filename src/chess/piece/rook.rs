
use crate::chess::crd::Crd;
use crate::chess::board::Board;


pub struct Rook {
    player: i8,
    first_move: bool,
}

impl Rook {
    pub fn create(player: i8) -> Self{
        Self{
            player,
            first_move: true,
        }
    }


    pub fn get_player(&self) -> i8 {
        self.player
    }

    pub fn get_moves(&self, crd: &Crd, board: &Board) -> Vec<Crd> {
        let mut moves: Vec<Crd> = vec![];
        let direction = [(1, 0),(0, 1),(-1, 0),(0, -1)];

        for &(ref a, ref b) in &direction {
            let mut search = true;
            while search {
                let crd = Crd::create(crd.x() + (*a as i8), crd.y() + (*b as i8));
                
                search = !board.is_piece(&crd);
                if search || board.is_enemy_piece(&crd, self.player) {
                    moves.push(crd.unwrap());
                }
            }

        }


        moves
    }
}