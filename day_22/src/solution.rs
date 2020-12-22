use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

pub struct Day22 {
    input: Vec<String>
}

impl Day22 {
    pub fn new() -> Day22 {
        Day22 { input: Vec::new() }
    }
}

impl PuzzleTrait for Day22 {
    fn print_info(&self) {
        println!("DAY 22 - CRAB COMBAT");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_22.txt");
    }

    // Part one: what is the winning player's score?
    fn solve_part_one(&self) {
        println!("Answer part one: {}", -1);
    }

    // Part two: ___
    fn solve_part_two(&self) {
        println!("Answer part two: {}", -1);
    }
}
