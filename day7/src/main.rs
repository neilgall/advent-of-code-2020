use std::collections::HashMap;
use parser::*;

// --- model

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct BagColor(String, String);

impl BagColor {
    fn of(adj: &str, col: &str) -> Self {
        BagColor(String::from(adj), String::from(col))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Content {
    color: BagColor,
    count: usize
}

#[derive(Debug, Eq, PartialEq)]
struct ContainsRule {
    container: BagColor,
    contents: Vec<Content>
}

#[derive(Debug)]
struct RuleSet {
    rules: HashMap<BagColor, Vec<Content>>
}

fn parse_rule<'a>() -> impl Parser<'a, ContainsRule> {
    fn bag_color<'b>() -> impl Parser<'b, BagColor> {
        let letter = any_char.pred(|c| c.is_alphabetic());
        let adjective = one_or_more(letter.clone()).map(|ls| ls.into_iter().collect());
        let color = one_or_more(letter).map(|ls| ls.into_iter().collect());

        pair(whitespace_wrap(adjective), color, |a, c| BagColor(a, c))
    }

    fn container<'b>() -> impl Parser<'b, BagColor> {
        left(bag_color(), match_literal(" bags contain "))
    }

    let bag_or_bags = match_literal(" bags, ").or(match_literal(" bag, ")).or(match_literal(" bags.")).or(match_literal(" bag."));
    let contained = pair(whitespace_wrap(integer), left(bag_color(), bag_or_bags), |n, c| (n, c));

    let contents_rule = pair(container(), one_or_more(contained), |color, contents| 
        ContainsRule {
            container: color.clone(),
            contents: contents.iter().map(|(n, c)| Content {
                color: c.clone(),
                count: *n as usize
            }).collect()
        }
    );

    let no_contents_rule = left(container(), match_literal("no other bags.")).map(|color| 
        ContainsRule {
            container: color,
            contents: vec![]
        }
    );
    
    contents_rule.or(no_contents_rule)
}

fn parse_input(input: &str) -> ParseResult<RuleSet> {
    let rule_set = one_or_more(whitespace_wrap(parse_rule()));

    rule_set.parse(input).map(|(rest, rules)| {
        let rule_set = RuleSet { 
            rules: rules.into_iter().map(|r| (r.container, r.contents)).collect()
        };
        (rest, rule_set)
    })
}

impl RuleSet {
    fn can_contain(&self, from: &BagColor, to: &BagColor) -> bool {
        self.rules.get(from)
            .map(|contents| 
                contents.iter().any(|c| &c.color == to))
            .unwrap_or(false)
    }

    fn can_contain_indirectly(&self, from: &BagColor, to: &BagColor) -> bool {
        self.can_contain(from, to) 
            || self.rules.get(from).map(|contents|
                contents.iter().any(|c| self.can_contain_indirectly(&c.color, to)))
                .unwrap_or(false)
    }

    fn number_of_contained_bags(&self, from: &BagColor) -> usize {
        self.rules.get(from)
            .map(|contents| contents.iter()
                .map(|c| c.count * (1 + self.number_of_contained_bags(&c.color)))
                .sum())
            .unwrap_or(0)
    }

    // --- problems 

    fn part1(&self) -> usize {
        self.rules.keys()
            .filter(|color| self.can_contain_indirectly(&color, &BagColor::of("shiny", "gold")))
            .count()
    }

    fn part2(&self) -> usize {
        self.number_of_contained_bags(&BagColor::of("shiny", "gold"))
    }
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let rules: RuleSet = parse_input(&input).unwrap().1;

    println!("part1 {}", rules.part1());
    println!("part2 {}", rules.part2());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_with_single_clause() {
        assert_eq!(
            parse_rule().parse("light red bags contain 1 bright white bag."),
            Ok(("", ContainsRule {
                container: BagColor::of("light", "red"),
                contents: vec![
                    Content { color: BagColor::of("bright", "white"), count: 1 }
                ]
            }))
        );
    }

    #[test]
    fn test_parse_with_two_clauses() {
        assert_eq!(
            parse_rule().parse("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            Ok(("", ContainsRule { 
                container: BagColor::of("light", "red"),
                contents: vec![
                    Content { color: BagColor::of("bright", "white"), count: 1 },
                    Content { color: BagColor::of("muted", "yellow"), count: 2 }
                ]
            }))
        );
    }

    #[test]
    fn test_parse_with_many_clauses() {
        assert_eq!(
            parse_rule().parse("dotted silver bags contain 2 dotted orange bags, 3 bright fuchsia bags, 5 bright tomato bags, 3 faded turquoise bags."),
            Ok(("", ContainsRule {
                container: BagColor::of("dotted", "silver"),
                contents: vec![
                    Content { color: BagColor::of("dotted", "orange"), count: 2 },
                    Content { color: BagColor::of("bright", "fuchsia"), count: 3 },
                    Content { color: BagColor::of("bright", "tomato"), count: 5 },
                    Content { color: BagColor::of("faded", "turquoise"), count: 3 }
                ]
            }))
        );
    }

    #[test]
    fn test_parse_with_no_contents() {
        assert_eq!(
            parse_rule().parse("faded blue bags contain no other bags."),
            Ok(("", ContainsRule {
                container: BagColor::of("faded", "blue"),
                contents: vec![]
            }))
        );
    }

    #[test]
    fn test_parse_records_separated_by_lines() {
        let p = one_or_more(whitespace_wrap(any_char));
        assert_eq!(p.parse("a\nb\nc\n"), Ok(("", vec!['a', 'b', 'c'])));
    }
}
