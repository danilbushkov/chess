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
}