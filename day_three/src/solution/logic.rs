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

fn traverse_slope(right: usize, down: usize, input: &InputData) -> u64 {
    let mut current_row = 0;
    let mut current_column = 0;
    let mut tree_count = 0;

    while current_row < input.slope_length {
        // Move
        current_row += down;
        current_column += right;

        // Grab the row data
        let row = &input.slope_rows[current_row];

        // Check the cell for '#' and account for wrapping around past the row's length
        if row.chars().nth(current_column % input.slope_width).unwrap() == '#' {
            tree_count += 1;
        }
    }

    tree_count
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
        println!("Answer part one: {} trees encountered", traverse_slope(3, 1, &self.input));
    }

    // Part two: follow a bunch of routes and multiply them all together
    fn solve_part_two(&self) {
        let tree_route_a = traverse_slope(1, 1, &self.input);
        let tree_route_b = traverse_slope(3, 1, &self.input);
        let tree_route_c = traverse_slope(5, 1, &self.input);
        let tree_route_d = traverse_slope(7, 1, &self.input);
        let tree_route_e = traverse_slope(1, 2, &self.input);

        let multiplied = tree_route_a * tree_route_b * tree_route_c * tree_route_d * tree_route_e;

        println!("Answer part two:");
        println!("\tRoute A: {} trees encountered", tree_route_a);
        println!("\tRoute B: {} trees encountered", tree_route_b);
        println!("\tRoute C: {} trees encountered", tree_route_c);
        println!("\tRoute D: {} trees encountered", tree_route_d);
        println!("\tRoute E: {} trees encountered", tree_route_e);
        println!();
        println!("\t{} * {} * {} * {} * {} = {}", tree_route_a, tree_route_b, tree_route_c, tree_route_d, tree_route_e, multiplied);
    }
}
