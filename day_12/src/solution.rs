use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

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
        println!("Answer part one: {} is the Manhattan distance between the start position and the current position", -1);
    }

    // Part two: ___
    fn solve_part_two(&self) {
        println!("Answer part two: {}", -1);
    }
}
