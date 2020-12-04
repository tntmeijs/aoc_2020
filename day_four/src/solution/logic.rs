use shared::file_io::input::read_input_as_vec;
use shared::traits::puzzle_trait::PuzzleTrait;

pub struct DayFour;

impl DayFour {
    pub fn new() -> DayFour {
        DayFour
    }
}

impl PuzzleTrait for DayFour {
    fn print_info(&self) {
        println!("DAY 4 - PASSPORT PROCESSING")
    }

    fn gather_input(&mut self) {
        let raw_input = read_input_as_vec("./input/day_four.txt");
    }

    // Part one: check how many documents are valid passports
    fn solve_part_one(&self) {
    }

    // Part two: ___
    fn solve_part_two(&self) {
    }
}
