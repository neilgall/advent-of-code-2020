
// --- model

#[derive(Debug, Eq, PartialEq)]
struct BoardingPass {
    row: usize,
    column: usize
}

impl BoardingPass {
    fn seat_id(&self) -> usize {
        self.row * 8 + self.column
    }
}

fn decode(s: &str, one: char) -> usize {
    s.chars().fold(0, |r, c| (r << 1) | (if c == one { 1 } else { 0 }))
}

impl From<&str> for BoardingPass {
    fn from(s: &str) -> BoardingPass {
        let row = decode(&s[0..7], 'B');
        let column = decode(&s[7..10], 'R');
        BoardingPass { row, column }
    }
}

// --- problems

fn part1(passes: &Vec<BoardingPass>) -> Option<usize> {
    passes.iter().map(|bp| bp.seat_id()).max()
}

fn part2(passes: &Vec<BoardingPass>) -> Option<usize> {
    let seat_ids: Vec<usize> = passes.iter().map(|bp| bp.seat_id()).collect();

    seat_ids.iter().max().and_then(|max_id| {
        (1..=*max_id).find(|id_ref| {
            let id = *id_ref;
            !seat_ids.contains(&id) && seat_ids.contains(&(id-1)) && seat_ids.contains(&(id+1))
        })
    })
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let passes: Vec<BoardingPass> = input.lines().map(|line| line.into()).collect();

    println!("part1 {}", part1(&passes).unwrap());
    println!("part2 {}", part2(&passes).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deocde() {
        assert_eq!(decode("BFFFBBF", 'B'), 70);
        assert_eq!(decode("RRR", 'R'), 7);
        assert_eq!(decode("FFFBBBF", 'B'), 14);
        assert_eq!(decode("BBFFBBF", 'B'), 102);
    }

    #[test]
    fn test_to_baording_pass() {
        assert_eq!(BoardingPass::from("BFFFBBFRRR"), BoardingPass { row: 70, column: 7 });
        assert_eq!(BoardingPass::from("FFFBBBFRRR"), BoardingPass { row: 14, column: 7 });
        assert_eq!(BoardingPass::from("BBFFBBFRLL"), BoardingPass { row: 102, column: 4 });
    }
}