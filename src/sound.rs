use crate::player::Direction;
use crate::protocol::Action;
use crate::vec2::Position;
use crate::vec2::Size;
use std::cmp::min;
use std::f64::consts::PI;

struct Emitter {
    pos: Position,
}

struct Receiver {
    pos: Position,
    direction: Direction,
}

fn get_shortest_path_torique(start: Position, end: Position, size: Size) -> (i64, i64) {
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

fn get_sound_direction(emitter: Emitter, receiver: Receiver, size: Size) -> Direction {
    let (dx, dy) = get_shortest_path_torique(receiver.pos, emitter.pos, size);

    let global_angle = (dy as f64).atan2(dx as f64);
    let mut relative_angle = global_angle - receiver.direction.to_radians();

    relative_angle = (relative_angle + 2.0 * PI) % (2.0 * PI);
    const DIRECTIONS: [Direction; 8] = [
        Direction::East,
        Direction::NorthEast,
        Direction::North,
        Direction::NorthWest,
        Direction::West,
        Direction::SouthWest,
        Direction::South,
        Direction::SouthEast,
    ];
    let sector_size = 2.0 * PI / 8.0;
    let sector = ((relative_angle + sector_size / 2.0) / sector_size).floor() as usize % 8;

    println!("Direction: {:?}", DIRECTIONS[sector]);
    Direction::East
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2::Position;
    use crate::vec2::Size;

    #[test]
    fn test_sound_direction() {
        let map_size = Size::new(10, 8);

        let emitter = Emitter {
            pos: Position::new(0, 0),
        };
        let receiver = Receiver {
            pos: Position::new(1, 0),
            direction: Direction::East,
        };

        let direction = get_sound_direction(emitter, receiver, map_size);

        assert_eq!(direction, Direction::East);
    }

    #[test]
    fn test_shortest_path() {
        let map_size = Size::new(10, 8);

        let emitter = Emitter {
            pos: Position::new(0, 6),
        };
        let receiver = Receiver {
            pos: Position::new(9, 3),
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
            pos: Position::new(9, 3),
        };
        let receiver = Receiver {
            pos: Position::new(0, 6),
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
            pos: Position::new(0, 3),
        };
        let receiver = Receiver {
            pos: Position::new(9, 6),
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
            pos: Position::new(5, 0),
        };
        let receiver = Receiver {
            pos: Position::new(5, 7),
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
            pos: Position::new(0, 3),
        };
        let receiver = Receiver {
            pos: Position::new(9, 3),
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
            pos: Position::new(0, 0),
        };
        let receiver = Receiver {
            pos: Position::new(9, 7),
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
            pos: Position::new(9, 7),
        };
        let receiver = Receiver {
            pos: Position::new(0, 0),
            direction: Direction::North,
        };

        let (dx, dy) = get_shortest_path_torique(emitter.pos, receiver.pos, map_size);

        assert_eq!(dx, 1);
        assert_eq!(dy, 1);
    }
}
