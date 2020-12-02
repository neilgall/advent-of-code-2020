
mod parser;

use std::fs::File;
use std::io::prelude::*;
use std::ops::Range;
use parser::*;


// ---- model

#[derive(Debug, Eq, PartialEq)]
struct Password {
	position1: usize,
	position2: usize,
	character: char,
	password: String
}

// ---- model parser

fn range(input: &str) -> ParseResult<Range<usize>> {
	let p = map(seq(first(integer, string("-")), integer), |(start, end)| (start as usize)..(end as usize)+1);
	p.parse(input)
}

fn password(input: &str) -> ParseResult<Password> {
	let pos1p = first(integer, string("-"));
	let pos2p = first(integer, whitespace);
	let charp = first(letter, string(": "));
	let passp = one_or_more(letter);
	let parser = map(seq(pos1p, seq(pos2p, seq(charp, passp))), |(n1, (n2, (c, p)))| Password {
		position1: n1 as usize,
		position2: n2 as usize,
		character: c,
		password: p.into_iter().collect()
	});
	parser.parse(input)
}

// --- input file

fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_input(input: &str) -> ParseResult<Vec<Password>> {
    let p = one_or_more(first(password, whitespace));
    p.parse(input)
}


// --- problem

impl Password {
	fn part1_is_valid(&self) -> bool {
		let n = self.password.chars().filter(|c| c == &self.character).count();
		self.position1 <= n && n <= self.position2
	}

	fn part2_is_valid(&self) -> bool {
		let c1 = self.password.chars().nth(self.position1 - 1) == Some(self.character);
		let c2 = self.password.chars().nth(self.position2 - 1) == Some(self.character);
		(c1 || c2) && !(c1 && c2)
	}
}

fn part1(passwords: &Vec<Password>) -> usize {
	passwords.iter().filter(|p| p.part1_is_valid()).count()
}

fn part2(passwords: &Vec<Password>) -> usize {
	passwords.iter().filter(|p| p.part2_is_valid()).count()
}

fn main() {
	let input = read_file("../input.txt").unwrap();
	let (_, passwords) = parse_input(&input).unwrap();
	println!("part1 {}", part1(&passwords));
	println!("part2 {}", part2(&passwords));
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_passwords() {
		let (rest, passwords) = parse_input("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc").unwrap();
		assert_eq!(rest, "");
		assert_eq!(passwords,
			vec![
				Password { position1: 1, position2: 3, character: 'a', password: String::from("abcde") },
				Password { position1: 1, position2: 3, character: 'b', password: String::from("cdefg") },
				Password { position1: 2, position2: 9, character: 'c', password: String::from("ccccccccc") }
			]
		);
	}

	#[test]
	fn test_part1_is_valid_1() {
		let p = Password { position1: 1, position2: 3, character: 'a', password: String::from("abcde") };
		assert_eq!(p.part1_is_valid(), true);
	}

	#[test]
	fn test_part1_is_valid_2() {
		let p = Password { position1: 1, position2: 3, character: 'b', password: String::from("cdefg") };
		assert_eq!(p.part1_is_valid(), false);
	}

	#[test]
	fn test_part1_is_valid_3() {
		let p = Password { position1: 2, position2: 9, character: 'c', password: String::from("ccccccccc") };
		assert_eq!(p.part1_is_valid(), true);
	}

	#[test]
	fn test_part2_is_valid_1() {
		let p = Password { position1: 1, position2: 3, character: 'a', password: String::from("abcde") };
		assert_eq!(p.part2_is_valid(), true);
	}

	#[test]
	fn test_part2_is_valid_2() {
		let p = Password { position1: 1, position2: 3, character: 'b', password: String::from("cdefg") };
		assert_eq!(p.part2_is_valid(), false);
	}

	#[test]
	fn test_part2_is_valid_3() {
		let p = Password { position1: 2, position2: 9, character: 'c', password: String::from("ccccccccc") };
		assert_eq!(p.part2_is_valid(), false);
	}
}
