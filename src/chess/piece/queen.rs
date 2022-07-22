
pub struct Queen {
    player: i8,
}

impl Queen {
    pub fn create(player: i8) -> Self{
        Self{
            player,
        }
    }

    pub fn get_player(&self) -> i8 {
        self.player
    }
}