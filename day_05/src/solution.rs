use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

pub struct Day05 {
    input: Vec<String>
}

impl Day05 {
    pub fn new() -> Day05 {
        Day05 { input: Vec::new() }
    }
}

enum Operation {
    TakeLowerHalf,
    TakeUpperHalf
}

struct LowHighRange {
    lower: u8,
    higher: u8
}

impl LowHighRange {
    // Check if the range has been narrowed down to a single value
    fn is_complete(&self) -> bool {
        self.lower == self.higher
    }
}

// Split the range either into the upper half, or into the lower half
fn calculate_correct_half(values: LowHighRange, operation: Operation) -> LowHighRange {
    let difference = values.higher as f64 - values.lower as f64;
    let half = (difference / 2.0).ceil() as u8;

    let mut result = LowHighRange { lower: values.lower, higher: values.higher };

    match operation {
        Operation::TakeLowerHalf => result.higher -= half,
        Operation::TakeUpperHalf => result.lower += half
    }

    result
}

// Returns the seat ID from the boarding pass
fn get_seat_id_from_boarding_pass(boarding_pass: &str) -> u32 {
    let mut row_range = LowHighRange { lower: 0, higher: 127 };
    let mut seat_range = LowHighRange { lower: 0, higher: 7 };

    for character in boarding_pass.chars() {
        if character != 'L' && character != 'R' {
            // Find the row that contains the seat
            if character == 'F' {
                row_range = calculate_correct_half(row_range, Operation::TakeLowerHalf);
            } else if character == 'B' {
                row_range = calculate_correct_half(row_range, Operation::TakeUpperHalf);
            }
        } else {
            // Once this block executes, the row must be known
            assert_eq!(row_range.is_complete(), true, "Rows are not equal: lower {} != higher {}", row_range.lower, row_range.higher);

            // Find the seat in the row
            if character == 'L' {
                seat_range = calculate_correct_half(seat_range, Operation::TakeLowerHalf);
            } else {
                // Take upper half
                seat_range = calculate_correct_half(seat_range, Operation::TakeUpperHalf);
            }
        }
    }

    // Once this executes, the seat must be known
    assert_eq!(seat_range.is_complete(), true, "Seats are not equal: lower {} != higher {}", seat_range.lower, seat_range.higher);

    // Calculate seat ID
    row_range.lower as u32 * 8 + seat_range.lower as u32
}

impl PuzzleTrait for Day05 {
    fn print_info(&self) {
        println!("DAY 5 - BINARY BOARDING");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_05.txt");
    }

    // Part one: find the highest seat ID on the boarding passes
    fn solve_part_one(&self) {
        let mut highest_seat_id = 0;

        for boarding_pass in &self.input {
            let result = get_seat_id_from_boarding_pass(boarding_pass);

            if result > highest_seat_id {
                highest_seat_id = result;
            }
        }

        println!("Answer part one: {} is the highest seat ID", highest_seat_id);
    }

    // Part two: find which seat ID is missing in the sequence
    fn solve_part_two(&self) {
        let mut all_seat_ids: Vec<u32> = Vec::new();

        for boarding_pass in &self.input {
            all_seat_ids.push(get_seat_id_from_boarding_pass(boarding_pass));
        }

        // Sort ascending to make it possible to find irregularities in the sequence
        all_seat_ids.sort();

        // Find missing ID
        let mut previous_id = all_seat_ids[0];
        for id in &all_seat_ids {
            if id - previous_id > 1 {
                // The seat between these two seats is my seat
                println!("Answer part two: {} my seat ID", id - 1);
            }

            previous_id = *id;
        }
    }
}
