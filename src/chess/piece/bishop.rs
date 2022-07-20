#[derive(Copy, Clone)]
pub struct Bishop {
    player: i8,

}

impl Bishop {
    pub fn create(player: i8) -> Self{
        Self{
            player,
        }
    }

    pub fn get_player(&self) -> i8 {
        self.player
    }
}