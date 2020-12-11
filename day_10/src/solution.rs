use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

use std::collections::BTreeMap;

pub struct Day10 {
    input: Vec<u32>
}

impl Day10 {
    pub fn new() -> Day10 {
        Day10 { input: Vec::new() }
    }
}

impl PuzzleTrait for Day10 {
    fn print_info(&self) {
        println!("DAY 10 - ADAPTER ARRAY");
    }

    fn gather_input(&mut self) {
        let input = read_input_as_vec("./input/day_10.txt");

        for line in &input {
            self.input.push(line.parse::<u32>().unwrap());
        }

        self.input.sort();
    }

    // Part one: what is the number of 1-jolt differences multiplied by the number of 3-jolt differences?
    fn solve_part_one(&self) {
        let mut one_jolt_difference_count = 0;
        let mut three_jolt_difference_count = 0;
        let mut previous_adapter_joltage = 0;

        for adapter in &self.input {
            let difference = adapter - previous_adapter_joltage;
            previous_adapter_joltage = *adapter;

            if difference == 1 {
                one_jolt_difference_count += 1;
            } else if difference == 3 {
                three_jolt_difference_count += 1;
            }
        }

        // The laptop itself is a 3 jolt adapter as well
        three_jolt_difference_count += 1;

        let answer = one_jolt_difference_count * three_jolt_difference_count;
        println!("Answer part one: {} is the number of one jolt differences multiplied by the number of three jolt differences", answer);
    }

    // Part two: what is the total number of distinct ways you can arrange the adapters to connect the charging outlet to your device?
    fn solve_part_two(&self) {
        // let mut possible_paths = BTreeMap::new();
        
        // // Every adapter starts at zero possible ways to reach it (initial state)
        // for adapter in &self.input {
        //     possible_paths.insert(adapter, 0);
        // }

        // // For each adapter, check how many adapters with a joltange of +1 to +3 exist
        // for i in 0..self.input.len() {
        //     let mut difference = 0;
        //     let mut path_count = 0;
        //     let mut next_index = i + 1;
        //     let mut previous = self.input[i];

        //     // Keep looking for adapters with a joltage difference between +1 and +3
        //     while difference <= 3 && next_index < self.input.len() - 1 {
        //         path_count += 1;

        //         let next = self.input[next_index];
        //         difference = next - previous;
        //         previous = next;
                
        //         next_index += 1;
        //     }

        //     // Save how many paths are available
        //     possible_paths.insert(&self.input[i], path_count);
        // }

        // // Calculate the total number of combinations by multiplying all paths
        // let all_paths = possible_paths.keys().copied().collect::<Vec<_>>();
        // let mut answer = *all_paths[0] as u128;

        // for i in 1..all_paths.len() {
        //     answer *= *all_paths[i] as u128;
        // }

        println!("Answer part two: {}", -1);
    }
}
