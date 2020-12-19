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
    fn get(&self, id: usize) -> &Rule {
        self.rules.get(&id).unwrap()
    }

    fn match_seq<'a>(&self, seq: &[usize], input: &'a str) -> MatchResult<'a> {
        seq.iter().try_fold(input, 
            |next_input, rule_id| self.match_rule(*rule_id, next_input)
        )
    }

    fn match_rule<'a>(&self, id: usize, input: &'a str) -> MatchResult<'a> {
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
        self.match_rule(0, input)
            .and_then(|remaining| if remaining.is_empty() {
                Ok(())
            } else {
                Err(remaining)
            }
        )
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

fn part1(rules: &Rules, messages: &[&str]) -> usize {
    messages.iter().filter_map(|m| rules.match_all(m).ok()).count()
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    
    let mut sections = input.split("\n\n");
    let rules = parse_rules(sections.next().unwrap()).unwrap().1;
    let messages: Vec<&str> = sections.next().unwrap().lines().collect();

    println!("part 1 {}", part1(&rules, &messages));
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
        assert_eq!(rules.match_rule(0, "ababbb"), Ok(""));
        assert_eq!(rules.match_rule(0, "abbbab"), Ok(""));
        assert_eq!(rules.match_rule(0, "aaaabbb"), Ok("b"));
    }

    #[test]
    fn test_matcher_failure() {
        let rules = sample_rules();
        assert_eq!(rules.match_rule(0, "bababa"), Err("bababa"));
        assert_eq!(rules.match_rule(0, "aaabbb"), Err("aabbb"));
    }

    #[test]
    fn test_match_all() {
        let rules = sample_rules();
        assert_eq!(rules.match_all("abbbab"), Ok(()));
        assert_eq!(rules.match_all("aaaabbb"), Err("b"));        
    }
}