use std::collections::HashMap;
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

type Address = u64;
type Word = u64;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct WriteMask {
    zeros: Word,
    ones: Word
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Mask(WriteMask),
    Write { address: Address, value: Word }
}

type Program = Vec<Instruction>;

struct Machine {
    memory: HashMap<Address, Word>,
    mask: WriteMask
}

impl Machine {
    fn new() -> Self {
        Machine { 
            memory: HashMap::new(),
            mask: WriteMask { zeros: 0, ones: 0 }
        }
    }

    fn run(&mut self, program: &Program) {
        program.iter().for_each(|instruction| match instruction {

            Instruction::Mask(mask) => {
                self.mask = *mask;
            }

            Instruction::Write { address, value } => {
                self.memory.insert(*address, value & (!self.mask.zeros) | self.mask.ones);
            }
        });
    }

    fn sum_of_all_memory_words(&self) -> Word {
        self.memory.values().sum()
    }
}


// -- parser

fn parse_input(input: &str) -> ParseResult<Program> {
    #[derive(Copy,Clone)]
    enum MaskBit {
        Zero,
        One,
        Unchanged
    }

    let mask_bit = match_literal("X").means(MaskBit::Unchanged)
        .or(match_literal("0").means(MaskBit::Zero))
        .or(match_literal("1").means(MaskBit::One));

    let mask = right(match_literal("mask = "), one_or_more(mask_bit))
        .map(|bits| {
            let (zeros, ones) = bits.iter().rev().enumerate().fold(
                (0, 0), 
                |(zeros, ones), (bit_index, mask_bit)| match mask_bit {
                    MaskBit::Zero => (zeros | 1 << bit_index, ones),
                    MaskBit::One => (zeros, ones | 1 << bit_index),
                    MaskBit::Unchanged => (zeros, ones),
                }
            );
            Instruction::Mask(WriteMask { zeros, ones })
        });

    let write = pair(
        right(match_literal("mem["), integer),
        right(match_literal("] = "), integer),
        |address, value| Instruction::Write {
            address: address as Address,
            value: value as Word
        }
    );

    let program = zero_or_more(whitespace_wrap(
            either(mask, write)
    ));

    program.parse(input)
}

// --- problems

fn part1(program: &Program) -> Word {
    let mut machine = Machine::new();
    machine.run(&program);
    machine.sum_of_all_memory_words()

}

fn main() {
    let input = read_file("./input.txt").unwrap();
    let program = parse_input(&input).unwrap().1;
    println!("part 1 {:?}", part1(&program));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_program() -> &'static str {
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
         mem[8] = 11
         mem[7] = 101
         mem[8] = 0"
    }

    #[test]
    fn test_parser() {
        let program = parse_input(sample_program());
        assert_eq!(program, Ok(("", vec![
            Instruction::Mask(WriteMask { zeros: 2, ones: 64 }),
            Instruction::Write { address: 8, value: 11 },
            Instruction::Write { address: 7, value: 101 },
            Instruction::Write { address: 8, value: 0 }
        ])));
    }

    #[test]
    fn test_part1() {
        let program = parse_input(sample_program()).unwrap().1;
        assert_eq!(part1(&program), 165);
    }
}