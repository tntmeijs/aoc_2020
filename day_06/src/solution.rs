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

    // Part one: how many people per group answered a question that has not answered by anyone else in their group yet
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

    // Part two: find how many times people in a group all answered "yes" to the same question
    fn solve_part_two(&self) {
        let mut number_people_in_group = 0;
        let mut number_same_answers = 0;
        let mut group_answers = HashMap::new();

        for (index, line) in self.input.iter().enumerate() {
            // Save this group's answers
            if line.trim().len() > 0 {
                number_people_in_group += 1;

                for answer in line.chars() {
                    // Keep track of how many times a question has been answered by members of the group
                    let responses = group_answers.entry(answer).or_insert(0);
                    *responses += 1;
                }
            }

            // End of a group
            if line.trim().len() == 0 || index == self.input.len() - 1 {
                for responses in group_answers.values() {
                    // Everyone in the group answered this question
                    if *responses == number_people_in_group {
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
