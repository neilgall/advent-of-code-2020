use std::collections::HashSet;

// --- model

type Person = HashSet<char>;

struct Group {
    people: Vec<Person>
}

impl From<&str> for Group {
    fn from(s: &str) -> Self {
        let people = s.lines()
                        .map(|line| line.chars().collect())
                        .collect();
        Group { people }
    }
}

impl Group {
    fn anyone_yesses(&self) -> usize {
        self.people.iter()
            .flat_map(|p| p.iter())
            .collect::<HashSet<&char>>()
            .len()
    }

    fn everyone_yesses(&self) -> usize {
        self.people.iter()
            .fold(
                ('a'..='z').collect::<HashSet<char>>(),
                |r, p| r.intersection(p).cloned().collect()
            )
            .len()

    }
}

// --- problems 

fn part1(groups: &Vec<Group>) -> usize {
    groups.iter().map(|g| g.anyone_yesses()).sum()
}

fn part2(groups: &Vec<Group>) -> usize {
    groups.iter().map(|g| g.everyone_yesses()).sum()
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let groups: Vec<Group> = input.split("\n\n").map(Group::from).collect();

    println!("part1 {}", part1(&groups));
    println!("part2 {}", part2(&groups));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anyone_yesses() {
        assert_eq!(Group::from("abc").anyone_yesses(), 3);
        assert_eq!(Group::from("a\nb\nc").anyone_yesses(), 3);
        assert_eq!(Group::from("ab\nac").anyone_yesses(), 3);
        assert_eq!(Group::from("a\na\na\na").anyone_yesses(), 1);
        assert_eq!(Group::from("b").anyone_yesses(), 1);
    }

    #[test]
    fn test_everyone_yesses() {
        assert_eq!(Group::from("abc").everyone_yesses(), 3);
        assert_eq!(Group::from("abc\nabcd").everyone_yesses(), 3);
        assert_eq!(Group::from("a\nb\nc").everyone_yesses(), 0);
        assert_eq!(Group::from("ab\nac").everyone_yesses(), 1);
        assert_eq!(Group::from("a\na\na\na").everyone_yesses(), 1);
        assert_eq!(Group::from("b").everyone_yesses(), 1);
    }
}