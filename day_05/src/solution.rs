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
    let mut row = 0u8;
    let mut seat = 0u8;

    // Calculate row
    for (index, character) in boarding_pass[0..7].chars().enumerate() {
        if character == 'B' { row |= 1 << (6 - (index % 8)) }
    }

    // Calculate column
    for (index, character) in boarding_pass[7..10].chars().enumerate() {
        if character == 'R' { seat |= 1 << (2 - index) }
    }

    // Calculate seat ID
    row as u32 * 8 + seat as u32
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
