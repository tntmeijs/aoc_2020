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

fn find_invalid_number(input: &Vec<u64>, preable_length: usize) -> u64 {
    let mut buffer = VecDeque::new();

    // Fill up the buffer with the preamble
    for value in &input[0..preable_length] {
        buffer.push_back(value);
    }

    // Go through the input, constantly checking the buffer against the numbers
    for value in &input[preable_length..] {
        let mut found = false;

        // Check which value is not the sum of any of the numbers in the preamble
        for first in &buffer {
            for second in &buffer {
                if *first + *second == *value {
                    found = true;
                }                    
            }
        }

        // The sum could not be computed, this means that we have found the invalid number
        if !found {
            return *value;
        }

        // Move the rolling window
        buffer.pop_front();
        buffer.push_back(value);
    }

    // Should never happen as long as the input is valid
    panic!("Unable to find the invalid numbers, please ensure that your input file is valid");
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
        let answer = find_invalid_number(&self.input, 25);
        println!("Answer part one: {} is the first number that is not the sum of any of the 25 numbers that came before it", answer);
    }

    // Part two: find the encryption weakness in the XMAS-encrypted list of numbers
    fn solve_part_two(&self) {
        let invalid_number = find_invalid_number(&self.input, 25);

        let mut start_index = 0;
        let mut end_index = 0;

        // Try to find if the sequence sums up to the invalid number
        loop {
            let mut sum = 0;
            let mut sequence = Vec::new();

            // Calculate the sum of the sequence
            for i in start_index..end_index {
                let value = self.input[i];
                sum += value;
                sequence.push(value);
            }

            // Found the encryption weakness
            if sum == invalid_number {
                // Sort to easily get the smallest and largest numbers
                sequence.sort();

                // The encryption weakness is the sum of the smallest and largest number in the sequence
                let answer = sequence[0] + sequence[sequence.len() - 1];
                println!("Answer part two: {} is the encryption weakness of the XMAS protocol", answer);
                return;
            }

            if sum > invalid_number {
                // Sum exceeds the invalid number, reset the sequence and start again at the next number
                start_index += 1;
                end_index = start_index;
            } else {
                // Sum less than invalid number, keep increasing the length of the sequence
                end_index += 1;
            }

            // New iteration, which means there is a new sequence to check
            sequence.clear();
        }
    }
}
