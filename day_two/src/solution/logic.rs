use shared::file_io::input::read_input_as_vec;
use shared::traits::puzzle_trait::PuzzleTrait;

struct ParsedInput {
    password: String,
    target: char,
    index_one: usize,
    index_two: usize
}

pub struct DayTwo {
    input: Vec<String>
}

impl DayTwo {
    pub fn new() -> DayTwo {
        DayTwo { input: Vec::new() }
    }

    fn parse(&self, input: &str) -> ParsedInput {
        let parts: Vec<&str> = input.split(':').collect();

        // Relevant input data without whitespaces
        let policy = parts[0].trim();
        let password = parts[1].trim();

        // Deconstruct the policy string into the information and the character it applies to
        let policy_parts: Vec<&str> = policy.split(' ').collect();

        let policy_info = policy_parts[0];
        let target = policy_parts[1];

        // Deconstruct policy info
        let min_max: Vec<usize> = policy_info.split('-').map(|value| value.parse().unwrap()).collect();

        ParsedInput { password: password.to_string(), index_one: min_max[0], index_two: min_max[1], target: target.chars().nth(0).unwrap() }
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
        let mut valid_password_count = 0;

        for input in &self.input {
            let parsed_input = self.parse(input);

            // Check if the target character occurs as often as the policy dictates
            let occurances = parsed_input.password.matches(parsed_input.target).count();
            if occurances >= parsed_input.index_one && occurances <= parsed_input.index_two {
                valid_password_count += 1;
            }
        }

        println!("Answer part one: {} valid passwords", valid_password_count);
    }

    // Part two: validate if characters are at the expected index
    fn solve_part_two(&self) {
        let mut valid_password_count = 0;

        for input in &self.input {
            let parsed_input = self.parse(input);

            // Indexing starts at 0 in Rust, but the problem requires indexing to start at one
            let first_index_char = parsed_input.password.chars().nth(parsed_input.index_one - 1).unwrap();
            let second_index_char = parsed_input.password.chars().nth(parsed_input.index_two - 1).unwrap();

            // Policy violation: both characters need to be different
            if first_index_char == second_index_char {
                continue;
            }

            // One of the two characters has to equal the target character
            // The check above already eliminates the possibility of duplicate characters
            if first_index_char == parsed_input.target || second_index_char == parsed_input.target {
                valid_password_count += 1;
            }
        }

        println!("Answer part two: {} valid passwords", valid_password_count);
    }
}
