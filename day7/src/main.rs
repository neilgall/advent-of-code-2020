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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum BagColor {
    LighRed,
    BrightWhite,
    MutedYellow,
    DarkOrange,
    ShinyGold,
    FadedBlue,
    DarkOlive,
    VibrantPlum,
    DottedBlack
}

#[derive(Debug, Eq, PartialEq)]
struct ContainsRule {
    container_color: BagColor,
    contained_color: BagColor,
    contain_count: usize
}

fn parse_rule<'a>() -> impl Parser<'a, Vec<ContainsRule>> {
    fn bag_color<'b>() -> impl Parser<'b, BagColor> {
        string("light red").means(BagColor::LighRed)
        .or(string("bright white").means(BagColor::BrightWhite))
        .or(string("muted yellow").means(BagColor::MutedYellow))
        .or(string("dark orange").means(BagColor::DarkOrange))
        .or(string("shiny gold").means(BagColor::ShinyGold))
        .or(string("faded blue").means(BagColor::FadedBlue))
        .or(string("dark olive").means(BagColor::DarkOlive))
        .or(string("vibrant plum").means(BagColor::VibrantPlum))
        .or(string("dotted black").means(BagColor::DottedBlack))
    }

    fn container<'b>() -> impl Parser<'b, BagColor> {
        first(bag_color(), string(" bags contain "))
    }

    let bag_or_bags = string(" bags, ").or(string(" bag, ")).or(string(" bags.")).or(string(" bag."));
    let contained = pair(first(integer, whitespace), first(bag_color(), bag_or_bags));

    let contents_rule = pair(container(), one_or_more(contained)).map(|(color, contents)| 
        contents.iter().map(|(n, c)| ContainsRule {
            container_color: color,
            contained_color: *c,
            contain_count: *n as usize
        }).collect()
    );

    let no_contents_rule = pair(container(), string("no other bags.")).map(|_| vec![]);
    
    contents_rule.or(no_contents_rule)
}

fn parse_input(input: &str) -> ParseResult<Vec<ContainsRule>> {
    one_or_more(parse_rule()).parse(input).map(|(rest, rules)|
        (rest, rules.into_iter().flat_map(|rs| rs.into_iter()).collect())
    )
}


// --- problems 

fn part1(groups: &Vec<ContainsRule>) -> usize {
    0
}

fn part2(groups: &Vec<ContainsRule>) -> usize {
    0
}

fn main() {
    let input = read_file("./input.txt").unwrap();
    let rules: Vec<ContainsRule> = parse_input(&input).unwrap().1;

    println!("part1 {}", part1(&rules));
    println!("part2 {}", part2(&rules));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_with_single_clause() {
        assert_eq!(
            parse_rule().parse("light red bags contain 1 bright white bag."),
            Ok(("", vec![
                ContainsRule { 
                    container_color: BagColor::LighRed,
                    contained_color: BagColor::BrightWhite,
                    contain_count: 1
                }
            ]))
        );
    }

    #[test]
    fn test_parse_with_two_clauses() {
        assert_eq!(
            parse_rule().parse("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            Ok(("", vec![
                ContainsRule { 
                    container_color: BagColor::LighRed,
                    contained_color: BagColor::BrightWhite,
                    contain_count: 1
                },
                ContainsRule { 
                    container_color: BagColor::LighRed,
                    contained_color: BagColor::MutedYellow,
                    contain_count: 2
                }
            ]))
        );
    }

    #[test]
    fn test_parse_with_no_contents() {
        assert_eq!(
            parse_rule().parse("faded blue bags contain no other bags."),
            Ok(("", vec![]))
        );
    }
}
