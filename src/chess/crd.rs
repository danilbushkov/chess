#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Crd {
    x:isize,
    y:isize,
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

    pub fn create_u((x, y): (usize, usize)) -> Option<Self> { 
        if x>7 {
            return None;
        }
        if y>7 {
            return None;
        }
        Some(Self {
            x: x as isize,
            y: y as isize,
        })
    }

}