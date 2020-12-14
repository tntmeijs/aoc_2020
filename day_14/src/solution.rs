use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

use std::collections::HashMap;

pub struct Day14 {
    input: Vec<String>
}

struct Instruction {
    value: u64,
    address: u32
}

impl Day14 {
    pub fn new() -> Day14 {
        Day14 { input: Vec::new() }
    }

    fn apply_mask_from_line(&self, line: &str) -> u64 {
        let mut mask = 0;
        for (index, character) in line.chars().enumerate() {
            if character == '1' {
                mask |= 1 << (line.len() - 1 - index);
            }
        }

        mask
    }

    fn line_to_instruction(&self, line: &str) -> Instruction {
        let address = line.chars().nth(4).unwrap().to_digit(10).unwrap();
        let value = line[9..].parse::<u64>().unwrap();

        println!("value: {}, address: {}", value, address);

        Instruction { value: value, address: address }
    }
}

impl PuzzleTrait for Day14 {
    fn print_info(&self) {
        println!("DAY 14 - DOCKING DATA");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_14.txt");
    }

    // Part one: what is the sum of all values left in memory after it completes?
    fn solve_part_one(&self) {
        let mut addres_storage = HashMap::new();
        let mut zero_indices = Vec::new();
        let mut one_indices = Vec::new();

        for line in &self.input {
            if line.starts_with("mask") {
                zero_indices.clear();
                one_indices.clear();

                for (index, character) in line[7..].chars().enumerate() {
                    let bit_index = line.len() - 8 - index;
                    match character {
                        '0' => zero_indices.push(bit_index),
                        '1' => one_indices.push(bit_index),
                        _ => ()
                    };
                }
            } else {
                let address = line[4..].split(']').collect::<Vec<_>>()[0].trim().parse::<u64>().unwrap();
                let mut value_from_input = line.split('=').collect::<Vec<_>>()[1].trim().parse::<u64>().unwrap();

                for index in &zero_indices {
                    value_from_input &= !(1 << index);
                }

                for index in &one_indices {
                    value_from_input |= 1 << index;
                }

                addres_storage.insert(address, value_from_input);
            }
        }

        let mut answer = 0;
        for entry in &addres_storage {
            answer += entry.1;
        }

        println!("Answer part one: {} is the sum of all values left in memory", answer);
    }

    // Part two: ___
    fn solve_part_two(&self) {
        println!("Answer part two: {}", -1);
    }
}
