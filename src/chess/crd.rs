
pub struct Crd {
    x:i8,
    y:i8,
}

impl Crd {
    pub fn new(x: i8, y: i8) -> Option<Self> { 
        if (x<0) && (x>7) {
            return None;
        }
        if (y<0) && (y>7) {
            return None;
        }
        Some(Self {
            x,
            y,
        })
    }
}