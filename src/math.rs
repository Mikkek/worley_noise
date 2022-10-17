#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

/// I don't know how much this is needed, but it can't hurt, i think
impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    /// Bandaid. This could be done better, but right now i don't care too mutch.
    pub fn floor(&self) -> [i32; 2] {
        [
        self.x.floor() as i32,
        self.y.floor() as i32,
        ]
    }

    pub fn frac(&self) -> Self{
        Vec2 {
            x: self.x - self.x.floor(),
            y: self.y - self.y.floor(),
        }
    }
}
