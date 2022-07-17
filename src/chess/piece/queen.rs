#[derive(Copy, Clone)]
pub struct Queen {
    player: i8,
}

impl Queen {
    pub fn create(player: i8) -> Self{
        Self{
            player,
        }
    }
}