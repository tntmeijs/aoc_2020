use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

pub struct Day07 {
    input: Vec<String>
}

impl Day07 {
    pub fn new() -> Day07 {
        Day07 { input: Vec::new() }
    }
}

impl PuzzleTrait for Day07 {
    fn print_info(&self) {
        println!("DAY 7 - HANDY HAVERSACKS");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_07.txt");
    }

    // Part one: how many bag colors can contain at least one shiny gold bag?
    fn solve_part_one(&self) {
        println!("Answer part one: {} bags can contain at least one shiny gold bag", -1);
    }

    // Part two: ___
    fn solve_part_two(&self) {
        println!("Answer part two: {} ___", -1);
    }
}
