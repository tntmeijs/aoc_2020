use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

pub struct Day21 {
}

impl Day21 {
    pub fn new() -> Day21 {
        Day21 {}
    }
}

impl PuzzleTrait for Day21 {
    fn print_info(&self) {
        println!("DAY 21 - ALLERGEN ASSESSMENT");
    }

    fn gather_input(&mut self) {
        let input = read_input_as_vec("./input/day_21.txt");
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
