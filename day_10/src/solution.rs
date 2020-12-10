use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

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

    // Part two: ___
    fn solve_part_two(&self) {
        println!("Answer part two: {}", -1);
    }
}
