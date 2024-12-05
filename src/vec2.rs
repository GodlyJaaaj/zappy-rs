#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Vec2<T> {
    x: T,
    y: T,
}

pub type Position = Vec2<u64>;
pub type Size = Vec2<u64>;

impl Vec2<u64> {
    pub fn new(x: u64, y: u64) -> Self {
        Vec2 { x, y }
    }
    pub fn x(&self) -> u64 {
        self.x
    }
    pub fn y(&self) -> u64 {
        self.y
    }
}

