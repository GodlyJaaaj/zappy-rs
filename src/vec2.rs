#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Vec2<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

/// A position in the game
pub type Position = Vec2<u64>;
/// A size in the game
pub type Size = Vec2<u64>;

impl Vec2<u64> {
    /// Create a new Vec2
    pub fn new(x: u64, y: u64) -> Self {
        Vec2 { x, y }
    }
    /// Get the x value
    pub fn x(&self) -> u64 {
        self.x
    }
    /// Get the y value
    pub fn y(&self) -> u64 {
        self.y
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
        let pos = Position::new(1, 2);
        assert_eq!(pos.x(), 1);
        assert_eq!(pos.y(), 2);
    }

    #[test]
    fn test_vec2_from_tuple() {
        let pos: Position = (1, 2).into();
        assert_eq!(pos.x(), 1);
        assert_eq!(pos.y(), 2);
    }

    #[test]
    fn test_vec2_eq() {
        let pos1 = Position::new(1, 2);
        let pos2 = Position::new(1, 2);
        assert_eq!(pos1, pos2);
    }

    #[test]
    fn test_vec2_ne() {
        let pos1 = Position::new(1, 2);
        let pos2 = Position::new(2, 1);
        assert_ne!(pos1, pos2);
    }

    #[test]
    fn test_vec2_clone() {
        let pos = Position::new(1, 2);
        let pos_clone = pos;
        assert_eq!(pos, pos_clone);
    }
}
