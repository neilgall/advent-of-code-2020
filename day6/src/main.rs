use std::collections::HashSet;
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

type Person = Vec<char>;

struct Group {
    people: Vec<Person>
}

impl From<&str> for Group {
    fn from(s: &str) -> Self {
        Group {
            people: s.lines().map(|line| line.chars().collect()).collect()
        }
    }
}

impl Group {
    fn yesses(&self) -> usize {
        let all: HashSet<&char> = self.people.iter().flat_map(|p| p.iter()).collect();
        all.len()
    }
}


// --- problems 


fn part1(groups: &Vec<Group>) -> usize {
    groups.iter().map(|g| g.yesses()).sum()
}

fn part2(groups: &Vec<Group>) -> usize {
    0
}

fn main() {
    let input = read_file("./input.txt").unwrap();
    let groups: Vec<Group> = input.split("\n\n").map(Group::from).collect();

    println!("part1 {}", part1(&groups));
    println!("part2 {}", part2(&groups));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_group() {
        assert_eq!(Group::from("abc").yesses(), 3);
        assert_eq!(Group::from("a\nb\nc").yesses(), 3);
        assert_eq!(Group::from("ab\nac").yesses(), 3);
        assert_eq!(Group::from("a\na\na\na").yesses(), 1);
        assert_eq!(Group::from("b").yesses(), 1);
    }
}