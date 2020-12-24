
// -- model

type Cup = usize;

#[derive(Debug)]
struct Cups {
    length: usize,
    cups: Vec<Cup>,
    current: Cup
}

fn str_as_cup_ids(s: &str) -> impl Iterator<Item = Cup> + '_ {
    s.chars().map(|c| (c as Cup) - 48)
}

impl Cups {
    fn new<I>(input: I, current: Cup) -> Self where I: Iterator<Item = Cup> {
        let ids: Vec<Cup> = input.collect();
        let last = ids.len()-1;
        let mut cups: Vec<Cup> = (2..=ids.len()+1).collect();
        cups[last] = 1;

        for i in 1..ids.len() {
            cups[ids[i-1]-1] = ids[i];
        }
        cups[ids[last]-1] = ids[0];

        Cups {
            length: cups.len(),
            cups,
            current
        }
    }

    fn from_str(input: &str, current_cup: Cup) -> Self {
        Cups::new(str_as_cup_ids(input), current_cup)
    }

    fn next(&self, id: Cup) -> Cup {
        self.cups[id-1]
    }

    fn set(&mut self, from: Cup, to: Cup) {
        self.cups[from-1] = to;
    }

    fn prev_id(&self, cup: Cup) -> Cup {
        if cup == 1 { self.length as Cup } else { cup - 1 }
    }

    fn labels(&self) -> Vec<Cup> {
        let mut labels = vec![];
        let mut cup = self.next(1);
        for _ in 1..self.length {
            labels.push(cup);
            cup = self.next(cup);
        }
        labels
    }

    fn labels_as_str(&self) -> String {
        self.labels().iter().map(|n| n.to_string()).collect::<Vec<String>>().join("")
    }

    fn apply_move(&mut self) {
        let curr = self.current;
        let next1 = self.next(curr);
        let next2 = self.next(next1);
        let next3 = self.next(next2);
        let next4 = self.next(next3);

        let mut dest = self.prev_id(curr);
        while dest == next1 || dest == next2 || dest == next3 {
            dest = self.prev_id(dest);
        }

        let dest_next1 = self.next(dest);

        // remove next1..next3 from ring
        self.set(curr, next4);

        // reinsert after dest
        self.set(dest, next1);
        self.set(next3, dest_next1);

        self.current = self.next(curr);
   }

    fn apply_n_moves(&mut self, count: usize) {
        for _ in 0..count {
            self.apply_move();
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
    let mut cups = Cups::new(str_as_cup_ids(input).chain(10..=1_000_000), start_cup);
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
    fn test_10_moves() {
        let mut cups = Cups::from_str("389125467", 3);
        println!("{:?}", cups.labels());
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