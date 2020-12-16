use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

use std::collections::HashMap;

pub struct Day15 {
    input: Vec<String>
}

impl Day15 {
    pub fn new() -> Day15 {
        Day15 { input: Vec::new() }
    }
}

impl PuzzleTrait for Day15 {
    fn print_info(&self) {
        println!("DAY 15 - RAMBUNCTIOUS RECITATION");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_15.txt");
    }

    // Part one: what will be the 2020th number spoken?
    fn solve_part_one(&self) {
        println!("Answer part one: {}", -1);
    }

    // Part two: ___
    fn solve_part_two(&self) {
        println!("Answer part two: {}", -1);
    }
}
