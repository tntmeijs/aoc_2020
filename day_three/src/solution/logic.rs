use shared::file_io::input::read_input_as_vec;
use shared::traits::puzzle_trait::PuzzleTrait;

struct InputData {
    slope_length: u32,
    slope_rows: Vec<String>
}

impl InputData {
    pub fn new() -> InputData {
        InputData { slope_length: 0, slope_rows: Vec::new() }
    }
}

struct SlopeMovement {
    right: u32,
    down: u32
}

impl SlopeMovement {
    pub fn new() -> SlopeMovement {
        SlopeMovement { right: 0, down: 1 }
    }
}

pub struct DayThree {
    input: InputData
}

impl DayThree {
    pub fn new() -> DayThree {
        DayThree { input: InputData::new() }
    }
}

impl PuzzleTrait for DayThree {
    fn print_info(&self) {
        println!("DAY 3 - TOBOGGAN TRAJECTORY")
    }

    fn gather_input(&mut self) {
        let raw_input = read_input_as_vec("./input/day_three.txt");
        self.input.slope_length = raw_input.len() as u32;
        self.input.slope_rows = raw_input;
    }

    // Part one: find how many threes you encounter when moving right 3, down 1
    fn solve_part_one(&self) {
    }

    // Part two: ___
    fn solve_part_two(&self) {
    }
}
