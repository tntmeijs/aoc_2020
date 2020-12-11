use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

pub struct Day11 {
    input: Vec<String>
}

impl Day11 {
    pub fn new() -> Day11 {
        Day11 { input: Vec::new() }
    }
}

impl PuzzleTrait for Day11 {
    fn print_info(&self) {
        println!("DAY 11 - SEATING SYSTEM");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_11.txt");
    }

    // Part one: how many seats end up occupied?
    fn solve_part_one(&self) {
        println!("Answer part one: {}", -1);
    }

    // Part two: ___
    fn solve_part_two(&self) {
        println!("Answer part two: {}", -1);
    }
}
