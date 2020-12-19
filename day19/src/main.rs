use std::collections::HashMap;
use parser::*;

// --- model

#[derive(Debug, Clone, PartialEq)]
enum Rule {
    MatchChar(char),
    Sequence(Vec<usize>),
    Alternative(Vec<usize>, Vec<usize>)
}

#[derive(Debug, PartialEq)]
struct Rules {
    rules: HashMap<usize, Rule>
}

type MatchResult<'a> = Result<&'a str, &'a str>;

impl Rules {
    fn get(&self, id: &usize) -> &Rule {
        self.rules.get(id).unwrap()
    }

    fn is_tailrec(&self, rule_id: &usize) -> bool {
        match self.get(rule_id) {
            Rule::Sequence(rs) => rs.last() == Some(rule_id),
            Rule::Alternative(xs, ys) => xs.last() == Some(rule_id) || ys.last() == Some(rule_id),
            _ => false
        }
    }

    fn match_seq<'a>(&self, seq: &[usize], input: &'a str) -> MatchResult<'a> {
        let mut remainings: Vec<&'a str> = vec![input];
        for rule in seq {
            if remainings.is_empty() {
                break;
            }
            if self.is_tailrec(rule) {
                remainings = remainings.iter().flat_map(|remaining|
                    self.match_tailrec_rule(rule, remaining)
                ).collect();
            } else {
                remainings = remainings.iter().filter_map(|remaining|
                    self.match_rule(rule, remaining).ok()
                ).collect();
            }
        }
        
        remainings.into_iter().next().ok_or(input)
    }

    fn match_seq_tailrec<'a>(&self, id: &usize, seq: &[usize], input: &'a str) -> Vec<&'a str> {
        if seq.last() != Some(id) {
            self.match_seq(seq, input).map(|r| vec![r]).unwrap_or(vec![])
        } else {
            let init = &seq[0..seq.len()-1];
            if let Ok(mut remaining) = self.match_seq(init, input) {
                let mut results = vec![remaining];
                while let Ok(next_remaining) = self.match_seq(init, remaining) {
                    results.push(next_remaining);
                    remaining = next_remaining;
                }
                results
            } else {
                vec![]
            }
        }
    }

    fn match_tailrec_rule<'a>(&self, id: &usize, input: &'a str) -> Vec<&'a str> {
        match self.get(id) {
            Rule::Sequence(rs) => {
                self.match_seq_tailrec(id, rs, input)
            }

            Rule::Alternative(xs, ys) => {
                let r = self.match_seq_tailrec(id, xs, input);
                if !r.is_empty() {
                    r
                } else {
                    self.match_seq_tailrec(id, ys, input)
                }
            }

            _ => panic!("match_tailrec_rule() with rule that cannot be tailrec")
        }
    }

    fn match_rule<'a>(&self, id: &usize, input: &'a str) -> MatchResult<'a> {
        match self.get(id) {
            Rule::MatchChar(c) => {
                if input.chars().next() == Some(*c) {
                    Ok(&input[c.len_utf8()..])
                } else {
                    Err(input)
                }
            }

            Rule::Sequence(rs) => {
                self.match_seq(rs, input)
            }

            Rule::Alternative(xs, ys) => {
                self.match_seq(xs, input)
                    .or_else(|_| self.match_seq(ys, input))
            }
        }
    }

    fn match_all<'a>(&self, input: &'a str) -> Result<(), &'a str> {
        self.match_rule(&0, input)
            .and_then(|remaining| if remaining.is_empty() {
                Ok(())
            } else {
                Err(remaining)
            }
        )
    }

    fn apply_modification(&mut self) {
        // self.rules.insert(8, Rule::OneOrMore(42));
        self.rules.insert(8, Rule::Alternative(vec![42, 8], vec![42]));
        self.rules.insert(11, Rule::Alternative(vec![42, 31], vec!(42, 11, 31)));
    }
}

// --- parser

fn parse_rules(input: &str) -> ParseResult<Rules> {
    let rule_id = integer.map(|i| i as usize);
    let space = match_literal(" ");
    
    let match_char = any_char
        .between(match_literal(" \""), match_literal("\""))
        .map(Rule::MatchChar);

    let raw_sequence = one_or_more(right(space, rule_id.clone())).boxed();
    let sequence = raw_sequence.clone().map(Rule::Sequence);

    let alternative = pair(left(raw_sequence.clone(), match_literal(" |")), raw_sequence,
        |a, b| Rule::Alternative(a, b)
    );

    let rule = pair(
        left(rule_id, match_literal(":")),
        match_char.or(alternative).or(sequence),
        |id, def| (id, def)
    );

    let rules = one_or_more(whitespace_wrap(rule))
        .map(|rs| Rules {
            rules: rs.into_iter().collect()
        });

    rules.parse(input)
}

// -- problems 

fn count_valid_messages(rules: &Rules, messages: &[&str]) -> usize {
    messages.iter().filter_map(|m| rules.match_all(m).ok()).count()
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    
    let mut sections = input.split("\n\n");
    let mut rules = parse_rules(sections.next().unwrap()).unwrap().1;
    let messages: Vec<&str> = sections.next().unwrap().lines().collect();

    println!("part 1 {}", count_valid_messages(&rules, &messages));

    rules.apply_modification();
    println!("part 2 {}", count_valid_messages(&rules, &messages));
}

#[cfg(test)]
#[macro_use] extern crate maplit;

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_rules() -> Rules {
        use Rule::*;
        Rules {
            rules: hashmap![
                0 => Sequence(vec![4, 1, 5]),
                1 => Alternative(vec![2, 3], vec![3, 2]),
                2 => Alternative(vec![4, 4], vec![5, 5]),
                3 => Alternative(vec![4, 5], vec![5, 4]),
                4 => MatchChar('a'),
                5 => MatchChar('b')
            ]
        }
    }

    fn part2_sample_rules() -> Rules {
        parse_rules(
            "42: 9 14 | 10 1
             9: 14 27 | 1 26
             10: 23 14 | 28 1
             1: \"a\"
             11: 42 31
             5: 1 14 | 15 1
             19: 14 1 | 14 14
             12: 24 14 | 19 1
             16: 15 1 | 14 14
             31: 14 17 | 1 13
             6: 14 14 | 1 14
             2: 1 24 | 14 4
             0: 8 11
             13: 14 3 | 1 12
             15: 1 | 14
             17: 14 2 | 1 7
             23: 25 1 | 22 14
             28: 16 1
             4: 1 1
             20: 14 14 | 1 15
             3: 5 14 | 16 1
             27: 1 6 | 14 18
             14: \"b\"
             21: 14 1 | 1 14
             25: 1 1 | 1 14
             22: 14 14
             8: 42
             26: 14 22 | 1 20
             18: 15 15
             7: 14 5 | 1 21
             24: 14 1
").unwrap().1
    }

    fn part2_sample_rules_modified() -> Rules {
        let mut rules = part2_sample_rules();
        rules.apply_modification();
        rules
    }

    fn part2_input() -> impl Iterator<Item = &'static str> {
"abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba".lines()
    }

    #[test]
    fn test_parser() {
        let rules = parse_rules(
            "0: 4 1 5
             1: 2 3 | 3 2
             2: 4 4 | 5 5
             3: 4 5 | 5 4
             4: \"a\"
             5: \"b\""
        );

        assert_eq!(rules, Ok(("", sample_rules())));
    }

    #[test]
    fn test_matcher_success() {
        let rules = sample_rules();
        assert_eq!(rules.match_rule(&0, "ababbb"), Ok(""));
        assert_eq!(rules.match_rule(&0, "abbbab"), Ok(""));
        assert_eq!(rules.match_rule(&0, "aaaabbb"), Ok("b"));
    }

    #[test]
    fn test_matcher_failure() {
        let rules = sample_rules();
        assert_eq!(rules.match_rule(&0, "bababa"), Err("bababa"));
        assert_eq!(rules.match_rule(&0, "aaabbb"), Err("aaabbb"));
    }

    #[test]
    fn test_match_all() {
        let rules = sample_rules();
        assert_eq!(rules.match_all("abbbab"), Ok(()));
        assert_eq!(rules.match_all("aaaabbb"), Err("b"));        
    }

    #[test]
    fn test_part2_rules_without_modification() {
        let rules = part2_sample_rules();
        let messages = part2_input();
        assert_eq!(messages.filter_map(|m| rules.match_all(m).ok()).count(), 3);
    }

    #[test]
    fn test_is_tailrec() {
        assert_eq!(part2_sample_rules_modified().is_tailrec(&8), true);
        assert_eq!(part2_sample_rules_modified().is_tailrec(&11), false);
        assert_eq!(part2_sample_rules_modified().is_tailrec(&3), false);
    }

    #[test]
    fn test_part2_rules_with_modification() {
        let rules = part2_sample_rules_modified();
        let messages = part2_input();
        assert_eq!(messages.filter_map(|m| rules.match_all(m).ok()).count(), 12);
    }

    #[test]
    fn test_part2_rules_with_modification_individual_cases() {
        let rules = part2_sample_rules_modified();
        assert_eq!(rules.match_all("bbabbbbaabaabba"), Ok(()));
        assert_eq!(rules.match_all("babbbbaabbbbbabbbbbbaabaaabaaa"), Ok(()));
        assert_eq!(rules.match_all("aaabbbbbbaaaabaababaabababbabaaabbababababaaa"), Ok(()));
        assert_eq!(rules.match_all("bbbbbbbaaaabbbbaaabbabaaa"), Ok(()));
        assert_eq!(rules.match_all("bbbababbbbaaaaaaaabbababaaababaabab"), Ok(()));
        assert_eq!(rules.match_all("ababaaaaaabaaab"), Ok(()));
        assert_eq!(rules.match_all("ababaaaaabbbaba"), Ok(()));
        assert_eq!(rules.match_all("baabbaaaabbaaaababbaababb"), Ok(()));
        assert_eq!(rules.match_all("abbbbabbbbaaaababbbbbbaaaababb"), Ok(()));
        assert_eq!(rules.match_all("aaaaabbaabaaaaababaa"), Ok(()));
        assert_eq!(rules.match_all("aaaabbaabbaaaaaaabbbabbbaaabbaabaaa"), Ok(()));
        assert_eq!(rules.match_all("aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"), Ok(()));
    }
}