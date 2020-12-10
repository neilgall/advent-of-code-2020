use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;


// --- file read

fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_input(input: &str) -> Vec<i64> {
    input.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect()
}

// --- problems

fn adapter_order(adapters: &Vec<i64>) -> Vec<i64> {
    let mut ordered = adapters.to_vec();
    ordered.push(0);
    ordered.push(*adapters.iter().max().unwrap() + 3);
    ordered.sort();
    ordered
}

fn distribution_of_diffs(sequence: &[i64]) -> HashMap<i64, usize> {
    let mut dist = HashMap::new();
    let mut prev = 0;
    for curr in sequence.iter() {
        let diff = curr - prev;
        prev = *curr;
        let count = dist.get(&diff).unwrap_or(&0) + 1;
        dist.insert(diff, count);
    }
    dist
}

fn part1(adapters: &Vec<i64>) -> Option<usize> {
    let dist = distribution_of_diffs(&adapter_order(adapters));
    dist.get(&1).and_then(|ones|
        dist.get(&3).map(|threes| ones * threes)
    )
}

fn part2(adapters: &Vec<i64>) -> Option<i64> {
    None
}   


fn main() {
    let input = read_file("./input.txt").unwrap();
    let adapters = parse_input(&input);
    println!("part1 {:?}", part1(&adapters));
    println!("part2 {:?}", part2(&adapters));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parse_input("1 2 3 4"), vec![1, 2, 3, 4]);
        assert_eq!(parse_input("15\n16\n0\n99"), vec![15, 16, 0, 99]);
    }

    #[test]
    fn test_adapter_order() {
        let adapters = vec![16,10,15,5,1,11,7,19,6,12,4];
        let ordered = adapter_order(&adapters);
        assert_eq!(ordered, vec![0,1,4,5,6,7,10,11,12,15,16,19,22]);
    }

    #[test]
    fn test_distribution_of_diffs() {
        let sequence = vec![1,4,5,6,7,10,11,12,15,16,19,22];
        let distribution = distribution_of_diffs(&sequence);
        assert_eq!(distribution.len(), 2);
        assert_eq!(distribution.get(&1), Some(&7));
        assert_eq!(distribution.get(&3), Some(&5));
    }

    #[test]
    fn test_part1_example_1() {
        let adapters = vec![16,10,15,5,1,11,7,19,6,12,4];
        assert_eq!(part1(&adapters), Some(35));
    }

    #[test]
    fn test_part2_example_2() {
        let adapters = vec![28,33,18,42,31,14,46,20,48,47,24,23,49,45,19,38,39,11,1,32,25,35,8,17,7,9,4,2,34,10,3];
        assert_eq!(part1(&adapters), Some(220));
    }
}
