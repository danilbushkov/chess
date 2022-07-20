#[derive(Copy, Clone)]
pub struct King {
    player: i8,
    first_move: bool,
}

impl King {
    pub fn create(player: i8) -> Self{
        Self{
            player,
            first_move: true,
        }
    }

    pub fn get_player(&self) -> i8 {
        self.player
    }
}