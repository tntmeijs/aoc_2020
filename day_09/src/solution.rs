use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

pub struct Day09 {
    input: Vec<String>
}

impl Day09 {
    pub fn new() -> Day09 {
        Day09 { input: Vec::new() }
    }
}

impl PuzzleTrait for Day09 {
    fn print_info(&self) {
        println!("DAY 9 - ENCODING ERROR");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_09.txt");
    }

    // Part one: what is the first number that is not the sum of two of the 25 numbers before it
    fn solve_part_one(&self) {
        println!("Answer part one: {}", -1);
    }

    // Part two: ___
    fn solve_part_two(&self) {
    }
}
