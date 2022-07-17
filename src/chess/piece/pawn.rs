pub struct Pawn {
    player: i8,
    two_moves: bool,
    first_move: bool,
}

impl Pawn {
    pub fn create(player: i8) -> Self{
        Self{
            player,
            two_moves: false,
            first_move: true,
        }
    }
}