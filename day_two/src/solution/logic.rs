use shared::file_io::input::read_input_as_vec;
use shared::traits::puzzle_trait::PuzzleTrait;

pub struct DayTwo {
    input: Vec<String>
}

impl DayTwo {
    pub fn new() -> DayTwo {
        DayTwo { input: Vec::new() }
    }
}

impl PuzzleTrait for DayTwo {
    fn print_info(&self) {
        println!("DAY 2 - PASSWORD PHILOSOPHY")
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_one.txt");
    }

    // Part one: find how many passwords are valid according to the rules
    fn solve_part_one(&self) {
    }

    // Part two: ___
    fn solve_part_two(&self) {
    }
}
