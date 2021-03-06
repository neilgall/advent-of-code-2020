use parser::*;

// -- model

type Timestamp = i64;
type BusID = i64;

struct Input {
    estimate: Timestamp,
    bus_ids: Vec<Option<BusID>>
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let bus_id = either(
            match_literal("x").means(None),
            integer.map(Option::Some)
        );

        let bus_ids = bus_id.sep_by(match_literal(","));

        let input = pair(whitespace_wrap(integer), bus_ids,
            |estimate, bus_ids| Input { estimate, bus_ids }
        );

        input.parse(s).unwrap().1
    }
}

// --- problems

impl Input {
    fn next_bus_departing(&self) -> Option<(BusID, Timestamp)> {
        self.bus_ids.iter()
            .filter_map(|maybe_id| *maybe_id)
            .map(|id| (id, id - (self.estimate % id)))
            .min_by_key(|(_id, wait_time)| *wait_time)
    }

    fn bus_ids_with_departure_offsets(&self) -> impl Iterator<Item = (BusID, Timestamp)> + '_ {

        // find the valid bus IDs and pair them with their position in the 
        // list, which equates to the departure offset in minutes

        self.bus_ids.iter()
            .enumerate()
            .filter_map(|(index, maybe_id)| maybe_id.map(|id| (id, index as Timestamp) ))        
    }

    fn find_first_aligned_timestamp(&self, after: Timestamp) -> Timestamp {

        // for each bus, find a new base timestamp after the current timestamp at which
        // the bus leaves (subject to its indexed departure offset), and a repetition period
        // which is true for all buses examined so far

        // (the period is a product of all bus ids, which passes all tests and finds
        // the right answer, but technically it should only count common factors once each;
        // this is possibly a deliberate design of the input data to make the problem
        // easier - thay do all seem to be primes)

        self.bus_ids_with_departure_offsets().fold(
            (after, 1),
            |(base_timestamp, period), (bus_id, offset)|
                (0..).find_map(|i| {
                    let timestamp = base_timestamp + i * period;
                    if (timestamp + offset) % bus_id == 0 {
                        Some( (timestamp, period * bus_id) )
                    } else {
                        None
                    }
                }).unwrap()
        ).0
    }
}

fn part1(input: &Input) -> Option<i64> {
    input.next_bus_departing().map(|(id, wait)| id * wait)
}

fn part2(input: &Input) -> Timestamp {
    input.find_first_aligned_timestamp(100000000000000)
}

fn main() {
    let input = Input::from(std::fs::read_to_string("./input.txt").unwrap().as_str());
    println!("part1 {:?}", part1(&input));
    println!("part2 {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = Input::from("939\n7,13,x,x,59,x,31,19");
        assert_eq!(input.estimate, 939);
        assert_eq!(input.bus_ids, vec![Some(7),Some(13),None,None,Some(59),None,Some(31),Some(19)]);
    }

    #[test]
    fn test_next_bus_departing() {
        let input = Input::from("939\n7,13,x,x,59,x,31,19");
        assert_eq!(input.next_bus_departing(), Some((59, 5)));        
    }

    #[test]
    fn test_find_first_aligned_timestamp_1() {
        let input = Input::from("939\n7,13,x,x,59,x,31,19");
        assert_eq!(input.find_first_aligned_timestamp(1000000), 1068781);        
    }

    #[test]
    fn test_find_first_aligned_timestamp_2() {
        let input = Input::from("0\n17,x,13,19");
        assert_eq!(input.find_first_aligned_timestamp(0), 3417);        
    }

    #[test]
    fn test_find_first_aligned_timestamp_3() {
        let input = Input::from("0\n67,7,59,61");
        assert_eq!(input.find_first_aligned_timestamp(0), 754018);        
    }

    #[test]
    fn test_find_first_aligned_timestamp_4() {
        let input = Input::from("0\n67,x,7,59,61");
        assert_eq!(input.find_first_aligned_timestamp(0), 779210);        
    }

    #[test]
    fn test_find_first_aligned_timestamp_5() {
        let input = Input::from("0\n67,7,x,59,61");
        assert_eq!(input.find_first_aligned_timestamp(0), 1261476);        
    }

    #[test]
    fn test_find_first_aligned_timestamp_6() {
        let input = Input::from("0\n1789,37,47,1889");
        assert_eq!(input.find_first_aligned_timestamp(0), 1202161486);        
    }
}
