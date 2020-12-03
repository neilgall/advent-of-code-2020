
use std::fs::File;
use std::io::prelude::*;

// --- model

#[derive(Debug)]
struct Model {
    width: usize,
    height: usize,
    bitmap: Vec<Vec<char>>
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize
}

#[derive(Debug, Clone, Copy)]
struct Offset {
    x: usize,
    y: usize
}

impl std::ops::Add<Offset> for Pos {
    type Output = Pos;

    fn add(self, offset: Offset) -> Self::Output {
        Pos { 
            x: self.x + offset.x,
            y: self.y + offset.y
        }
    }
}

impl Pos {
    fn slope(self, offset: Offset) -> impl Iterator<Item = Pos> {
        let mut pos = self;
        std::iter::from_fn(move || {
            let result = pos;
            pos = pos + offset;
            Some(result)
        })
    }
}

impl Model {
    fn tree_at(&self, p: &Pos) -> bool {
        self.bitmap[p.y % self.height][p.x % self.width] == '#'
    }

    fn count_trees_on_slope(&self, start: Pos, slope: Offset) -> usize {
        start.slope(slope)
            .take_while(|p| p.y < self.height)
            .filter(|p| self.tree_at(&p))
            .count()
    }
}

// --- input file

fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_input(input: &str) -> Model {
    let bitmap: Vec<Vec<char>> = input.lines().map(|line| line.trim().chars().collect()).collect();
    let min_length = bitmap.iter().map(|row| row.len()).min();
    let max_length = bitmap.iter().map(|row| row.len()).max();
    let length = bitmap.iter().next().map(|row| row.len());
    if length != min_length || length != max_length {
        panic!();
    }

    Model {
        width: length.unwrap(),
        height: bitmap.len(),
        bitmap
    }
}

// --- problems

fn part1(model: &Model) -> usize {
    model.count_trees_on_slope(Pos { x: 0, y: 0 }, Offset { x: 3, y: 1 })
}

fn part2(model: &Model) -> usize {
    let offsets = vec![
        Offset { x: 1, y: 1 },
        Offset { x: 3, y: 1 },
        Offset { x: 5, y: 1 },
        Offset { x: 7, y: 1 },
        Offset { x: 1, y: 2 }
    ];
    let start = Pos { x: 0, y: 0 };

    offsets.iter()
        .map(|offset| model.count_trees_on_slope(start, *offset))
        .product()
}

fn main() {
    let input = read_file("../input.txt").unwrap();
    let model = parse_input(&input);
    println!("part1 {}", part1(&model));
    println!("part2 {}", part2(&model));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
}

    #[test]
    fn test_parse_input() {
        let model = parse_input(sample_input());
        assert_eq!(model.width, 11);
        assert_eq!(model.height, 11);
        assert_eq!(model.bitmap[0][0], '.');
        assert_eq!(model.bitmap[8][7], '#');
    }

    #[test]
    fn test_count_trees_on_slope() {
        let model = parse_input(sample_input());
        assert_eq!(model.count_trees_on_slope(Pos { x: 0, y: 0 }, Offset { x: 1, y: 1 }), 2);
        assert_eq!(model.count_trees_on_slope(Pos { x: 0, y: 0 }, Offset { x: 3, y: 1 }), 7);
        assert_eq!(model.count_trees_on_slope(Pos { x: 0, y: 0 }, Offset { x: 5, y: 1 }), 3);
        assert_eq!(model.count_trees_on_slope(Pos { x: 0, y: 0 }, Offset { x: 7, y: 1 }), 4);
        assert_eq!(model.count_trees_on_slope(Pos { x: 0, y: 0 }, Offset { x: 1, y: 2 }), 2);
    }
}
