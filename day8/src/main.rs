use std::collections::HashSet;
use parser::*;

// --- model

#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
enum Termination {
    InfiniteLoop,
    EndOfProgram
}

impl Machine {
    fn new() -> Self {
        Machine {
            instr_ptr: 0,
            accumulator: 0
        }
    }

    fn run(&mut self, program: &Program) -> Termination {
        let mut visited = HashSet::new();

        loop {
            if self.instr_ptr >= program.len() {
                return Termination::EndOfProgram;
            }
            if !visited.insert(self.instr_ptr) {
                return Termination::InfiniteLoop
            }

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
    let sign = either(
        any_char.pred(|c| *c == '+').means(1),
        any_char.pred(|c| *c == '-').means(-1)
    );
    let signed_integer = pair(sign, integer, |s, i| s * i).boxed();

    let acc = right(match_literal("acc "), signed_integer.clone()).map(Instruction::Acc);
    let jmp = right(match_literal("jmp "), signed_integer.clone()).map(Instruction::Jmp);
    let nop = right(match_literal("nop "), signed_integer).map(Instruction::Nop);
    let instruction = whitespace_wrap(one_of3(acc, jmp, nop));

    zero_or_more(instruction).parse(input)
}


// --- problems

fn part1(program: &Program) -> i64 {
    let mut machine = Machine::new();
    machine.run(program);
    machine.accumulator
}

fn part2(program: &Program) -> Option<i64> {
    fn is_jmp(i: &Instruction) -> bool {
        if let Instruction::Jmp(_) = i { true } else { false }
    }

    program.iter()
        .enumerate()
        .filter(|(_, instr)| is_jmp(instr))
        .find_map(|(index, _)| {
            let mut modified: Program = program.to_vec();
            modified[index] = Instruction::Nop(0);

            let mut machine = Machine::new();
            if machine.run(&modified) == Termination::EndOfProgram {
                Some(machine.accumulator)
            } else {
                None
            }
        })
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let program: Program = parse_input(&input).unwrap().1;

    println!("part1 {:?}", part1(&program));
    println!("part2 {:?}", part2(&program));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_program() -> Program {
        vec![
            Instruction::Nop(0),
            Instruction::Acc(1),
            Instruction::Jmp(4),
            Instruction::Acc(3),
            Instruction::Jmp(-3),
            Instruction::Acc(-99),
            Instruction::Acc(1),
            Instruction::Jmp(-4),
            Instruction::Acc(6)
        ]
    }

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

        assert_eq!(instructions, Ok(("", test_program())));
    }

    #[test]
    fn test_running_until_instruction_visited_twice() {
        let mut machine = Machine::new();
        let term = machine.run(&test_program());

        assert_eq!(term, Termination::InfiniteLoop);
        assert_eq!(machine.accumulator, 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&test_program()), Some(8));        
    }

}