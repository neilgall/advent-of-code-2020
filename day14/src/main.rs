use std::collections::HashMap;
use parser::*;

// -- model

type Address = u64;
type Word = u64;

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Mask { zeros: Word, ones: Word },
    Write { address: Address, value: Word }
}

type Program = Vec<Instruction>;

struct Machine {
    memory: HashMap<Address, Word>,
    mask_zeros: Word,
    mask_ones: Word,
    mask_floating: Word
}

impl Machine {
    fn new() -> Self {
        Machine { 
            memory: HashMap::new(),
            mask_zeros: 0,
            mask_ones: 0,
            mask_floating: 0
        }
    }

    fn run(&mut self, program: &Program) {
        program.iter().for_each(|instruction| match instruction {

            Instruction::Mask { zeros, ones } => {
                self.mask_zeros = *zeros;
                self.mask_ones = *ones;
            }

            Instruction::Write { address, value } => {
                self.memory.insert(*address, value & (!self.mask_zeros) | self.mask_ones);
            }
        });
    }

    fn run_v2(&mut self, program: &Program) {
        program.iter().for_each(|instruction| match instruction {

            Instruction::Mask { zeros, ones } => {
                self.mask_zeros = *zeros;
                self.mask_ones = *ones;
                self.mask_floating = !(zeros | ones) & 0xfffffffff;
            }

            Instruction::Write { address, value } => {
                let address = address & !(self.mask_floating) | self.mask_ones;
                self.write_floating_address(&address, value, 0);
            }
        });
    }

    fn write_floating_address(&mut self, address: &Address, value: &Word, bit_index: usize) {
        let bit_mask = 1 << bit_index;
        if self.mask_floating & bit_mask != 0 {
            [address & !bit_mask, address | bit_mask].iter().for_each(|address| {
                self.memory.insert(*address, *value);
                self.write_floating_address(address, value, bit_index + 1);
            });

        } else if self.mask_floating >> bit_index != 0 {
            self.write_floating_address(address, value, bit_index + 1)
        }
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
            Instruction::Mask { zeros, ones }
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

fn part2(program: &Program) -> Word {
    let mut machine = Machine::new();
    machine.run_v2(&program);
    machine.sum_of_all_memory_words()
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let program = parse_input(&input).unwrap().1;
    println!("part 1 {:?}", part1(&program));
    println!("part 2 {:?}", part2(&program));
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
            Instruction::Mask { zeros: 2, ones: 64 },
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

    #[test]
    fn test_part2() {
        let program = parse_input("
            mask = 000000000000000000000000000000X1001X
            mem[42] = 100
            mask = 00000000000000000000000000000000X0XX
            mem[26] = 1
        ").unwrap().1;
        assert_eq!(part2(&program), 208);
    }
}