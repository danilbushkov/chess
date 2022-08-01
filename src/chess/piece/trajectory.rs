use std::collections::HashSet;
pub use crate::chess::piece::Piece;
use crate::chess::crd::Crd;
use crate::chess::board::Board;


impl Piece {
    pub fn get_trajectory(moves: &mut HashSet<Crd>,
                        crd: &Crd, 
                        board: &Board, 
                        d: &(isize, isize),
                        player: usize, 
                        skip: Option<&Crd>,
                        ) {
        match skip {
            Some(c) => Self::skip(moves, crd, board, d, player, &c),
            None => Self::get_moves(moves, crd, board, d, player),
        }


        
    }

    fn skip(moves: &mut HashSet<Crd>,
        crd: &Crd, 
        board: &Board, 
        (a, b): &(isize, isize),
        player: usize, 
        skip: &Crd) {
            let mut search = true;
            let mut crd = crd.clone();
            while search {
                match Crd::create(crd.x() + *a, crd.y() + *b) {
                    Some(c) => {
                        if board.is_piece(&c) && c != *skip {
                            search = false;
                            if board.is_enemy_piece(&c, player) {
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

    fn get_moves(moves: &mut HashSet<Crd>,
            crd: &Crd, 
            board: &Board, 
            (a, b): &(isize, isize),
            player: usize) {
            let mut search = true;
            let mut crd = crd.clone();
            while search {
                match Crd::create(crd.x() + *a, crd.y() + *b) {
                    Some(c) => {
                        if board.is_piece(&c) {
                            search = false;
                            if board.is_enemy_piece(&c, player) {
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