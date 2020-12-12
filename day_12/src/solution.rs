use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

use super::ferry::Ferry;

pub struct Day12 {
    input: Vec<String>
}

impl Day12 {
    pub fn new() -> Day12 {
        Day12 { input: Vec::new() }
    }
}

impl PuzzleTrait for Day12 {
    fn print_info(&self) {
        println!("DAY 12 - RAIN RISK");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_12.txt");
    }

    // Part one: what is the Manhattan distance between the current location and the starting position?
    fn solve_part_one(&self) {
        let mut ferry = Ferry::new(&self.input);

        loop {
            if !ferry.execute_next_action_abs() {
                // Last action has been reached, no more actions exist
                break;
            }
        }

        let answer = ferry.get_distance_from_start();
        println!("Answer part one: {} is the Manhattan distance between the start position and the current position", answer);
    }

    // Part two: ___
    fn solve_part_two(&self) {
        println!("Answer part two: {}", -1);
    }
}
