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
        self.input = read_input_as_vec("./input/day_two.txt");
    }

    // Part one: find how many passwords are valid according to the rules
    fn solve_part_one(&self) {
        let mut valid_password_count: u32 = 0;

        for input in &self.input {
            let parts: Vec<&str> = input.split(':').collect();
            
            // Invalid input
            if parts.len() != 2 {
                continue;
            }

            // Relevant input data without whitespaces
            let policy = parts[0].trim();
            let password = parts[1].trim();

            // Deconstruct the policy string into the information and the character it applies to
            let policy_parts: Vec<&str> = policy.split(' ').collect();

            // Invalid input
            if policy_parts.len() != 2 {
                continue;
            }

            let policy_info = policy_parts[0];
            let target_character = policy_parts[1];

            // Deconstruct policy info
            let min_max: Vec<usize> = policy_info.split('-').map(|value| value.parse().unwrap()).collect();

            // Invalid input
            if min_max.len() != 2 {
                continue;
            }

            let min = min_max[0];
            let max = min_max[1];

            // Password validation
            let occurances = password.matches(target_character).count();
            if occurances >= min && occurances <= max {
                valid_password_count += 1;
            }
        }

        println!("Answer part one: {}", valid_password_count);
    }

    // Part two: ___
    fn solve_part_two(&self) {
    }
}
