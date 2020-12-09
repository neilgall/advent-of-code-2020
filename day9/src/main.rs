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
    let index = (preamble..vec.len()).find(
        |index| !sum_of_two_is(vec[*index], &vec[index-preamble..*index])
    );

    index.map(|i| vec[i])
}

fn find_contiguous_set_summing_to(target: i64, vec: &[i64]) -> Option<&[i64]> {
    let mut start = 0;
    let mut end = 1;
    let mut sum: i64 = vec[start..=end].iter().sum();

    while end < vec.len() {
        if sum == target {
            return Some(&vec[start..=end]);
        }
        if sum > target {
            sum -= vec[start];
            start += 1;
        } else {
            end += 1;
            sum += vec[end];
        }
    }

    None
}

fn sum_of_min_and_max(vec: &[i64]) -> Option<i64> {
    vec.iter().min()
        .and_then(|min|
            vec.iter().max().map(|max| 
                min + max
            )
        )
}

fn part1(sequence: &Vec<i64>) -> Option<i64> {
    find_first_invalid(sequence, 25)
}

fn part2(sequence: &Vec<i64>) -> Option<i64> {
    let target = part1(sequence).unwrap();
    find_contiguous_set_summing_to(target, sequence)
        .and_then(sum_of_min_and_max)
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
    fn test_find_first_invalid() {
        let input = vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576];
        assert_eq!(find_first_invalid(&input, 5), Some(127));
    }

    #[test]
    fn test_find_first_invalid_when_close_to_end() {
        let input = vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219];
        assert_eq!(find_first_invalid(&input, 5), Some(127));
    }

    #[test]
    fn test_find_contiguous_set_summing_to() {
        let input = vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576];
        let expect: &[i64] = &vec![15,25,47,40];
        assert_eq!(find_contiguous_set_summing_to(127, &input), Some(expect));     
    }

    #[test]
    fn test_sum_of_min_and_max() {
        let input = vec![15,25,47,40];
        assert_eq!(sum_of_min_and_max(&input), Some(62));
    }
}
