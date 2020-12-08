use std::collections::HashSet;
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

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64)
}

type Program = Vec<Instruction>;

#[derive(Debug)]
struct Machine {
    instr_ptr: usize,
    accumulator: i64
}

impl Machine {
    fn new() -> Self {
        Machine {
            instr_ptr: 0,
            accumulator: 0
        }
    }

    fn run_until_instruction_visited_twice(&mut self, program: &Program) {
        let mut visited = HashSet::new();

        while !visited.contains(&self.instr_ptr) {
            visited.insert(self.instr_ptr);

            match program[self.instr_ptr] {
                Instruction::Acc(arg) => {
                    self.accumulator += arg;
                    self.instr_ptr += 1;
                }
                Instruction::Jmp(arg) => {
                    self.instr_ptr = ((self.instr_ptr as i64) + arg) as usize;
                }
                Instruction::Nop(_) => {
                    self.instr_ptr += 1;
                }
            }
        }
    }
}

// --- parser

fn parse_input(input: &str) -> ParseResult<Program> {
    fn signed_integer<'a>() -> impl Parser<'a, i64> {
        let sign = either(
            any_char.pred(|c| *c == '+').means(1),
            any_char.pred(|c| *c == '-').means(-1)
        );
        pair(sign, integer).map(|(s, i)| s * i)
    }

    let acc = right(match_literal("acc "), signed_integer()).map(Instruction::Acc);
    let jmp = right(match_literal("jmp "), signed_integer()).map(Instruction::Jmp);
    let nop = right(match_literal("nop "), signed_integer()).map(Instruction::Nop);
    let instruction = whitespace_wrap(either(either(acc, jmp), nop));

    zero_or_more(instruction).parse(input)
}


// --- problems

fn part1(program: &Program) -> i64 {
    let mut machine = Machine::new();
    machine.run_until_instruction_visited_twice(program);
    machine.accumulator
}

fn part2(program: &Program) -> i64 {
    0
}

fn main() {
    let input = read_file("./input.txt").unwrap();
    let program: Program = parse_input(&input).unwrap().1;

    println!("part1 {}", part1(&program));
    println!("part2 {}", part2(&program));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instructions() {
        let sample = "
            nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6
        ";
        let instructions = parse_input(sample);

        assert_eq!(instructions, Ok(("", vec![
            Instruction::Nop(0),
            Instruction::Acc(1),
            Instruction::Jmp(4),
            Instruction::Acc(3),
            Instruction::Jmp(-3),
            Instruction::Acc(-99),
            Instruction::Acc(1),
            Instruction::Jmp(-4),
            Instruction::Acc(6)
        ])));
    }

    #[test]
    fn test_running_until_instruction_visited_twice() {
        let program = vec![
            Instruction::Nop(0),
            Instruction::Acc(1),
            Instruction::Jmp(4),
            Instruction::Acc(3),
            Instruction::Jmp(-3),
            Instruction::Acc(-99),
            Instruction::Acc(1),
            Instruction::Jmp(-4),
            Instruction::Acc(6)
        ];
        let mut machine = Machine::new();
        machine.run_until_instruction_visited_twice(&program);

        assert_eq!(machine.accumulator, 5);
    }

}