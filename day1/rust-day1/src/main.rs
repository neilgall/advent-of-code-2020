
use std::fs::File;
use std::io::prelude::*;


fn read_file(filename: &str) -> std::io::Result<Vec<i64>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let input = contents.lines().filter_map(|line| line.parse().ok()).collect();
    Ok(input)
}

fn part1(input: &Vec<i64>) -> i64 {
    input.iter().flat_map(
        move |x| input.iter().filter_map(
            move |y|
                if x+y == 2020 { Some(x*y) } else { None }
        )
    ).next().unwrap()
}

fn part2(input: &Vec<i64>) -> i64 {
    input.iter().flat_map(
        move |x| input.iter().flat_map(
            move |y| input.iter().filter_map(
                move |z| 
                    if x+y+z == 2020 { Some(x*y*z) } else { None }
            )
        )
    ).next().unwrap()

}


fn main() {
    let input = read_file("../input.txt").unwrap();
    println!("Part1 {}", part1(&input));
    println!("Part2 {}", part2(&input));
}
