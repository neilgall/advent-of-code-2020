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

fn sum_of_two_is(sum: i64, vec: &[i64]) -> bool {
    vec.iter().enumerate()
        .flat_map(|(i, a)| vec.iter().skip(i+1).map(move |b| a + b))
        .find(|x| *x == sum)
        .is_some()
}

fn find_first_invalid(vec: &Vec<i64>, preamble: usize) -> Option<i64> {
    let end = vec.len() - preamble;
    let index = (preamble..end).find(
        |index| !sum_of_two_is(vec[*index], &vec[index-preamble..*index])
    );

    index.map(|i| vec[i])
}

fn part1(sequence: &Vec<i64>) -> Option<i64> {
    find_first_invalid(sequence, 25)
}

fn part2(sequence: &Vec<i64>) -> i64 {
    0
}


fn main() {
    let input = read_file("./input.txt").unwrap();
    let sequence = parse_input(&input);
    println!("part1 {:?}", part1(&sequence));
    println!("part2 {:?}", part2(&sequence));
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
    fn test_sum_of_two_is() {
        assert!(sum_of_two_is(3, &vec![1,2,3,4,5]));
        assert!(sum_of_two_is(6, &vec![1,2,3,4,5]));
        assert!(sum_of_two_is(9, &vec![1,2,3,4,5]));
        assert!(!sum_of_two_is(20, &vec![9,8,7,6,5]));
    }

    #[test]
    fn test_xmas_decoder() {
        let input = vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576];
        assert_eq!(find_first_invalid(&input, 5), Some(127));
    }
}