
use crate::chess::context::Context;
use crate::chess::board::Board;
use crate::chess::crd::Crd;
use crate::chess::state::State;
use crate::chess::piece::Piece;

impl Context {
    
    pub fn is_mate(&self) -> bool {
        for item in &self.get_board().get_pieces()[self.get_player()/2] {
            
            let moves = self.get_possible_moves(item);
            for piece in moves {
                
            }
            
        }

        false
    }

    pub fn is_check(&self) -> bool {
        

        false
    }

}