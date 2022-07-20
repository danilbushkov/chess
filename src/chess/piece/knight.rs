#[derive(Copy, Clone)]
pub struct Knight {
    player: i8,
}

impl Knight {
    pub fn create(player: i8) -> Self{
        Self{
            player,
        }
    }

    pub fn get_player(&self) -> i8 {
        self.player
    }
}