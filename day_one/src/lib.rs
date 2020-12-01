use shared;

pub struct DayOne {
    input: Vec<String>
}

impl DayOne {
    pub fn new() -> DayOne {
        DayOne { input: Vec::new() }
    }
}

impl shared::puzzle_input::PuzzleTrait for DayOne {
    fn print_info(&self) {
        println!("DAY 1 - REPORT REPAIR");
    }

    fn gather_input(&mut self) {
        self.input = shared::puzzle_input::read_input_as_vec("D:/Projects/aoc_2020/input/day_one.txt");
    }

    fn solve_part_one(&self) {}

    fn solve_part_two(&self) {}
}
