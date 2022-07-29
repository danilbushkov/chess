
pub struct Crd {
    x:isize,
    y:isize,
}


impl PartialEq for Crd {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && 
        self.y == other.y
    }
}

impl Clone for Crd {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}


impl Crd {
    pub fn create(x: isize, y: isize) -> Option<Self> { 
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

    pub fn x(&self) -> isize {
        self.x 
    }

    pub fn y(&self) -> isize {
        self.y 
    }

    pub fn get_tuple(&self) -> (usize, usize) {
        (self.x as usize , self.y as usize )
    }

    

}