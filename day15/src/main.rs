use std::collections::HashMap;

type Turn = usize;
type Number = i64;

struct NumberGame {
    last_turns: HashMap<Number, Turn>,
    prev_turns: HashMap<Number, Turn>,
    starting_numbers: Vec<Number>,
    next_turn: Turn,
    last_spoken: Number
}

impl NumberGame {
    fn new(starting_numbers: &[Number]) -> Self {
        NumberGame {
            last_turns: HashMap::new(),
            prev_turns: HashMap::new(),
            starting_numbers: starting_numbers.iter().cloned().collect(),
            next_turn: 0,
            last_spoken: 0
        }
    }

    fn iter(&mut self) -> impl Iterator<Item = i64> + '_ {
        std::iter::from_fn(move || {
            let next: Number = if self.next_turn < self.starting_numbers.len() {
                self.starting_numbers[self.next_turn]
            } else {
                let last = self.last_turns.get(&self.last_spoken).unwrap();
                match self.prev_turns.get(&self.last_spoken) {
                    None => 0,
                    Some(prev) => (last - prev) as Number
                }
            };

            if let Some(prev) = self.last_turns.get(&next) {
                self.prev_turns.insert(next, *prev);
            }
            self.last_turns.insert(next, self.next_turn);
            self.last_spoken = next;
            self.next_turn += 1;

            Some(next)
        })
    }
}


fn number_spoken_at_index(starting_numbers: &[Number], target_index: Turn) -> Number {
    NumberGame::new(starting_numbers)
        .iter()
        .skip(target_index - 1)
        .next()
        .unwrap()
}

fn part1(starting_numbers: &[Number]) -> Number {
    number_spoken_at_index(starting_numbers, 2020)
}

fn part2(starting_numbers: &[Number]) -> Number {
    number_spoken_at_index(starting_numbers, 30000000)
}


fn main() {
    let input = [15,5,1,4,7,0];
    println!("part 1 {}", part1(&input));
    println!("part 2 {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_spoken_at_index() {
        assert_eq!(number_spoken_at_index(&[0,3,6], 10), 0);
        assert_eq!(number_spoken_at_index(&[0,3,6], 30000000), 175594);
    }
}
