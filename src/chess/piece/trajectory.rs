use std::collections::HashSet;
pub use crate::chess::piece::Piece;
use crate::chess::crd::Crd;
use crate::chess::board::Board;


impl Piece {
    pub fn get_trajectory(moves: &mut HashSet<Crd>,
                        crd: &Crd, 
                        board: &Board, 
                        (a, b): &(isize, isize),
                        player: usize, 
                        cover: bool) {
        let mut search = true;
            let mut crd = crd.clone();
            while search {
                match Crd::create(crd.x() + *a, crd.y() + *b) {
                    Some(c) => {
                        if board.is_piece(&c) {
                            search = false;
                            if board.is_enemy_piece(&c, player) || cover {
                                moves.insert(c);
                            }
                        } else {
                            moves.insert(c.clone());
                            crd = c;
                        }
                    },
                    None => {
                        search = false;
                    },
                }
            }
    }
}