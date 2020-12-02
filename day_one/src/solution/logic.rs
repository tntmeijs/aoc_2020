use shared::file_io::input::read_input_as_vec;
use shared::traits::puzzle_trait::PuzzleTrait;

pub struct DayOne {
    input: Vec<String>
}

impl DayOne {
    pub fn new() -> DayOne {
        DayOne { input: Vec::new() }
    }
}

impl PuzzleTrait for DayOne {
    fn print_info(&self) {
        println!("DAY 1 - REPORT REPAIR");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_one.txt");
    }

    // Part one: in the input list, find two entries that sum to 2020 and multiply them
    fn solve_part_one(&self) {
        // Need a vector of integers
        let vec_ints: Vec<u32> = self.input.iter().map(|value| value.parse().unwrap()).collect();

        // I <3 brute force solutions!
        for value_a in &vec_ints {
            for value_b in &vec_ints {
                if value_a + value_b == 2020 {
                    println!("{} + {} = 2020", value_a, value_b);
                    println!("Answer part one: {}", value_a * value_b);
                    return
                }
            }
        }
    }

    // Part two: same as part one, but with three instead of two numbers
    fn solve_part_two(&self) {
        let vec_ints: Vec<u32> = self.input.iter().map(|value| value.parse().unwrap()).collect();

        // O(scary), but as I said, I <3 brute force solutions!
        for value_a in &vec_ints {
            for value_b in &vec_ints {
                for value_c in &vec_ints {
                    if value_a + value_b + value_c == 2020 {
                        println!("{} + {} + {} = 2020", value_a, value_b, value_c);
                        println!("Answer part two: {}", value_a * value_b * value_c);
                        return
                    }
                }
            }
        }
    }
}
