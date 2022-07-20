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
mod queen;
mod rook;


use crate::chess::crd::Crd;
use crate::chess::piece::bishop::Bishop;
use crate::chess::piece::king::King;
use crate::chess::piece::knight::Knight;
use crate::chess::piece::pawn::Pawn;
use crate::chess::piece::queen::Queen;
use crate::chess::piece::rook::Rook;




#[derive(Copy, Clone)]
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
    pub fn create_none() -> Self {
        Piece::Knight(Knight::create(1))
    }


    pub fn moves(&self) -> Vec<Crd> {
        match self {
            _ => vec![],
        }
    }

    pub fn get_code(&self) -> i8 {
        match self {
            Piece::None => 0,
            piece => piece.code() + piece.get_player() / 2,
        }
    }

    fn code(&self) -> i8 {
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

    pub fn get_player(&self) -> i8 {
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


}
