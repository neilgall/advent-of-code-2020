use std::ops::{Add, Sub};
use parser::*;

// --- model

type Distance = i64;
type Rotation = i64;

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    North(Distance),
    South(Distance),
    East(Distance),
    West(Distance),
    Left(Rotation),
    Right(Rotation),
    Forward(Distance)
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    East = 0,
    North = 90,
    West = 180,
    South = 270
}

impl From<i64> for Direction {
    fn from(i: i64) -> Direction {
        match i {
            0 => Direction::East,
            90 => Direction::North,
            180 => Direction::West,
            270 => Direction::South,
            _ => panic!("invalid direction")
        }
    }
}

impl Add<&Rotation> for Direction {
    type Output = Direction;

    fn add(self, r: &Rotation) -> Direction {
        Direction::from(((self as i64) + r) % 360)
    }
}

impl Sub<&Rotation> for Direction {
    type Output = Direction;

    fn sub(self, r: &Rotation) -> Direction {
        Direction::from(((self as i64) + 360 - r) % 360)
    }
}

impl Direction {
    fn to_instruction(&self, distance: Distance) -> Instruction {
        match self {
            Direction::North => Instruction::North(distance),
            Direction::South => Instruction::South(distance),
            Direction::East => Instruction::East(distance),
            Direction::West => Instruction::West(distance)            
        }
    }
}

#[derive(Debug,PartialEq,Eq)]
struct Pos {
    x: i64,
    y: i64
}

impl Pos {
    fn rotate_around(&self, origin: &Pos, rotation: Rotation) -> Pos {
        let x = self.x - origin.x;
        let y = self.y - origin.y;
        let (new_x, new_y) = match rotation {
            90 => (-y, x),
            180 => (-x, -y),
            270 => (y, -x),
            _ => panic!("invalid rotation")
        };
        Pos {
            x: origin.x + new_x,
            y: origin.y + new_y
        }
    }
}

struct Ship {
    pos: Pos,
    direction: Direction
}

impl Ship {
    fn new() -> Self {
        Ship {
            pos: Pos { x: 0, y: 0 },
            direction: Direction::East
        }
    }

    fn go(&mut self, inst: &Instruction) {
        use Instruction::*;
        match inst {
            North(n) => self.pos.y += n,
            South(n) => self.pos.y -= n,
            East(n) => self.pos.x += n,
            West(n) => self.pos.x -= n,
            Left(n) => self.direction = self.direction + n,
            Right(n) => self.direction = self.direction - n,
            Forward(n) => self.go(&self.direction.to_instruction(*n))
        }
    }

    fn manhattan_distance_from_start(&self) -> Distance {
        self.pos.x.abs() + self.pos.y.abs()
    }
}

struct WaypointShip {
    ship: Pos,
    waypoint: Pos
}

impl WaypointShip {
    fn new() -> Self {
        WaypointShip {
            ship: Pos { x: 0, y : 0 },
            waypoint: Pos { x: 10, y: 1 }
        }
    }

    fn go(&mut self, inst: &Instruction) {
        use Instruction::*;
        match inst {
            North(n) => self.waypoint.y += n,
            South(n) => self.waypoint.y -= n,
            East(n) => self.waypoint.x += n,
            West(n) => self.waypoint.x -= n,
            Left(n) => self.waypoint = self.waypoint.rotate_around(&self.ship, *n),
            Right(n) => self.waypoint = self.waypoint.rotate_around(&self.ship, 360-(*n)),
            Forward(n) => {
                let x = (self.waypoint.x - self.ship.x) * n;
                let y = (self.waypoint.y - self.ship.y) * n;
                self.ship.x += x;
                self.ship.y += y;
                self.waypoint.x += x;
                self.waypoint.y += y;
            }
        }
    }

    fn manhattan_distance_from_start(&self) -> Distance {
        self.ship.x.abs() + self.ship.y.abs()
    }
}

// --- parser

fn parse_input(input: &str) -> ParseResult<Vec<Instruction>> {
    let north = right(match_literal("N"), integer).map(Instruction::North);
    let south = right(match_literal("S"), integer).map(Instruction::South);
    let east = right(match_literal("E"), integer).map(Instruction::East);
    let west = right(match_literal("W"), integer).map(Instruction::West);
    let tright = right(match_literal("R"), integer).map(Instruction::Right);
    let tleft = right(match_literal("L"), integer).map(Instruction::Left);
    let forward = right(match_literal("F"), integer).map(Instruction::Forward);
    let instruction = north.or(south).or(east).or(west).or(tright).or(tleft).or(forward);
    let parser = one_or_more(whitespace_wrap(instruction));

    parser.parse(input)
}

// --- problems

fn part1(instructions: &Vec<Instruction>) -> i64 {
    let mut ship = Ship::new();
    instructions.iter().for_each(|i| ship.go(i));
    ship.manhattan_distance_from_start()
}

fn part2(instructions: &Vec<Instruction>) -> i64 {
    let mut ship = WaypointShip::new();
    instructions.iter().for_each(|i| ship.go(i));
    ship.manhattan_distance_from_start()
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let instructions = parse_input(&input).unwrap().1;
    println!("part1 {}", part1(&instructions));
    println!("part2 {}", part2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        use Instruction::*;
        let instructions = parse_input("F10\nN3\nF7\nR90\nF11");
        assert_eq!(instructions, Ok(("", vec![Forward(10), North(3), Forward(7), Right(90), Forward(11)])));
    }

    #[test]
    fn test_part1() {
        use Instruction::*;
        let instructions = vec![Forward(10), North(3), Forward(7), Right(90), Forward(11)];
        assert_eq!(part1(&instructions), 25);
    }

    #[test]
    fn test_part2() {
        use Instruction::*;
        let instructions = vec![Forward(10), North(3), Forward(7), Right(90), Forward(11)];
        assert_eq!(part2(&instructions), 286);
    }

    #[test]
    fn test_rotate_around_90() {
        let origin = Pos { x: 0, y: 0 };
        let pos = Pos { x: 10, y: 1 };
        assert_eq!(pos.rotate_around(&origin, 90), Pos { x: -1, y: 10 });
    }

    #[test]
    fn test_rotate_around_180() {
        let origin = Pos { x: 0, y: 0 };
        let pos = Pos { x: 10, y: 1 };
        assert_eq!(pos.rotate_around(&origin, 180), Pos { x: -10, y: -1 });
    }

    #[test]
    fn test_rotate_around_270() {
        let origin = Pos { x: 0, y: 0 };
        let pos = Pos { x: 10, y: 1 };
        assert_eq!(pos.rotate_around(&origin, 270), Pos { x: 1, y: -10 });
    }

}