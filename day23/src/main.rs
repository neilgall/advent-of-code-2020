use std::io::Write;
use std::iter::once;
use std::time::{Duration, SystemTime};

// -- model

type Cup = u32;

#[derive(Debug)]
struct Cups {
    length: usize,
    offset: usize,
    cups: Vec<Cup>
}

#[derive(Debug)]
struct Move {
    removed: Vec<Cup>,
    destination: Cup
}

fn prev(cup: Cup) -> Cup {
    if cup == 1 { 9 } else { cup - 1 }
}

fn str_as_cups(s: &str) -> impl Iterator<Item = Cup> + '_ {
    s.chars().map(|c| (c as Cup) - 48)
}

fn estimate_remaining(done: usize, total: usize, start: &SystemTime) -> Duration {
    if done == 0 {
        Duration::from_secs(0)
    } else {
        start.elapsed().unwrap() * (total as u32) / (done as u32)
    }
}

impl Cups {
    fn new<I>(input: I, current_cup: Cup) -> Self where I: Iterator<Item = Cup> {
        let cups: Vec<Cup> = input.collect();
        let offset = cups.iter().position(|c| *c == current_cup).unwrap();
        let length = cups.len();
        Cups {
            cups,
            length,
            offset
        }
    }

    fn from_str(input: &str, current_cup: Cup) -> Self {
        Cups::new(str_as_cups(input), current_cup)
    }

    fn current_cup(&self) -> Cup {
        self.cups[self.offset]
    }

    fn index_of_cup(&self, cup: Cup) -> usize {
        self.cups.iter().position(|c| *c == cup).unwrap()
    }

    fn iter(&self) -> impl Iterator<Item = Cup> + '_ {
        self.cups.iter().cycle().skip(self.offset).copied()
    }

    fn labels(&self) -> Vec<Cup> {
        let start = (self.index_of_cup(1) + 1) % self.length;
        self.cups.iter().cycle().skip(start).take(self.length-1).copied().collect()
    }

    fn labels_as_str(&self) -> String {
        self.labels().iter().map(|n| n.to_string()).collect::<Vec<String>>().join("")
    }

    fn create_move(&self) -> Move {
        let removed: Vec<Cup> = self.iter().skip(1).take(3).collect();
        let mut destination = prev(self.current_cup());
        while removed.contains(&destination) {
            destination = prev(destination);
        }
        Move {
            removed,
            destination
        }
    }

    fn apply(&mut self, m: Move) {
        let dest_index = self.index_of_cup(m.destination);

        let remaining_cups = self.cups.iter()
            .cycle()
            .skip(dest_index+1)
            .filter(|c| !m.removed.contains(c))
            .take(self.length - 4);

        let new_cups: Vec<Cup> = once(&m.destination)
            .chain(m.removed.iter())
            .chain(remaining_cups)
            .copied()
            .collect();

        self.cups = new_cups;

        let diff = (dest_index + self.length - self.offset) % self.length;
        let new_offset = if diff <= 4 { 0 } else { self.length - diff + 4 };

        // println!("offset={} dest={} diff={} cups={:?} new_offset={}", self.offset, dest_index, diff, self.cups, new_offset);
        self.offset = new_offset;
    }

    fn apply_n_moves(&mut self, count: usize) {
        let start = SystemTime::now();
        for n in 0..count {
            self.apply(self.create_move());
            if n % 1000 == 0 {
                print!("{}... {:?} remaining\r", n, estimate_remaining(n, count, &start));
                std::io::stdout().flush().unwrap();
            }
        }
    }
}

// -- problems

fn part1(input: &str, start_cup: Cup) -> String {
    let mut cups = Cups::from_str(input, start_cup);
    cups.apply_n_moves(100);
    cups.labels_as_str()
}

fn part2(input: &str, start_cup: Cup) -> Cup {
    let mut cups = Cups::new(str_as_cups(input).chain(10..=1_000_000), start_cup);
    cups.apply_n_moves(10_000_000);

    let first_2: Vec<Cup> = cups.labels().iter().take(2).copied().collect();
    first_2[0] * first_2[1]
}

fn main() {
    let input = "523764819";
    println!("part 1 {}", part1(input, 5));
    println!("part 2 {}", part2(input, 5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_current_3() {
        let cups = Cups::from_str("389125467", 3);
        let x: Vec<Cup> = cups.iter().take(15).collect();
        assert_eq!(x, vec![3, 8, 9, 1, 2, 5, 4, 6, 7, 3, 8, 9, 1, 2, 5]);
    }

    #[test]
    fn test_iter_current_2() {
        let cups = Cups::from_str("389125467", 2);
        let x: Vec<Cup> = cups.iter().take(15).collect();
        assert_eq!(x, vec![2, 5, 4, 6, 7, 3, 8, 9, 1, 2, 5, 4 ,6, 7, 3]);
    }

    #[test]
    fn test_create_move_1() {
        let cups = Cups::from_str("389125467", 3);
        let mv = cups.create_move();
        assert_eq!(mv.removed, vec![8, 9, 1]);
        assert_eq!(mv.destination, 2);
    }

    #[test]
    fn test_create_move_3() {
        let cups = Cups::from_str("325467891", 5);
        let mv = cups.create_move();
        assert_eq!(mv.removed, vec![4, 6, 7]);
        assert_eq!(mv.destination, 3);
    }

    #[test]
    fn test_apply_move_1() {
        let mut cups = Cups::from_str("389125467", 3);
        cups.apply(cups.create_move());
        assert_eq!(cups.labels(), vec![5, 4, 6, 7, 3, 2, 8, 9]);
        assert_eq!(cups.current_cup(), 2);
    }

    #[test]
    fn test_apply_move_2() {
        let mut cups = Cups::from_str("328915467", 2);
        println!("move {:?}", cups.create_move());
        cups.apply(cups.create_move());
        assert_eq!(cups.labels(), vec![3, 2, 5, 4, 6, 7, 8, 9]);
        assert_eq!(cups.current_cup(), 5);
    }

    #[test]
    fn test_apply_move_4() {
        let mut cups = Cups::from_str("725891346", 8);
        cups.apply(cups.create_move());
        assert_eq!(cups.labels(), vec![3, 2, 5, 8, 4, 6,7, 9]);
        assert_eq!(cups.current_cup(), 4);
    }

    #[test]
    fn test_apply_move_5() {
        let mut cups = Cups::from_str("325846791", 4);
        cups.apply(cups.create_move());
        assert_eq!(cups.labels(), vec![3, 6, 7, 9, 2, 5, 8, 4]);
        assert_eq!(cups.current_cup(), 1);
    }

    #[test]
    fn test_apply_move_9() {
        let mut cups = Cups::from_str("741583926", 6);
        let mv = cups.create_move();
        assert_eq!(mv.removed, vec![7, 4, 1]);
        assert_eq!(mv.destination, 5);
        cups.apply(mv);
        assert_eq!(cups.labels(), vec![8, 3, 9, 2, 6, 5, 7, 4]);
        assert_eq!(cups.current_cup(), 5);
    }

    #[test]
    fn test_10_moves() {
        let mut cups = Cups::from_str("389125467", 3);
        cups.apply_n_moves(10);
        assert_eq!(cups.labels(), vec![9, 2, 6, 5, 8, 3, 7, 4]);
    }

    #[test]
    fn test_100_moves() {
        let mut cups = Cups::from_str("389125467", 3);
        cups.apply_n_moves(100);
        assert_eq!(cups.labels(), vec![6, 7, 3, 8, 4, 5, 2, 9]);        
    }

}