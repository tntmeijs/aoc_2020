use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

pub struct Day16 {
    input: Vec<String>
}

impl Day16 {
    pub fn new() -> Day16 {
        Day16 { input: Vec::new() }
    }
}

impl PuzzleTrait for Day16 {
    fn print_info(&self) {
        println!("DAY 16 - TICKET TRANSLATION");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_16.txt");
    }

    // Part one: what is your ticket scanning error rate?
    fn solve_part_one(&self) {
        println!("Answer part one: {}", -1);
    }

    // Part two: ___
    fn solve_part_two(&self) {
        println!("Answer part two: {}", -1);
    }
}
