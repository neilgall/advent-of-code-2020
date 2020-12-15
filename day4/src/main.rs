use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::str::Chars;

mod parser;
use parser::*;

// --- model

struct PassportData(HashMap<String, String>);

enum Height {
    CM(usize),
    IN(usize)
}

fn parse_height(input: &str) -> ParseResult<Height> {
    let p = or(
        map(seq(integer, string("cm")), |(h, _)| Height::CM(h as usize)),
        map(seq(integer, string("in")), |(h, _)| Height::IN(h as usize))
    );
    p.parse(input)
}

fn height_string_is_valid(s: &str) -> bool {
    if let Ok((_, height)) = parse_height(s) {
        match height {
            Height::CM(h) => (150..=193).contains(&h),
            Height::IN(h) => (59..=76).contains(&h)
        }
    } else {
        false
    }
}

fn has_n_digits(mut chars: Chars, n: usize, radix: u32) -> bool {
    (0..n).all(|_|
        chars.next().map(|c| c.is_digit(radix)).unwrap_or(false)
    ) && chars.next().is_none()
}

fn is_valid_hair_color(s: &str) -> bool {
    let mut chars = s.chars();
    chars.next() == Some('#') && has_n_digits(chars, 6, 16)
}

fn is_valid_eye_color(s: &str) -> bool {
    vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s)
}

impl PassportData {
    fn new(data: Vec<(String,String)>) -> Self {
        PassportData(data.iter().cloned().collect())
    }

    fn contains_required_fields(&self) -> bool {
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .into_iter()
            .all(|key| self.0.contains_key(key))
    }

    fn all_fields_are_valid(&self) -> bool {
        self.birth_year_is_valid()
        && self.issue_year_is_valid()
        && self.expiration_year_is_valid()
        && self.height_is_valid()
        && self.hair_color_is_valid()
        && self.eye_color_is_valid()
        && self.passport_id_is_valid()
    }

    fn year_is_valid(&self, key: &str, valid_range: RangeInclusive<usize>) -> bool {
        if let Some(s) = self.0.get(key) {
            if let Ok(byr) = s.parse::<usize>() {
                return valid_range.contains(&byr)
            }
        }
        false
    }

    fn birth_year_is_valid(&self) -> bool {
        self.year_is_valid("byr", 1920..=2002)
    }

    fn issue_year_is_valid(&self) -> bool {
        self.year_is_valid("iyr", 2010..=2020)
    }

    fn expiration_year_is_valid(&self) -> bool {
        self.year_is_valid("eyr", 2020..=2030)
    }

    fn height_is_valid(&self) -> bool {
        if let Some(s) = self.0.get("hgt") {
            height_string_is_valid(s)
        } else {
            false
        }
    }

    fn hair_color_is_valid(&self) -> bool {
        if let Some(s) = self.0.get("hcl") {
            is_valid_hair_color(s)
        } else {
            false
        }
    }

    fn eye_color_is_valid(&self) -> bool {
        if let Some(s) = self.0.get("ecl") {
            is_valid_eye_color(s)
        } else {
            false
        }
    }

    fn passport_id_is_valid(&self) -> bool {
        if let Some(s) = self.0.get("pid") {
            has_n_digits(s.chars(), 9, 10)
        } else {
            false
        }
    }
}

// --- input file

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
    data.iter().filter(|p| p.contains_required_fields()).count()
}

fn part2(data: &Vec<PassportData>) -> usize {
    data.iter().filter(|p| p.all_fields_are_valid()).count()
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let passport_data = parse_input(&input);
    println!("part1 {}", part1(&passport_data));
    println!("part2 {}", part2(&passport_data));
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

    #[test]
    fn test_has_required_fields() {
        let data = parse_input(sample_input());
        assert_eq!(data[0].contains_required_fields(), true);
        assert_eq!(data[1].contains_required_fields(), false);
        assert_eq!(data[2].contains_required_fields(), true);
        assert_eq!(data[3].contains_required_fields(), false);
    }

    #[test]
    fn test_invalid_passports() {
        let data = parse_input("eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007");

        data.iter().for_each(|p| {
            assert_eq!(p.all_fields_are_valid(), false)
        });
    }

    #[test]
    fn test_valid_passports() {
        let data = parse_input("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719");

        data.iter().for_each(|p| {
            assert_eq!(p.all_fields_are_valid(), true)
        });
    }

    #[test]
    fn test_height_validation() {
        assert_eq!(height_string_is_valid("60in"), true);
        assert_eq!(height_string_is_valid("190cm"), true);
        assert_eq!(height_string_is_valid("190in"), false);
        assert_eq!(height_string_is_valid("190"), false);
    }

    #[test]
    fn test_hair_color_validation() {
        assert_eq!(is_valid_hair_color("#123abc"), true);
        assert_eq!(is_valid_hair_color("#123abz"), false);
        assert_eq!(is_valid_hair_color("#123"), false);
        assert_eq!(is_valid_hair_color("123abc"), false);
        assert_eq!(is_valid_hair_color("#123abcd"), false);
    }

    #[test]
    fn test_eye_color_validation() {
        assert_eq!(is_valid_eye_color("brn"), true);
        assert_eq!(is_valid_eye_color("wat"), false);
    }
}
