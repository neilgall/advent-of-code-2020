use std::fs::File;
use std::io::prelude::*;
use std::ops::{Add, Sub};

mod parser;
use parser::*;

// --- file read

fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

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

struct Ship {
    x: i64,
    y: i64,
    direction: Direction
}

impl Ship {
    fn new() -> Self {
        Ship {
            x: 0,
            y: 0,
            direction: Direction::East
        }
    }

    fn go(&mut self, inst: &Instruction) {
        use Instruction::*;
        match inst {
            North(n) => self.y -= n,
            South(n) => self.y += n,
            East(n) => self.x += n,
            West(n) => self.x -= n,
            Left(n) => self.direction = self.direction + n,
            Right(n) => self.direction = self.direction - n,
            Forward(n) => self.go(&self.direction.to_instruction(*n))
        }
    }

    fn manhattan_distance_from_start(&self) -> Distance {
        self.x.abs() + self.y.abs()
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

fn main() {
    let input = read_file("./input.txt").unwrap();
    let instructions = parse_input(&input).unwrap().1;
    println!("part1 {}", part1(&instructions));
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
    fn test_follow_instructions() {
        use Instruction::*;
        let instructions = vec![Forward(10), North(3), Forward(7), Right(90), Forward(11)];
        assert_eq!(part1(&instructions), 25);
    }

}