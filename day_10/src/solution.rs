use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

pub struct Day10 {
    input: Vec<String>
}

impl Day10 {
    pub fn new() -> Day10 {
        Day10 { input: Vec::new() }
    }
}

impl PuzzleTrait for Day10 {
    fn print_info(&self) {
        println!("DAY 10 - ___");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_10.txt");
    }

    // Part one: ___
    fn solve_part_one(&self) {
        println!("Answer part one: {}", -1);
    }

    // Part two: ___
    fn solve_part_two(&self) {
        println!("Answer part two: {}", -1);
    }
}
