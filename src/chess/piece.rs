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
    pub fn moves(&self) -> Vec<Crd> {
        match self {
            _ => vec![],
        }
    }
}
