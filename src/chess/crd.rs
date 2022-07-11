use std::i8;

pub struct Crd {
    x:i8,
    y:i8,
}

impl Crd {
    pub fn new(x: i32, y: i32) -> Option<Self> { 
        if (x<0) && (x>7) {
            return None;
        }
        if (y<0) && (y>7) {
            return None;
        }
        Some(Self {
            x: x as i8,
            y: y as i8,
        })
    }
}