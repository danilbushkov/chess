
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
        
        moves.append(&mut self.possible_moves(crd, board));
        moves.append(&mut self.possible_capture(crd, board));
        moves.append(&mut self.en_passant(crd, board));

        moves
    }

    pub fn possible_moves(&self, crd: &Crd, board: &Board) -> Vec<Crd> {
        let mut moves: Vec<Crd> = vec![];
        let direction = [1, -1]; //black, white



        let c = Crd::create(
            crd.x() + direction[(self.player % 2) as usize],
            crd.y());
        
        if !board.is_piece_or_border(&c) {
            moves.push(c.unwrap());
            
            if self.first_move {
                let c = Crd::create(
                    crd.x() + 2*direction[(self.player % 2) as usize], 
                    crd.y());
                if !board.is_piece_or_border(&c) {
                    moves.push(c.unwrap());
                }
            }
        }

        moves
        
    }

    pub fn possible_capture(&self, crd: &Crd, board: &Board) -> Vec<Crd> {
        let mut moves: Vec<Crd> = vec![];
        let direction = [1, -1]; //black, white
        
        for b in direction {
            let c = Crd::create(
                crd.x() + direction[(self.player % 2) as usize],
                crd.y() + b);

            if board.is_enemy_piece(&c, self.player) {
                moves.push(c.unwrap());
            }

        }

        moves 
    }

    pub fn en_passant(&self, crd: &Crd, board: &Board) -> Vec<Crd> {
        let mut moves: Vec<Crd> = vec![];
        let direction = [1, -1]; //black, white
        for b in direction {
            let crd_1 = Crd::create(
                crd.x(),
                crd.y() + b, 
            );
            
            let enemy_piece = board.get_enemy_piece(&crd_1, self.player);
            if let Some(p) = enemy_piece {
                if p.is_en_passant() {
                    let crd_2 = Crd::create(
                        crd.x() + direction[(self.player % 2) as usize], 
                        crd.y() + b);
                    if !board.is_piece_or_border(&crd_2) {
                        moves.push(crd_2.unwrap());
                    }
                    
                }
            }

        }

        moves 
    }




    pub fn two_cells(&self) -> bool {
        self.two_cells
    }
}