
use crate::chess::crd::Crd;
use crate::chess::board::Board;

pub struct Pawn {
    player: i8,
    two_cells: bool,
    first_move: bool,
}

impl Pawn {
    pub fn create(player: i8) -> Self{
        Self{
            player,
            two_cells: false,
            first_move: true,
        }
    }

    pub fn get_player(&self) -> i8 {
        self.player
    }

    pub fn get_moves(&self, crd: &Crd, board: &Board) -> Vec<Crd> {
        let mut moves: Vec<Crd> = vec![];
        let direction = [1, -1]; //black, white
        let side = [(1, 1),(-1, 1)];



        let c = Crd::create(
            crd.x() + direction[(self.player % 2) as usize],
            crd.y());
        
        if !board.is_piece(&c) {
            moves.push(c.unwrap());
            
            if self.first_move {
                let c = Crd::create(
                    crd.x() + 2*direction[(self.player % 2) as usize], 
                    crd.y());
                if !board.is_piece(&c) {
                    moves.push(c.unwrap());
                }
            }
        }

        moves
    }
    pub fn possible_moves() -> Vec<Crd> {
        vec![]
    }

    pub fn possible_capture() -> Vec<Crd> {
        vec![]
    }
}