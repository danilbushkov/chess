//  white/black
// 0 - empty cell
// 1/2 - pawn
// 3/4 - rook
// 5/6 - knight
// 7/8 - bishop
// 9/10 - queen
// 11/12 - king

// player: 
// 1 - white
// 2 - black


mod bishop;
mod king;
mod knight;
mod pawn;
mod rook;
pub mod queen;
pub mod trajectory;

use std::collections::HashSet;
use crate::chess::crd::Crd;
use crate::chess::piece::bishop::Bishop;
use crate::chess::piece::king::King;
use crate::chess::piece::knight::Knight;
use crate::chess::piece::pawn::Pawn;
use crate::chess::piece::queen::Queen;
use crate::chess::piece::rook::Rook;
use crate::chess::board::Board;




pub enum Piece {
    Bishop(Bishop),
    King(King),
    Knight(Knight),
    Pawn(Pawn),
    Queen(Queen),
    Rook(Rook),
    None,
}


impl Piece {


    pub fn create(code: usize) -> Self {
        match code {
            1 => Piece::Pawn(Pawn::create(1)),
            2 => Piece::Pawn(Pawn::create(2)),
            3 => Piece::Rook(Rook::create(1)),
            4 => Piece::Rook(Rook::create(2)),
            5 => Piece::Knight(Knight::create(1)),
            6 => Piece::Knight(Knight::create(2)),
            7 => Piece::Bishop(Bishop::create(1)),
            8 => Piece::Bishop(Bishop::create(2)),
            9 => Piece::Queen(Queen::create(1)),
            10 => Piece::Queen(Queen::create(2)),
            11 => Piece::King(King::create(1)),
            12 => Piece::King(King::create(2)),
            _ => Piece::None,
        }
    }


    pub fn moves(&self, crd: &Crd, board: &Board) -> HashSet<Crd> {
        match self {
            Piece::Pawn(p) => p.get_moves(crd, board, false),
            Piece::Rook(p) => p.get_moves(crd, board, None),
            Piece::Knight(p) => p.get_moves(crd, board, false),
            Piece::Bishop(p) => p.get_moves(crd, board, None),
            Piece::Queen(p) => p.get_moves(crd, board, None),
            Piece::King(p) => p.get_moves(crd, board, false),
            Piece::None => HashSet::new(),
        }
    }

    pub fn attacks(&self, crd: &Crd, board: &Board, skip: &Crd) -> HashSet<Crd> {
        match self {
            Piece::Pawn(p) => p.get_moves(crd, board, true),
            Piece::Rook(p) => p.get_moves(crd, board, Some(skip)),
            Piece::Knight(p) => p.get_moves(crd, board, true),
            Piece::Bishop(p) => p.get_moves(crd, board, Some(skip)),
            Piece::Queen(p) => p.get_moves(crd, board, Some(skip)),
            Piece::King(p) => p.get_moves(crd, board, true),
            Piece::None => HashSet::new(),
        }
    }

    pub fn get_code(&self) -> usize {
        match self {
            Piece::None => 0,
            piece => piece.code() + piece.get_player() / 2,
        }
    }

    fn code(&self) -> usize {
        match self {
            Piece::Pawn(_) => 1,
            Piece::Rook(_) => 3,
            Piece::Knight(_) => 5,
            Piece::Bishop(_) => 7,
            Piece::Queen(_) => 9,
            Piece::King(_) => 11,
            Piece::None => 0,
        }
    }

    pub fn get_player(&self) -> usize {
        match self {
            Piece::Pawn(p) => p.get_player(),
            Piece::Rook(p) => p.get_player(),
            Piece::Knight(p) => p.get_player(),
            Piece::Bishop(p) => p.get_player(),
            Piece::Queen(p) => p.get_player(),
            Piece::King(p) => p.get_player(),
            Piece::None => 0,
        }
    }


    pub fn is_en_passant(&self) -> bool {
        match self {
            Piece::Pawn(p) => p.two_cells(), 
            _ => false,
        }
    }

    pub fn is_pawn(&self) -> bool {
        match self {
            Piece::Pawn(_) => true,
            _ => false,
        }
    }
    pub fn is_king(&self) -> bool {
        match self {
            Piece::King(_) => true,
            _ => false,
        }
    }
    pub fn is_rook(&self) -> bool {
        match self {
            Piece::Rook(_) => true,
            _ => false,
        }
    }
    pub fn is_queen(&self) -> bool {
        match self {
            Piece::Queen(_) => true,
            _ => false,
        }
    }

    pub fn is_queen_or_bishop_or_rook(&self) -> bool {
        match self {
            Piece::Queen(_) => true,
            Piece::Bishop(_) => true,
            Piece::Rook(_) => true,
            _ => false,
        }
    }


    pub fn first_move(&self) -> bool {
        match self {
            Piece::Pawn(p) => p.first_move(),
            Piece::King(p) => p.first_move(),
            Piece::Rook(p) => p.first_move(),
            _ => false,
        }
    }


    pub fn change_first_move(&mut self) {
        match self {
            Piece::Pawn(p) => p.change_first_move(),
            Piece::King(p) => p.change_first_move(),
            Piece::Rook(p) => p.change_first_move(),
            _ => (),
        }
    }

    pub fn change_two_calls(&mut self, location: &Crd, target: &Crd) {
        match self {
            Piece::Pawn(p) => p.change_two_calls(location, target),
            _ => (),
        }
    }

}
