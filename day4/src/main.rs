use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

mod parser;
use parser::*;

// --- model

struct PassportData(HashMap<String, String>);

impl PassportData {
    fn new(data: Vec<(String,String)>) -> Self {
        PassportData(data.iter().cloned().collect())
    }

    fn is_valid(&self) -> bool {
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .into_iter()
            .all(|key| self.0.contains_key(key))
    }
}

// --- input file

fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_input(input: &str) -> Vec<PassportData> {
    let tag = map(one_or_more(letter), |ls| ls.iter().collect());
    let value = map(one_or_more(non_whitespace), |cs| cs.iter().collect());
    let item = between(tag, string(":"), value);
    let passport = map(one_or_more(first(item, whitespace)), PassportData::new);

    input.split("\n\n")
        .map(|inp| passport.parse(inp).unwrap().1)
        .collect()
}


// --- problems

fn part1(data: &Vec<PassportData>) -> usize {
    data.iter().filter(|p| p.is_valid()).count()
}


fn main() {
    let input = read_file("./input.txt").unwrap();
    let passport_data = parse_input(&input);
    println!("part1 {}", part1(&passport_data));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
    }

    #[test]
    fn test_parse_input() {
        let data = parse_input(sample_input());
        assert_eq!(data.len(), 4);

        vec![
            ("ecl", "gry"),
            ("pid", "860033327"),
            ("eyr", "2020"),
            ("hcl", "#fffffd"),
            ("byr", "1937"),
            ("iyr", "2017"),
            ("cid", "147"),
            ("hgt", "183cm")
        ].into_iter().for_each(|(k,v)| {
            assert_eq!(data[0].0.get(k), Some(&String::from(v)));
        });
    }
}
