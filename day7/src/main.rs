use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

mod parser;
use parser::*;

// --- file read

fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// --- model

// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
// enum Adjective {
//     Bright,
//     Clear,
//     Dark,
//     Dim,
//     Dotted,
//     Drab,
//     Dull,
//     Faded,
//     Light,
//     Mirrored,
//     Muted,
//     Pale,
//     Plaid,
//     Plain,
//     Posh,
//     Shiny,
//     Striped,
//     Vibrant,
//     Wavy
// }

// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
// enum Color {
//     Aqua,
//     Beige,
//     Black,
//     Blue,
//     Bronze,
//     Brown,
//     Chartreuse,
//     Coral,
//     Crimson,
//     Cyan,
//     Fuchsia,
//     Gold,
//     Gray,
//     Green,
//     Indigo,
//     Lavender,
//     Magenta,
//     Maroon,
//     Olive,
//     Orange,
//     Plum,
//     Purple,
//     Red,
//     Salmon,
//     Silver,
//     Tan,
//     Teal,
//     Tomato,
//     Turquoise,
//     Violet,
//     White,
//     Yellow,
// }

type Adjective = String;
type Color = String;

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
        // let adjective =
        //     string("bright").means(Adjective::Bright)
        //     .or(string("clear").means(Adjective::Clear))
        //     .or(string("dark").means(Adjective::Dark))
        //     .or(string("dim").means(Adjective::Dim))
        //     .or(string("dotted").means(Adjective::Dotted))
        //     .or(string("drab").means(Adjective::Drab))
        //     .or(string("dull").means(Adjective::Dull))
        //     .or(string("faded").means(Adjective::Faded))
        //     .or(string("light").means(Adjective::Light))
        //     .or(string("mirrored").means(Adjective::Mirrored))
        //     .or(string("muted").means(Adjective::Muted))
        //     .or(string("pale").means(Adjective::Pale))
        //     .or(string("plaid").means(Adjective::Plaid))
        //     .or(string("plain").means(Adjective::Plain))
        //     .or(string("posh").means(Adjective::Posh))
        //     .or(string("shiny").means(Adjective::Shiny))
        //     .or(string("striped").means(Adjective::Striped))
        //     .or(string("vibrant").means(Adjective::Vibrant))
        //     .or(string("wavy").means(Adjective::Wavy));

        // let color =
        //     string("aqua").means(Color::Aqua)
        //     .or(string("beige").means(Color::Beige))
        //     .or(string("black").means(Color::Black))
        //     .or(string("blue").means(Color::Blue))
        //     .or(string("bronze").means(Color::Bronze))
        //     .or(string("brown").means(Color::Brown))
        //     .or(string("chartreuse").means(Color::Chartreuse))
        //     .or(string("coral").means(Color::Coral))
        //     .or(string("crimson").means(Color::Crimson))
        //     .or(string("cyan").means(Color::Cyan))
        //     .or(string("fuchsia").means(Color::Fuchsia))
        //     .or(string("gold").means(Color::Gold))
        //     .or(string("gray").means(Color::Gray))
        //     .or(string("green").means(Color::Green))
        //     .or(string("indigo").means(Color::Indigo))
        //     .or(string("lavender").means(Color::Lavender))
        //     .or(string("magenta").means(Color::Magenta))
        //     .or(string("maroon").means(Color::Maroon))
        //     .or(string("olive").means(Color::Olive))
        //     .or(string("orange").means(Color::Orange))
        //     .or(string("plum").means(Color::Plum))
        //     .or(string("purple").means(Color::Purple))
        //     .or(string("red").means(Color::Red))
        //     .or(string("salmon").means(Color::Salmon))
        //     .or(string("silver").means(Color::Silver))
        //     .or(string("tan").means(Color::Tan))
        //     .or(string("teal").means(Color::Teal))
        //     .or(string("tomato").means(Color::Tomato))
        //     .or(string("turquoise").means(Color::Turquoise))
        //     .or(string("violet").means(Color::Violet))
        //     .or(string("white").means(Color::White))
        //     .or(string("yellow").means(Color::Yellow));

        let adjective = one_or_more(letter).map(|ls| ls.into_iter().collect());
        let color = one_or_more(letter).map(|ls| ls.into_iter().collect());

        pair(first(adjective, whitespace), color).map(|(a, c)| BagColor(a, c))
    }

    fn container<'b>() -> impl Parser<'b, BagColor> {
        first(bag_color(), string(" bags contain "))
    }

    let bag_or_bags = string(" bags, ").or(string(" bag, ")).or(string(" bags.")).or(string(" bag."));
    let contained = pair(first(integer, whitespace), first(bag_color(), bag_or_bags));

    let contents_rule = pair(container(), one_or_more(contained)).map(|(color, contents)| 
        ContainsRule {
            container: color.clone(),
            contents: contents.iter().map(|(n, c)| Content {
                color: c.clone(),
                count: *n as usize
            }).collect()
        }
    );

    let no_contents_rule = first(container(), string("no other bags.")).map(|color| 
        ContainsRule {
            container: color,
            contents: vec![]
        }
    );
    
    contents_rule.or(no_contents_rule)
}

fn parse_input(input: &str) -> RuleSet {
    let rules: Vec<ContainsRule> = input.lines().map(|line| parse_rule().parse(line).unwrap().1).collect();
    RuleSet { 
        rules: rules.into_iter().map(|r| (r.container, r.contents)).collect()
    }

    // let rule_set = one_or_more(first(parse_rule(), pair(whitespace, string("\n"))));

    // rule_set.parse(input).map(|(rest, rules)| {
    //     let rule_set = RuleSet { rules: rules.into_iter().flat_map(|rs| rs.into_iter()).collect() };
    //     (rest, rule_set)
    // })
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

    // --- problems 

    fn part1(&self) -> usize {
        self.rules.keys()
            .filter(|color| self.can_contain_indirectly(&color, &BagColor::of("shiny", "gold")))
            .count()
    }

    fn part2(&self) -> usize {
        0
    }
}

fn main() {
    let input = read_file("./input.txt").unwrap();
    let rules: RuleSet = parse_input(&input);

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
}
