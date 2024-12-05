use crate::vec2::Size;

struct Map {
    size: Size,
}

impl Map {
    pub fn new(size: Size) -> Self {
        Map { size }
    }

    pub fn size(&self) -> Size {
        self.size
    }
}
