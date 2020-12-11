use std::fs::File;
use std::io::prelude::*;

// --- file read

fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// --- model

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Cell {
    Floor,
    Empty,
    Occupied
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            'L' => Cell::Empty,
            '#' => Cell::Occupied,
            _ => Cell::Floor
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Layout {
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize
}

impl From<&str> for Layout {
    fn from(s: &str) -> Self {
        let grid: Vec<Vec<Cell>> = s.lines().map(|line| line.trim().chars().map(Cell::from).collect()).collect();
        Layout {
            width: grid[0].len(),
            height: grid.len(),
            grid
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize
}

impl Pos {
    fn neighbours(&self) -> impl Iterator<Item = Pos> + '_ {
        let xmin = if self.x == 0 { 0 } else { self.x - 1 };
        let ymin = if self.y == 0 { 0 } else { self.y - 1 };
        let xmax = self.x + 1;
        let ymax = self.y + 1;
        (ymin..=ymax).flat_map(
            move |y| (xmin..=xmax).map(
                move |x| Pos { x, y }
            )
        ).filter(move |p| p != self)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Offset {
    x: i64,
    y: i64
}

impl std::ops::Add<&Offset> for Pos {
    type Output = Pos;
    fn add(self, off: &Offset) -> Pos {
        Pos { 
            x: (self.x as i64 + off.x) as usize,
            y: (self.y as i64 + off.y) as usize
        }
    }
}

impl Layout {
    fn valid_pos(&self, p: &Pos) -> bool {
        p.x < self.width && p.y < self.height
    }

    fn current(&self, p: &Pos) -> Cell {
        self.grid[p.y][p.x]
    }

    fn occupied_neighbours(&self, p: &Pos) -> usize {
        p.neighbours()
            .filter(|p| 
                self.valid_pos(p) && self.current(p) == Cell::Occupied
            ).count()
    }

    fn find_seat_in_direction(&self, p: &Pos, dir: &Offset) -> Cell {
        let mut pos = *p;
        loop {
            pos = pos + dir;
            if !self.valid_pos(&pos) {
                return Cell::Floor;
            } else {
                let c = self.current(&pos);
                if c != Cell::Floor {
                    return c;
                }
            }
        }
    }

    fn visible_occupied_seats(&self, p: &Pos) -> usize {
        let directions = vec![
            Offset { x: -1, y: -1 },
            Offset { x:  0, y: -1 },
            Offset { x:  1, y: -1 },
            Offset { x: -1, y:  0 },
            Offset { x:  1, y:  0 },
            Offset { x: -1, y:  1 },
            Offset { x:  0, y:  1 },
            Offset { x:  1, y:  1 }
        ];

        directions.iter()
            .filter(|d| self.find_seat_in_direction(p, d) == Cell::Occupied)
            .count()
    }

    fn iter(&self) -> impl Iterator<Item = Cell> + '_ {
        self.grid.iter().flat_map(|row| row.iter().cloned())
    }

    fn count_occupied_seats(&self) -> usize {
        self.iter().filter(|c| *c == Cell::Occupied).count()
    }

    fn next_generation<F>(&self, f: F) -> Layout where F: Fn(&Pos) -> Cell {
        let grid = self.grid.iter().enumerate().map(
             |(y,row)| row.iter().enumerate().map(
                |(x,_)| f(&Pos { x, y })
             ).collect()
        ).collect();
        Layout {
            width: self.width,
            height: self.height,
            grid
        }
    }

    fn next_generation_v1(&self) -> Layout {
        self.next_generation(|p|
            match self.current(p) {
                Cell::Floor => Cell::Floor,
                
                Cell::Empty => {
                    if self.occupied_neighbours(p) == 0 {
                        Cell::Occupied
                    } else {
                        Cell::Empty
                    }
                }

                Cell::Occupied => {
                    if self.occupied_neighbours(p) >= 4 {
                        Cell::Empty
                    } else {
                        Cell::Occupied
                    }
                }
            }
        )
    }

    fn next_generation_v2(&self) -> Layout {
        self.next_generation(|p|
            match self.current(p) {
                Cell::Floor => Cell::Floor,

                Cell::Empty => {
                    if self.visible_occupied_seats(p) == 0 {
                        Cell::Occupied
                    } else {
                        Cell::Empty
                    }
                }

                Cell::Occupied => {
                    if self.visible_occupied_seats(p) >= 5 {
                        Cell::Empty
                    } else {
                        Cell::Occupied
                    }
                }
            }
        )
    }
}

// --- problems

fn run_until_stable<F>(layout: &Layout, f: F) -> Layout where F: Fn(&Layout) -> Layout {
    let mut current = layout.clone();
    loop {
        let next = f(&current);
        if next == current {
            return current;
        } else {
            current = next;
        }
    }
}

fn part1(layout: &Layout) -> usize {
    let stable = run_until_stable(layout, Layout::next_generation_v1);
    stable.count_occupied_seats()    
}

fn part2(layout: &Layout) -> usize {
    let stable = run_until_stable(layout, Layout::next_generation_v2);
    stable.count_occupied_seats()
}


fn main() {
    let input = read_file("./input.txt").unwrap();
    let layout: Layout = input.as_str().into();
    println!("part1 {:?}", part1(&layout));
    println!("part2 {:?}", part2(&layout));
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_grid() -> &'static str {
        "L.LL.LL.LL
         LLLLLLL.LL
         L.L.L..L..
         LLLL.LL.LL
         L.LL.LL.LL
         L.LLLLL.LL
         ..L.L.....
         LLLLLLLLLL
         L.LLLLLL.L
         L.LLLLL.LL"
    }

    fn test_grid_with_occupied_seats() -> &'static str {
        "L.LL.LL.LL
         ##LLLLL.LL
         L.L.L..L..
         LLLL.LL.LL
         L.LL.LL.LL
         L.LLLLL.LL
         ..L.L.....
         LLLLLLLLLL
         L.LLLLLL.L
         L.LLLLL.LL"
    }

    #[test]
    fn test_init() {
        let layout = Layout::from(test_grid());
        assert_eq!(layout.current(&Pos { x: 0, y: 0 }), Cell::Empty);
        assert_eq!(layout.current(&Pos { x: 1, y: 0 }), Cell::Floor);
    }

    #[test]
    fn test_bounds() {
        let layout = Layout::from(test_grid());
        assert!(layout.valid_pos(&Pos { x: 0, y: 0 }));
        assert!(layout.valid_pos(&Pos { x: 9, y: 9 }));
        assert!(!layout.valid_pos(&Pos { x: 10, y: 0 }));
        assert!(!layout.valid_pos(&Pos { x: 0, y: 10 }));
    }

    #[test]
    fn test_neighbours() {
        let ns: Vec<Pos> = Pos { x: 0, y: 0 }.neighbours().collect();
        assert!(ns.contains(&Pos { x: 1, y: 0 }));
        assert!(ns.contains(&Pos { x: 0, y: 1 }));
        assert!(ns.contains(&Pos { x: 1, y: 1 }));
        assert_eq!(ns.len(), 3);

        let ns: Vec<Pos> = Pos { x: 5, y: 0 }.neighbours().collect();
        assert!(ns.contains(&Pos { x: 4, y: 0 }));
        assert!(ns.contains(&Pos { x: 6, y: 0 }));
        assert!(ns.contains(&Pos { x: 4, y: 1 }));
        assert!(ns.contains(&Pos { x: 5, y: 1 }));
        assert!(ns.contains(&Pos { x: 6, y: 1 }));
        assert_eq!(ns.len(), 5);

        let ns: Vec<Pos> = Pos { x: 0, y: 8 }.neighbours().collect();
        assert!(ns.contains(&Pos { x: 0, y: 7 }));
        assert!(ns.contains(&Pos { x: 0, y: 9 }));
        assert!(ns.contains(&Pos { x: 1, y: 7 }));
        assert!(ns.contains(&Pos { x: 1, y: 8 }));
        assert!(ns.contains(&Pos { x: 1, y: 9 }));
        assert_eq!(ns.len(), 5);

        let ns: Vec<Pos> = Pos { x: 6, y: 3 }.neighbours().collect();
        assert!(ns.contains(&Pos { x: 5, y: 2 }));
        assert!(ns.contains(&Pos { x: 6, y: 2 }));
        assert!(ns.contains(&Pos { x: 7, y: 2 }));
        assert!(ns.contains(&Pos { x: 5, y: 3 }));
        assert!(ns.contains(&Pos { x: 7, y: 3 }));
        assert!(ns.contains(&Pos { x: 5, y: 4 }));
        assert!(ns.contains(&Pos { x: 6, y: 4 }));
        assert!(ns.contains(&Pos { x: 7, y: 4 }));
        assert_eq!(ns.len(), 8);
    }

    #[test]
    fn test_occupied_neighbours() {
        let layout = Layout::from(test_grid());
        assert_eq!(layout.occupied_neighbours(&Pos { x: 0, y: 0 }), 0);        

        let layout = Layout::from(test_grid_with_occupied_seats());
        assert_eq!(layout.occupied_neighbours(&Pos { x: 0, y: 0 }), 2);        
    }

    #[test]
    fn test_generations_v1() {
        let layout = Layout::from(test_grid());

        let gen1 = layout.next_generation_v1();
        assert_eq!(gen1, Layout::from(
            "#.##.##.##
            #######.##
            #.#.#..#..
            ####.##.##
            #.##.##.##
            #.#####.##
            ..#.#.....
            ##########
            #.######.#
            #.#####.##"
        ));

        let gen2 = gen1.next_generation_v1();
        assert_eq!(gen2, Layout::from(
            "#.LL.L#.##
             #LLLLLL.L#
             L.L.L..L..
             #LLL.LL.L#
             #.LL.LL.LL
             #.LLLL#.##
             ..L.L.....
             #LLLLLLLL#
             #.LLLLLL.L
             #.#LLLL.##"
        ));

        let gen3 = gen2.next_generation_v1();
        assert_eq!(gen3, Layout::from(
            "#.##.L#.##
             #L###LL.L#
             L.#.#..#..
             #L##.##.L#
             #.##.LL.LL
             #.###L#.##
             ..#.#.....
             #L######L#
             #.LL###L.L
             #.#L###.##"
        ));
    }

    #[test]
    fn test_generations_v2() {
        let layout = Layout::from(test_grid());

        let gen1 = layout.next_generation_v2();
        assert_eq!(gen1, Layout::from(
            "#.##.##.##
             #######.##
             #.#.#..#..
             ####.##.##
             #.##.##.##
             #.#####.##
             ..#.#.....
             ##########
             #.######.#
             #.#####.##"
        ));

        let gen2 = gen1.next_generation_v2();
        assert_eq!(gen2, Layout::from(
            "#.LL.LL.L#
             #LLLLLL.LL
             L.L.L..L..
             LLLL.LL.LL
             L.LL.LL.LL
             L.LLLLL.LL
             ..L.L.....
             LLLLLLLLL#
             #.LLLLLL.L
             #.LLLLL.L#"
        ));

        let gen3 = gen2.next_generation_v2();
        assert_eq!(gen3, Layout::from(
            "#.L#.##.L#
             #L#####.LL
             L.#.#..#..
             ##L#.##.##
             #.##.#L.##
             #.#####.#L
             ..#.#.....
             LLL####LL#
             #.L#####.L
             #.L####.L#"
        ));
    }
}