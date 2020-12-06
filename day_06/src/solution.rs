use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

use std::collections::HashMap;
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
        let mut unique_group_answers = HashSet::new();

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

    // Part two: to which question did everyone in a group answer "yes"
    fn solve_part_two(&self) {
        let mut number_people_in_group = 0;
        let mut number_same_answers = 0;
        let mut group_answers = HashMap::new();

        for (index, line) in self.input.iter().enumerate() {
            // Save this group's answers
            if line.trim().len() > 0 {
                number_people_in_group += 1;
            }

            for answer in line.chars() {
                let occurances = group_answers.entry(answer).or_insert(0);
                *occurances += 1;
            }

            // End of a group
            if line.trim().len() == 0 || index == self.input.len() - 1 {
                println!("{:?} - in group: {}", group_answers, number_people_in_group);

                for (key, value) in &group_answers {
                    if *value == number_people_in_group {
                        number_same_answers += 1;
                    }
                }

                number_people_in_group = 0;
                group_answers.clear();
            }
        }

        println!("Answer part two: {} is the sum of the number of questions everyone in a group answered with \"yes\"", number_same_answers);
    }
}
