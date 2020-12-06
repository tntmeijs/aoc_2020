use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

pub struct Day06 {
    input: Vec<String>
}

impl Day06 {
    pub fn new() -> Day06 {
        Day06 { input: Vec::new() }
    }
}

impl PuzzleTrait for Day06 {
    fn print_info(&self) {
        println!("DAY 6 - CUSTOM CUSTOMS");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_06.txt");
    }

    // Part one: how many people answered unique questions
    fn solve_part_one(&self) {
        let mut question_count = 0;

        println!("Answer part one: {} is the sum of the number of questions people answered with \"yes\"", question_count);
    }

    // Part two: ___
    fn solve_part_two(&self) {
    }
}
