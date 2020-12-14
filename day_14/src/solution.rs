use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

pub struct Day14 {
    input: Vec<String>
}

impl Day14 {
    pub fn new() -> Day14 {
        Day14 { input: Vec::new() }
    }
}

impl PuzzleTrait for Day14 {
    fn print_info(&self) {
        println!("DAY 14 - DOCKING DATA");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_14.txt");
    }

    // Part one: what is the sum of all values left in memory after it completes?
    fn solve_part_one(&self) {
        println!("Answer part one: {}", -1);
    }

    // Part two: ___
    fn solve_part_two(&self) {
        println!("Answer part two: {}", -1);
    }
}
