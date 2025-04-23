use crate::player::{Direction, Player};
use crate::vec2::Size;
use crate::vec2::{HasPosition, UPosition};
use std::f64::consts::PI;

pub struct Emitter {
    pos: UPosition,
}

impl From<&Player> for Emitter {
    fn from(player: &Player) -> Self {
        Emitter {
            pos: player.position(),
        }
    }
}

pub struct Receiver {
    pos: UPosition,
    direction: Direction,
}

impl From<&Player> for Receiver {
    fn from(player: &Player) -> Self {
        Receiver {
            pos: player.position(),
            direction: player.direction(),
        }
    }
}

/// Returns the shortest path between two points on a torus.
/// * `start` - The starting position.
/// * `end` - The ending position.
/// * `size` - The size of the torus.
/// # Examples
/// ```
/// use crate::vec2::Position;
/// use crate::vec2::Size;
/// use crate::sound::get_shortest_path_torique;
/// let map_size = Size::new(10, 8);
/// let start = Position::new(0, 6);
/// let end = Position::new(9, 3);
/// let (dx, dy) = get_shortest_path_torique(start, end, map_size);
/// assert_eq!(dx, -1);
/// assert_eq!(dy, -3);
/// ```
/// * `return` - A tuple containing the shortest path in the x and y directions starting from the start position.
fn get_shortest_path_torique(start: UPosition, end: UPosition, size: Size) -> (i64, i64) {
    let (dx, dy) = (
        (end.x() as i64 - start.x() as i64).rem_euclid(size.x() as i64),
        (end.y() as i64 - start.y() as i64).rem_euclid(size.y() as i64),
    );

    let dx = if dx > size.x() as i64 / 2 {
        dx - size.x() as i64
    } else {
        dx
    };
    let dy = if dy > size.y() as i64 / 2 {
        dy - size.y() as i64
    } else {
        dy
    };

    (dx, dy)
}

pub fn get_sound_direction(emitter: Emitter, receiver: Receiver, size: Size) -> u8 {
    if emitter.pos == receiver.pos {
        return 0;
    }
    let (dx, dy) = get_shortest_path_torique(receiver.pos, emitter.pos, size);
    let mut global_angle = (dy as f64).atan2(dx as f64);
    if global_angle < 0.0 {
        global_angle += 2.0 * PI;
    }
    let dir = (global_angle / (PI / 4.0)).round_ties_even() as i64;
    ((dir
        + match receiver.direction {
            Direction::East => 0,
            Direction::North => 6,
            Direction::South => 2,
            Direction::West => 4,
        })
    .rem_euclid(8)
        + 1) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2::Size;
    use crate::vec2::UPosition;

    #[test]
    fn test_sound_same_position() {
        let map_size = Size::new(10, 10);

        let emitter = Emitter {
            pos: UPosition::new(5, 5),
        };
        let receiver = Receiver {
            pos: UPosition::new(5, 5),
            direction: Direction::North,
        };

        let direction = get_sound_direction(emitter, receiver, map_size);

        assert_eq!(direction, 0);
    }

    #[test]
    fn test_sound_direction_edges() {
        let map_size = Size::new(21, 21);

        let emitter = Emitter {
            pos: UPosition::new(20, 20),
        };
        let receiver = Receiver {
            pos: UPosition::new(0, 0),
            direction: Direction::South,
        };

        let direction = get_sound_direction(emitter, receiver, map_size);

        assert_eq!(direction, 8);
    }

    #[test]
    fn test_sound_direction_pp() {
        let map_size = Size::new(8, 10);

        let emitter = Emitter {
            pos: UPosition::new(5, 6),
        };
        let receiver = Receiver {
            pos: UPosition::new(3, 2),
            direction: Direction::North,
        };

        let direction = get_sound_direction(emitter, receiver, map_size);

        assert_eq!(direction, 8);
    }

    #[test]
    fn test_sound_direction_pp2() {
        let map_size = Size::new(8, 10);

        let emitter = Emitter {
            pos: UPosition::new(5, 6),
        };
        let receiver = Receiver {
            pos: UPosition::new(0, 5),
            direction: Direction::South,
        };

        let direction = get_sound_direction(emitter, receiver, map_size);

        assert_eq!(direction, 7);
    }

    #[test]
    fn test_sound_direction_pp3() {
        let map_size = Size::new(8, 10);

        let emitter = Emitter {
            pos: UPosition::new(5, 6),
        };
        let receiver = Receiver {
            pos: UPosition::new(6, 6),
            direction: Direction::East,
        };

        let direction = get_sound_direction(emitter, receiver, map_size);

        assert_eq!(direction, 5);
    }

    #[test]
    fn test_sound_direction_pp4() {
        let map_size = Size::new(8, 10);

        let emitter = Emitter {
            pos: UPosition::new(5, 6),
        };
        let receiver = Receiver {
            pos: UPosition::new(0, 7),
            direction: Direction::West,
        };

        let direction = get_sound_direction(emitter, receiver, map_size);

        assert_eq!(direction, 1);
    }

    #[test]
    fn test_sound_direction_pp5() {
        let map_size = Size::new(8, 10);

        let emitter = Emitter {
            pos: UPosition::new(7, 1),
        };
        let receiver = Receiver {
            pos: UPosition::new(6, 0),
            direction: Direction::North,
        };

        let direction = get_sound_direction(emitter, receiver, map_size);

        assert_eq!(direction, 8);
    }

    #[test]
    fn test_sound_direction_pp6() {
        let map_size = Size::new(8, 10);

        let emitter = Emitter {
            pos: UPosition::new(2, 4),
        };
        let receiver = Receiver {
            pos: UPosition::new(0, 9),
            direction: Direction::North,
        };

        let direction = get_sound_direction(emitter, receiver, map_size);

        assert_eq!(direction, 1);
    }

    #[test]
    fn test_sound_direction_no_edges() {
        let map_size = Size::new(50, 50);

        let emitter = Emitter {
            pos: UPosition::new(20, 20),
        };
        let receiver = Receiver {
            pos: UPosition::new(0, 0),
            direction: Direction::South,
        };

        let direction = get_sound_direction(emitter, receiver, map_size);

        assert_eq!(direction, 4);
    }

    #[test]
    fn test_shortest_path() {
        let map_size = Size::new(10, 8);

        let emitter = Emitter {
            pos: UPosition::new(0, 6),
        };
        let receiver = Receiver {
            pos: UPosition::new(9, 3),
            direction: Direction::North,
        };

        let (dx, dy) = get_shortest_path_torique(emitter.pos, receiver.pos, map_size);

        assert_eq!(dx, -1);
        assert_eq!(dy, -3);
    }

    #[test]
    fn test_shortest_path_wraparound() {
        let map_size = Size::new(10, 8);

        let emitter = Emitter {
            pos: UPosition::new(9, 3),
        };
        let receiver = Receiver {
            pos: UPosition::new(0, 6),
            direction: Direction::North,
        };

        let (dx, dy) = get_shortest_path_torique(emitter.pos, receiver.pos, map_size);

        assert_eq!(dx, 1);
        assert_eq!(dy, 3);
    }

    #[test]
    fn test_shortest_path_wraparound_negative() {
        let map_size = Size::new(10, 8);

        let emitter = Emitter {
            pos: UPosition::new(0, 3),
        };
        let receiver = Receiver {
            pos: UPosition::new(9, 6),
            direction: Direction::North,
        };

        let (dx, dy) = get_shortest_path_torique(emitter.pos, receiver.pos, map_size);

        assert_eq!(dx, -1);
        assert_eq!(dy, 3);
    }

    #[test]
    fn test_shortest_path_wraparound_negative_y() {
        let map_size = Size::new(10, 8);

        let emitter = Emitter {
            pos: UPosition::new(5, 0),
        };
        let receiver = Receiver {
            pos: UPosition::new(5, 7),
            direction: Direction::North,
        };

        let (dx, dy) = get_shortest_path_torique(emitter.pos, receiver.pos, map_size);

        assert_eq!(dx, 0);
        assert_eq!(dy, -1);
    }

    #[test]
    fn test_shortest_path_wraparound_negative_x() {
        let map_size = Size::new(10, 8);

        let emitter = Emitter {
            pos: UPosition::new(0, 3),
        };
        let receiver = Receiver {
            pos: UPosition::new(9, 3),
            direction: Direction::North,
        };

        let (dx, dy) = get_shortest_path_torique(emitter.pos, receiver.pos, map_size);

        assert_eq!(dx, -1);
        assert_eq!(dy, 0);
    }

    #[test]
    fn test_shortest_path_wraparound_negative_x_y() {
        let map_size = Size::new(10, 8);

        let emitter = Emitter {
            pos: UPosition::new(0, 0),
        };
        let receiver = Receiver {
            pos: UPosition::new(9, 7),
            direction: Direction::North,
        };

        let (dx, dy) = get_shortest_path_torique(emitter.pos, receiver.pos, map_size);

        assert_eq!(dx, -1);
        assert_eq!(dy, -1);
    }

    #[test]
    fn test_shortest_path_wraparound_negative_x_y_2() {
        let map_size = Size::new(10, 8);

        let emitter = Emitter {
            pos: UPosition::new(9, 7),
        };
        let receiver = Receiver {
            pos: UPosition::new(0, 0),
            direction: Direction::North,
        };

        let (dx, dy) = get_shortest_path_torique(emitter.pos, receiver.pos, map_size);

        assert_eq!(dx, 1);
        assert_eq!(dy, 1);
    }
}
