use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

use std::collections::VecDeque;

pub struct Day09 {
    input: Vec<u64>
}

impl Day09 {
    pub fn new() -> Day09 {
        Day09 { input: Vec::new() }
    }
}

impl PuzzleTrait for Day09 {
    fn print_info(&self) {
        println!("DAY 9 - ENCODING ERROR");
    }

    fn gather_input(&mut self) {
        let input = read_input_as_vec("./input/day_09.txt");
        for line in &input {
            self.input.push(line.parse::<u64>().unwrap());
        }
    }

    // Part one: what is the first number that is not the sum of two of the 25 numbers before it
    fn solve_part_one(&self) {
        let mut buffer = VecDeque::new();

        // Fill up the buffer with the preamble
        for value in &self.input[0..25] {
            buffer.push_back(value);
        }

        for value in &self.input[25..] {
            let mut found = false;

            // Check which value is not the sum of any of 25 numbers that came before it
            for first in &buffer {
                for second in &buffer {
                    if *first + *second == *value {
                        found = true;
                    }                    
                }
            }

            if !found {
                println!("Answer part one: {} is the first number that is not the sum of any of the 25 numbers that came before it", value);
                return;
            }

            // Update rolling window
            buffer.pop_front();
            buffer.push_back(value);
        }
    }

    // Part two: ___
    fn solve_part_two(&self) {
    }
}
