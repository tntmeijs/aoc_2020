use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

use std::collections::HashSet;

pub struct Day06 {
    input: Vec<String>
}

impl Day06 {
    pub fn new() -> Day06 {
        Day06 { input: Vec::new() }
    }
}

impl PuzzleTrait for Day06 {
    fn print_info(&self) {
        println!("DAY 6 - CUSTOM CUSTOMS");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_06.txt");
    }

    // Part one: how many people answered unique questions
    fn solve_part_one(&self) {
        let mut unique_answers_total = 0;
        let mut unique_group_answers: HashSet<char> = HashSet::new();

        for (index, line) in self.input.iter().enumerate() {
            // Check answers
            for answer in line.chars() {
                unique_group_answers.insert(answer);
            }

            // End of a group
            if line.trim().len() == 0 || index == self.input.len() - 1 {
                unique_answers_total += unique_group_answers.len();
                unique_group_answers.clear();
            }
        }

        println!("Answer part one: {} is the sum of the number of questions people answered with \"yes\"", unique_answers_total);
    }

    // Part two: ___
    fn solve_part_two(&self) {
    }
}
