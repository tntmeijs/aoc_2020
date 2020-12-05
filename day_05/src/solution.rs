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

// Returns the seat ID from the boarding pass
fn get_seat_id_from_boarding_pass(boarding_pass: &str) -> u32 {
    let mut lower_row = 0;
    let mut higher_row = 127;

    let mut lower_seat = 0;
    let mut higher_seat = 7;

    for character in boarding_pass.chars() {
        if character != 'L' && character != 'R' {
            let difference: f64 = higher_row as f64 - lower_row as f64;
            let half = (difference / 2.0).ceil() as u8;

            // Find correct row
            if character == 'F' {
                // Take lower half
                higher_row -= half;
            } else {
                // Take upper half
                lower_row += half;
            }
        } else {
            // Once this block executes, the row has been found
            assert_eq!(lower_row, higher_row, "Rows are not equal: lower {} != higher {}", lower_row, higher_row);

            // Find correct seat
            let difference: f64 = higher_seat as f64 - lower_seat as f64;
            let half = (difference / 2.0).ceil() as u8;

            // Find correct row
            if character == 'L' {
                // Take lower half
                higher_seat -= half;
            } else {
                // Take upper half
                lower_seat += half;
            }
        }
    }

    // Rows and columns have been found
    assert_eq!(lower_seat, higher_seat, "Seats are not equal: lower {} != higher {}", lower_seat, higher_seat);

    // Calculate seat ID
    lower_row as u32 * 8 + lower_seat as u32
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
