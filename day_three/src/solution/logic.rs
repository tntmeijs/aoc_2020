use shared::file_io::input::read_input_as_vec;
use shared::traits::puzzle_trait::PuzzleTrait;

pub struct DayThree {
    input: Vec<String>
}

impl DayThree {
    pub fn new() -> DayThree {
        DayThree { input: Vec::new() }
    }
}

impl PuzzleTrait for DayThree {
    fn print_info(&self) {
        println!("DAY 3 - TOBOGGAN TRAJECTORY")
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_three.txt");
    }

    // Part one: find how many passwords are valid according to the rules
    fn solve_part_one(&self) {
    }

    // Part two: ___
    fn solve_part_two(&self) {
    }
}
