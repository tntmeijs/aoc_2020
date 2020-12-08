use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

pub struct Day08 {
    input: Vec<String>
}

impl Day08 {
    pub fn new() -> Day08 {
        Day08 { input: Vec::new() }
    }
}

impl PuzzleTrait for Day08 {
    fn print_info(&self) {
        println!("DAY 8 - HANDHELD HALTING");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_08.txt");
    }

    // Part one: what value is the accumulator as soon as an instruction is executed twice?
    fn solve_part_one(&self) {
        println!("Answer part one: {}", -1);
    }

    // Part two: ___
    fn solve_part_two(&self) {
            println!("Answer part two: {}", -1);
    }
}
