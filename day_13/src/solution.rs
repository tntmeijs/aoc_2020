use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

pub struct Day13 {
    input: Vec<String>
}

impl Day13 {
    pub fn new() -> Day13 {
        Day13 { input: Vec::new() }
    }
}

impl PuzzleTrait for Day13 {
    fn print_info(&self) {
        println!("DAY 13 - SHUTTLE SEARCH");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_13.txt");
    }

    // Part one: what is the ID of the earliest bus you can take to the airport multiplied by the number of minutes you will need to wait for that bus?
    fn solve_part_one(&self) {
        let answer = -1;
        println!("Answer part one: {} is the ID of the earliest bus to the airport, multiplied with the waiting time", answer);
    }

    // Part two: ___
    fn solve_part_two(&self) {
        let answer = -1;
        println!("Answer part two: {}", answer);
    }
}
