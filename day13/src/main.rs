use std::fs::File;
use std::io::prelude::*;
use parser::*;

// --- file read

fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// -- model

type Timestamp = i64;
type BusID = i64;

struct Input {
    estimate: Timestamp,
    in_service_buses: Vec<BusID>
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let bus_id = either(
            match_literal("x").means(None),
            integer.map(Option::Some)
        );

        let valid_bus_ids = bus_id.sep_by(match_literal(","))
            .map(|ids| ids.iter().filter_map(|id| *id).collect());


        let input = pair(whitespace_wrap(integer), valid_bus_ids,
            |estimate, in_service_buses| Input { estimate, in_service_buses }
        );

        input.parse(s).unwrap().1
    }
}

// --- problems

fn next_bus_departing(input: &Input) -> Option<(BusID, Timestamp)> {
    let buses_with_wait_times: Vec<(BusID, Timestamp)> = input.in_service_buses.iter()
        .map(|id| (*id, id - (input.estimate % id)))
        .collect();

    buses_with_wait_times.iter().min_by_key(|(_id, wait_time)| wait_time)
        .map(|r| r.clone())
}

fn part1(input: &Input) -> Option<i64> {
    next_bus_departing(input).map(|(id, wait)| id * wait)
}


fn main() {
    let input = Input::from(read_file("./input.txt").unwrap().as_str());
    println!("part1 {:?}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = Input::from("939\n7,13,x,x,59,x,31,19");
        assert_eq!(input.estimate, 939);
        assert_eq!(input.in_service_buses, vec![7,13,59,31,19]);
    }

    #[test]
    fn test_next_bus_departing() {
        let input = Input::from("939\n7,13,x,x,59,x,31,19");
        assert_eq!(next_bus_departing(&input), Some((59, 5)));        
    }
}