use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Eq, PartialEq)]
struct BoardingPass {
    row: usize,
    column: usize
}

impl BoardingPass {
    fn seat_id(&self) -> usize {
        self.row * 8 + self.column
    }
}

fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn decode(s: &str, one: char) -> usize {
    s.chars().fold(0, |r, c| (r << 1) | (if c == one { 1 } else { 0 }))
}

fn to_boarding_pass(s: &str) -> BoardingPass {
    let row = decode(&s[0..7], 'B');
    let column = decode(&s[7..10], 'R');
    BoardingPass { row, column }
}

fn part1(passes: &Vec<BoardingPass>) -> Option<usize> {
    passes.iter().map(|bp| bp.seat_id()).max()
}

fn main() {
    let input = read_file("./input.txt").unwrap();
    let passes: Vec<BoardingPass> = input.lines().map(to_boarding_pass).collect();

    println!("part1 {}", part1(&passes).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deocde() {
        assert_eq!(decode("BFFFBBF", 'B'), 70);
        assert_eq!(decode("RRR", 'R'), 7);
        assert_eq!(decode("FFFBBBF", 'B'), 14);
        assert_eq!(decode("BBFFBBF", 'B'), 102);
    }

    #[test]
    fn test_to_baording_pass() {
        assert_eq!(to_boarding_pass("BFFFBBFRRR"), BoardingPass { row: 70, column: 7 });
        assert_eq!(to_boarding_pass("FFFBBBFRRR"), BoardingPass { row: 14, column: 7 });
        assert_eq!(to_boarding_pass("BBFFBBFRLL"), BoardingPass { row: 102, column: 4 });
    }
}