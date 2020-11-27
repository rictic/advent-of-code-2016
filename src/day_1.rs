#![allow(dead_code)]
use Orientation::*;

#[derive(Clone, Copy, Debug)]
enum Turn {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
struct Move {
    turn: Turn,
    distance: i64,
}
impl Move {
    fn parse(input: &[u8]) -> (&[u8], Move) {
        let turn = match input[0] {
            b'L' => Turn::Left,
            b'R' => Turn::Right,
            c => panic!("Expected L or R, got {}", c),
        };
        let mut i = 1;
        loop {
            match input.get(i) {
                None => break,
                Some(c) => match c {
                    b'0'..=b'9' => {
                        i += 1;
                    }
                    _ => break,
                },
            }
        }
        let digits = &input[1..i];
        let distance = std::str::from_utf8(digits).unwrap().parse().unwrap();
        (&input[i..], Move { turn, distance })
    }
}

#[derive(Clone, Copy, Debug)]
enum Orientation {
    North,
    South,
    East,
    West,
}
impl Orientation {
    fn rotate(self, turn: Turn) -> Orientation {
        match (turn, self) {
            (Turn::Left, Orientation::North) => West,
            (Turn::Left, Orientation::South) => East,
            (Turn::Left, Orientation::East) => North,
            (Turn::Left, Orientation::West) => South,
            (Turn::Right, Orientation::North) => East,
            (Turn::Right, Orientation::South) => West,
            (Turn::Right, Orientation::East) => South,
            (Turn::Right, Orientation::West) => North,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Location {
    orientation: Orientation,
    x: i64,
    y: i64,
}
impl Location {
    fn new() -> Self {
        Location {
            orientation: North,
            x: 0,
            y: 0,
        }
    }

    fn make_move(self, movement: Move) -> Self {
        let orientation = self.orientation.rotate(movement.turn);
        let mut x = self.x;
        let mut y = self.y;
        match orientation {
            North => {
                y += movement.distance;
            }
            South => {
                y -= movement.distance;
            }
            East => {
                x += movement.distance;
            }
            West => {
                x -= movement.distance;
            }
        }
        Self { orientation, x, y }
    }

    fn distance_from_origin(self) -> u64 {
        self.x.abs() as u64 + self.y.abs() as u64
    }
}

struct MoveReader<'a> {
    route: &'a [u8],
}
impl<'a> Iterator for MoveReader<'a> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        if self.route.len() == 0 {
            return None;
        }
        let (next_route, movement) = Move::parse(self.route);
        self.route = next_route;
        if self.route.len() != 0 {
            if b", " != &self.route[0..2] {
                panic!("Expected `, ` but got {:?}", &self.route[0..2]);
            }
            self.route = &self.route[2..];
        }
        println!("{:?}", movement);
        Some(movement)
    }
}

fn location_at_end_of_route(route: &[u8]) -> Location {
    let mut location = Location::new();
    let reader = MoveReader { route };
    for movement in reader {
        location = location.make_move(movement);
        println!("{:?}", location);
    }
    location
}

fn first_duplicated_location(route: &[u8]) -> Option<Location> {
    let mut seen = std::collections::BTreeSet::new();
    seen.insert((0, 0));
    let mut location = Location::new();
    let reader = MoveReader { route };
    for movement in reader {
        let orientation = location.orientation.rotate(movement.turn);
        let mut x = location.x;
        let mut y = location.y;
        for _ in 0..movement.distance {
            match orientation {
                North => {
                    y += 1;
                }
                South => {
                    y -= 1;
                }
                East => {
                    x += 1;
                }
                West => {
                    x -= 1;
                }
            }
            if !seen.insert((x, y)) {
                return Some(Location { orientation, x, y });
            }
        }
        location = Location { orientation, x, y };
    }
    None
}

fn main() {
    println!("{:?}", location_at_end_of_route(b"R2, L3"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_example() {
        let location = location_at_end_of_route(b"R2, L3");
        assert_eq!((2, 3), (location.x, location.y));
        assert_eq!(5, location.distance_from_origin());
    }

    #[test]
    fn second_example() {
        let location = location_at_end_of_route(b"R2, R2, R2");
        assert_eq!((0, -2), (location.x, location.y));
        assert_eq!(2, location.distance_from_origin());
    }

    #[test]
    fn third_example() {
        let location = location_at_end_of_route(b"R5, L5, R5, R3");
        assert_eq!(12, location.distance_from_origin());
    }

    #[test]
    fn my_input() {
        let location = location_at_end_of_route(b"L4, L3, R1, L4, R2, R2, L1, L2, R1, R1, L3, R5, L2, R5, L4, L3, R2, R2, L5, L1, R4, L1, R3, L3, R5, R2, L5, R2, R1, R1, L5, R1, L3, L2, L5, R4, R4, L2, L1, L1, R1, R1, L185, R4, L1, L1, R5, R1, L1, L3, L2, L1, R2, R2, R2, L1, L1, R4, R5, R53, L1, R1, R78, R3, R4, L1, R5, L1, L4, R3, R3, L3, L3, R191, R4, R1, L4, L1, R3, L1, L2, R3, R2, R4, R5, R5, L3, L5, R2, R3, L1, L1, L3, R1, R4, R1, R3, R4, R4, R4, R5, R2, L5, R1, R2, R5, L3, L4, R1, L5, R1, L4, L3, R5, R5, L3, L4, L4, R2, R2, L5, R3, R1, R2, R5, L5, L3, R4, L5, R5, L3, R1, L1, R4, R4, L3, R2, R5, R1, R2, L1, R4, R1, L3, L3, L5, R2, R5, L1, L4, R3, R3, L3, R2, L5, R1, R3, L3, R2, L1, R4, R3, L4, R5, L2, L2, R5, R1, R2, L4, L4, L5, R3, L4");
        assert_eq!(332, location.distance_from_origin());
    }

    #[test]
    fn part_two_example_one() {
        let location = first_duplicated_location(b"R8, R4, R4, R8");
        assert_eq!(
            Some((4, 0)),
            location.map(|location| (location.x, location.y))
        );
    }

    #[test]
    fn part_two_my_input() {
        let location = first_duplicated_location(b"L4, L3, R1, L4, R2, R2, L1, L2, R1, R1, L3, R5, L2, R5, L4, L3, R2, R2, L5, L1, R4, L1, R3, L3, R5, R2, L5, R2, R1, R1, L5, R1, L3, L2, L5, R4, R4, L2, L1, L1, R1, R1, L185, R4, L1, L1, R5, R1, L1, L3, L2, L1, R2, R2, R2, L1, L1, R4, R5, R53, L1, R1, R78, R3, R4, L1, R5, L1, L4, R3, R3, L3, L3, R191, R4, R1, L4, L1, R3, L1, L2, R3, R2, R4, R5, R5, L3, L5, R2, R3, L1, L1, L3, R1, R4, R1, R3, R4, R4, R4, R5, R2, L5, R1, R2, R5, L3, L4, R1, L5, R1, L4, L3, R5, R5, L3, L4, L4, R2, R2, L5, R3, R1, R2, R5, L5, L3, R4, L5, R5, L3, R1, L1, R4, R4, L3, R2, R5, R1, R2, L1, R4, R1, L3, L3, L5, R2, R5, L1, L4, R3, R3, L3, R2, L5, R1, R3, L3, R2, L1, R4, R3, L4, R5, L2, L2, R5, R1, R2, L4, L4, L5, R3, L4");
        assert_eq!(166, location.unwrap().distance_from_origin());
    }
}
