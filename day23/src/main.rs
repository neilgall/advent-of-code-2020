use std::ptr;

// -- model

type CupID = usize;

#[derive(Debug)]
struct Cup {
    id: CupID,
    next: *mut Cup
}

#[derive(Debug)]
struct Cups {
    length: usize,
    cups: Vec<Cup>,
    current: *mut Cup
}

fn str_as_cup_ids(s: &str) -> impl Iterator<Item = CupID> + '_ {
    s.chars().map(|c| (c as CupID) - 48)
}

impl Cups {
    fn new<I>(input: I, current_cup: CupID) -> Self where I: Iterator<Item = CupID> {
        let ids: Vec<CupID> = input.collect();
        let mut cups: Vec<Cup> = (1..=ids.len()).map(|id| Cup { id, next: ptr::null_mut() }).collect();

        for i in 0..cups.len()-1 {
            cups[ids[i]-1].next = &mut cups[ids[i+1]-1] as *mut Cup;
        }

        unsafe {
            let len = cups.len();
            let mut last = &mut cups[ids[len-1]-1] as *mut Cup;
            (*last).next = &mut cups[ids[0]-1] as *mut Cup;
        }

        let current = &mut cups[current_cup-1] as *mut Cup;

        Cups {
            length: cups.len(),
            cups,
            current
        }
    }

    fn prev_id(&self, cup: CupID) -> CupID {
        if cup == 1 { self.length as CupID } else { cup - 1 }
    }

    fn from_str(input: &str, current_cup: CupID) -> Self {
        Cups::new(str_as_cup_ids(input), current_cup)
    }

    fn labels(&self) -> Vec<CupID> {
        let mut labels = vec![];
        unsafe {
            let mut cup = self.cups[0].next as *const Cup;
            for _ in 1..self.length {
                labels.push((*cup).id);
                cup = (*cup).next as *const Cup;
            }
        }
        labels
    }

    fn labels_as_str(&self) -> String {
        self.labels().iter().map(|n| n.to_string()).collect::<Vec<String>>().join("")
    }

    fn apply_move(&mut self) {
        unsafe {
            let curr = self.current;
            let next1 = (*curr).next;
            let next2 = (*next1).next;
            let next3 = (*next2).next;
            let next4 = (*next3).next;

            let mut dest_id = self.prev_id((*curr).id);
            let mut dest;
            loop {
                dest = &mut self.cups[dest_id - 1];
                if ptr::eq(dest, next1) || ptr::eq(dest, next2) || ptr::eq(dest, next3) {
                    dest_id = self.prev_id(dest_id);
                } else {
                    break;
                }
            }

            let dest_next1 = (*dest).next;

            // remove next1..next3 from ring
            (*curr).next = next4;

            // reinsert after dest
            (*dest).next = next1;
            (*next3).next = dest_next1;

            self.current = (*self.current).next;
        }
   }

    fn apply_n_moves(&mut self, count: usize) {
        for _ in 0..count {
            self.apply_move();
        }
    }
}

// -- problems

fn part1(input: &str, start_cup: CupID) -> String {
    let mut cups = Cups::from_str(input, start_cup);
    cups.apply_n_moves(100);
    cups.labels_as_str()
}

fn part2(input: &str, start_cup: CupID) -> CupID {
    let mut cups = Cups::new(str_as_cup_ids(input).chain(10..=1_000_000), start_cup);
    cups.apply_n_moves(10_000_000);

    let first_2: Vec<CupID> = cups.labels().iter().take(2).copied().collect();
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