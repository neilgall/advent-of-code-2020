
use std::collections::HashMap;
use std::ops::RangeInclusive;
use parser::*;

// --- model

#[derive(Debug, Eq, PartialEq)]
struct Ranges(Vec<RangeInclusive<i64>>);

type FieldRanges = HashMap<String, Ranges>;
type Ticket = Vec<i64>;

#[derive(Debug, Eq, PartialEq)]
struct TicketData {
    field_ranges: FieldRanges,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>
}

fn parse_input(input: &str) -> ParseResult<TicketData> {
    let range = pair(
        left(integer, match_literal("-")),
        integer,
        |min, max| (min..=max)
    );

    let ranges = range
        .sep_by(whitespace_wrap(match_literal("or")))
        .map(|rs| Ranges(rs));

    let field_name = one_or_more(any_char.pred(|c| *c != ':'))
        .map(|cs| cs.iter().collect());

    let field_range = tuple2(
        left(field_name, match_literal(":")),
        whitespace_wrap(ranges)
    );

    let csv = integer.sep_by(match_literal(","));

    let your_ticket = right(
        whitespace_wrap(match_literal("your ticket:")),
        csv.clone()
    );

    let nearby_tickets = right(
        whitespace_wrap(match_literal("nearby tickets:")),
        one_or_more(whitespace_wrap(csv))
    );

    let ticket_data = tuple3(one_or_more(field_range), your_ticket, nearby_tickets)
        .map(|(field_ranges, your_ticket, nearby_tickets)| TicketData {
            field_ranges: field_ranges.into_iter().collect(),
            your_ticket,
            nearby_tickets
        });

    ticket_data.parse(input)
}

// --- problems

fn part1(ticket_data: &TicketData) -> usize {
    0
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let ticket_data = parse_input(&input).unwrap().1;

    println!("part 1 {:?}", part1(&ticket_data));
}


#[cfg(test)]
#[macro_use] extern crate maplit;

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> TicketData {
        TicketData {
            field_ranges: hashmap![
                "class".to_string() => Ranges(vec![1..=3, 5..=7]),
                "row".to_string() => Ranges(vec![6..=11, 33..=44]),
                "seat".to_string() => Ranges(vec![13..=40, 45..=50])
            ],
            your_ticket: vec![7, 1, 14],
            nearby_tickets: vec![
                vec![7 ,3, 47],
                vec![40, 4, 50],
                vec![55, 2, 20],
                vec![38, 6, 12]
            ]
        }
    }

    #[test]
    fn test_parser() {
        let ticket_data = parse_input(
            "class: 1-3 or 5-7
             row: 6-11 or 33-44
             seat: 13-40 or 45-50

             your ticket:
             7,1,14

             nearby tickets:
             7,3,47
             40,4,50
             55,2,20
             38,6,12"
        );

        assert_eq!(ticket_data, Ok(("", sample_data())));
    }
}