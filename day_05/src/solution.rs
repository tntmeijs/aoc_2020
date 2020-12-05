use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

pub struct Day05 {
    input: Vec<String>
}

impl Day05 {
    pub fn new() -> Day05 {
        Day05 { input: Vec::new() }
    }
}

impl PuzzleTrait for Day05 {
    fn print_info(&self) {
        println!("DAY 5 - BINARY BOARDING");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_05.txt");
    }

    // Part one: find the highest seat ID on the boarding passes
    fn solve_part_one(&self) {
    }

    // Part two: ___
    fn solve_part_two(&self) {
    }
}
