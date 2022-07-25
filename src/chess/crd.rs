
pub struct Crd {
    x:i8,
    y:i8,
}

impl Crd {
    pub fn create(x: i8, y: i8) -> Option<Self> { 
        if (x<0) || (x>7) {
            return None;
        }
        if (y<0) || (y>7) {
            return None;
        }
        Some(Self {
            x,
            y,
        })
    }
    pub fn default() -> Option<Self> {
        None
    }

    pub fn x(&self) -> i8 {
        self.x as i8
    }

    pub fn y(&self) -> i8 {
        self.y as i8
    }
}