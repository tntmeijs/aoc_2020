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

    fn solve_part_two(&self) {}
}
