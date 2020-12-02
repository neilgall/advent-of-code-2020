
mod parser;

use std::fs::File;
use std::io::prelude::*;
use std::ops::Range;
use parser::*;


// ---- model

#[derive(Debug, Eq, PartialEq)]
struct Password {
	num_chars: Range<usize>,
	character: char,
	password: String
}

// ---- model parser

fn range(input: &str) -> ParseResult<Range<usize>> {
	let p = map(seq(first(integer, string("-")), integer), |(start, end)| (start as usize)..(end as usize)+1);
	p.parse(input)
}

fn password(input: &str) -> ParseResult<Password> {
	let rp = first(range, whitespace);
	let cp = first(letter, string(": "));
	let pp = one_or_more(letter);
	let parser = map(seq(seq(rp, cp), pp), |((r, c), p)| Password {
		num_chars: r,
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
	fn is_valid(&self) -> bool {
		let n = self.password.chars().filter(|c| c == &self.character).count();
		self.num_chars.contains(&n)
	}
}

fn part1(passwords: &Vec<Password>) -> usize {
	passwords.iter().filter(|p| p.is_valid()).count()
}

fn main() {
	let input = read_file("../input.txt").unwrap();
	let (_, passwords) = parse_input(&input).unwrap();
	println!("part1 {}", part1(&passwords));
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
				Password { num_chars: 1..4, character: 'a', password: String::from("abcde") },
				Password { num_chars: 1..4, character: 'b', password: String::from("cdefg") },
				Password { num_chars: 2..10, character: 'c', password: String::from("ccccccccc") }
			]
		);
	}

	#[test]
	fn test_is_valid_1() {
		let p = Password { num_chars: 1..4, character: 'a', password: String::from("abcde") };
		assert_eq!(p.is_valid(), true);
	}

	#[test]
	fn test_is_valid_2() {
		let p = Password { num_chars: 1..4, character: 'b', password: String::from("cdefg") };
		assert_eq!(p.is_valid(), false);
	}

	#[test]
	fn test_is_valid_3() {
		let p = Password { num_chars: 2..10, character: 'c', password: String::from("ccccccccc") };
		assert_eq!(p.is_valid(), true);
	}
}
