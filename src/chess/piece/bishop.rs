pub struct Bishop {
    player: i8,

}

impl Bishop {
    pub fn create(player: i8) -> Self{
        Self{
            player,
        }
    }
}