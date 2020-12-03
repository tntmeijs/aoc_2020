use shared::file_io::input::read_input_as_vec;
use shared::traits::puzzle_trait::PuzzleTrait;

struct InputData {
    slope_length: usize,
    slope_width: usize,
    slope_rows: Vec<String>
}

impl InputData {
    pub fn new() -> InputData {
        InputData { slope_length: 0, slope_width: 0, slope_rows: Vec::new() }
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
        self.input.slope_length = raw_input.len() - 1;
        self.input.slope_width = raw_input[0].len();
        self.input.slope_rows = raw_input;
    }

    // Part one: find how many threes you encounter when moving right 3, down 1
    fn solve_part_one(&self) {
        let mut current_row = 0;
        let mut current_column = 0;
        let mut tree_count = 0;

        while current_row < self.input.slope_length {
            // Move
            current_row += 1;
            current_column += 3;

            // Grab the row data
            let row = &self.input.slope_rows[current_row];

            // Check the cell for '#' and account for wrapping around past the row's length
            if row.chars().nth(current_column % self.input.slope_width).unwrap() == '#' {
                tree_count += 1;
            }
        }

        println!("Answer part one: {} trees encountered", tree_count);
    }

    // Part two: ___
    fn solve_part_two(&self) {
    }
}
