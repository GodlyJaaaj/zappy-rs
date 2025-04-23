#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Vec2<T> {
    x: T,
    y: T,
}

pub type Position = Vec2<i64>;
/// An unsigned position in the game
pub type UPosition = Vec2<u64>;
/// A size in the game
pub type Size = Vec2<u64>;

pub trait HasPosition {
    fn position(&self) -> UPosition;

    fn position_mut(&mut self) -> &mut UPosition;
}

impl<T: Copy> Vec2<T> {
    /// Create a new Vec2
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
    /// Get the x value
    pub fn x(&self) -> T {
        self.x
    }
    /// Get the y value
    pub fn y(&self) -> T {
        self.y
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.x
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self.y
    }

    pub fn replace(&mut self, other: Vec2<T>) {
        self.x = other.x;
        self.y = other.y;
    }
}

impl Default for Vec2<u64> {
    fn default() -> Self {
        Vec2::new(0, 0)
    }
}

impl From<(u64, u64)> for Vec2<u64> {
    /// Convert a tuple into a Vec2
    fn from((x, y): (u64, u64)) -> Self {
        Vec2 { x, y }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_vec2() {
        let pos = UPosition::new(1, 2);
        assert_eq!(pos.x(), 1);
        assert_eq!(pos.y(), 2);
    }

    #[test]
    fn test_vec2_from_tuple() {
        let pos: UPosition = (1, 2).into();
        assert_eq!(pos.x(), 1);
        assert_eq!(pos.y(), 2);
    }

    #[test]
    fn test_vec2_eq() {
        let pos1 = UPosition::new(1, 2);
        let pos2 = UPosition::new(1, 2);
        assert_eq!(pos1, pos2);
    }

    #[test]
    fn test_vec2_ne() {
        let pos1 = UPosition::new(1, 2);
        let pos2 = UPosition::new(2, 1);
        assert_ne!(pos1, pos2);
    }

    #[test]
    fn test_vec2_clone() {
        let pos = UPosition::new(1, 2);
        let pos_clone = pos;
        assert_eq!(pos, pos_clone);
    }
}
