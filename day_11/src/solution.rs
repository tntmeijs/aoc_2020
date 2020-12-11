use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

use super::grid::CellState;
use super::grid::CellNeighbour;
use super::grid::Grid;

pub struct Day11 {
    input: Vec<String>
}

impl Day11 {
    pub fn new() -> Day11 {
        Day11 { input: Vec::new() }
    }
}

fn input_to_grid(input: &Vec<String>) -> Grid {
    let row_count = input.len();
    let column_count = input[0].len();

    // Allocate the grid that represents the seating area
    let mut state = Grid::new(row_count, column_count);

    // Fill the seating area
    for (row, line) in input.iter().enumerate() {
        for (column, seat) in line.chars().enumerate() {
            state.set_cell(row, column, CellState::from_char(seat));
        }
    }

    // Ensure the grid is ready to be used
    assert_eq!(state.is_valid(), true, "Not all cells in the grid have a valid state");

    // Grid has been constructed
    state
}

impl PuzzleTrait for Day11 {
    fn print_info(&self) {
        println!("DAY 11 - SEATING SYSTEM");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_11.txt");
    }

    // Part one: how many seats end up occupied?
    fn solve_part_one(&self) {
        let mut current_state = input_to_grid(&self.input);

        // Keep chaging the grid until it becomes stable
        loop {
            // The new grid will be based off of the result of the previous iteration
            let mut next_state = current_state.clone();

            for row in 0..current_state.row_count() {
                for column in 0..current_state.column_count() {
                    // Cell to update this iteration
                    let seat = current_state.get_cell_state(row, column);

                    // Attempt to get all cells surrounding the cell we have to check
                    let top             = current_state.get_neightbour_state(row, column, CellNeighbour::Top);
                    let bottom          = current_state.get_neightbour_state(row, column, CellNeighbour::Bottom);
                    let left            = current_state.get_neightbour_state(row, column, CellNeighbour::Left);
                    let right           = current_state.get_neightbour_state(row, column, CellNeighbour::Right);
                    let top_left        = current_state.get_neightbour_state(row, column, CellNeighbour::TopLeft);
                    let top_right       = current_state.get_neightbour_state(row, column, CellNeighbour::TopRight);
                    let bottom_left     = current_state.get_neightbour_state(row, column, CellNeighbour::BottomLeft);
                    let bottom_right    = current_state.get_neightbour_state(row, column, CellNeighbour::BottomRight);

                    if seat == CellState::Available {
                        let empty_around =
                            top             != CellState::Occupied &&
                            bottom          != CellState::Occupied &&
                            left            != CellState::Occupied &&
                            right           != CellState::Occupied &&
                            top_left        != CellState::Occupied &&
                            top_right       != CellState::Occupied &&
                            bottom_left     != CellState::Occupied &&
                            bottom_right    != CellState::Occupied;

                        // The seat becomes occupied if all surrounding seats are empty
                        if empty_around {
                            next_state.set_cell(row, column, CellState::Occupied);
                        }
                    } else if seat == CellState::Occupied {
                        let mut adjecent_occupied = 0;
                        if top          == CellState::Occupied { adjecent_occupied += 1 }
                        if bottom       == CellState::Occupied { adjecent_occupied += 1 }
                        if left         == CellState::Occupied { adjecent_occupied += 1 }
                        if right        == CellState::Occupied { adjecent_occupied += 1 }
                        if top_left     == CellState::Occupied { adjecent_occupied += 1 }
                        if top_right    == CellState::Occupied { adjecent_occupied += 1 }
                        if bottom_left  == CellState::Occupied { adjecent_occupied += 1 }
                        if bottom_right == CellState::Occupied { adjecent_occupied += 1 }

                        // The seat becomes available if four or more seats surrounding it become occupied
                        if adjecent_occupied >= 4 {
                            next_state.set_cell(row, column, CellState::Available);
                        }
                    }
                }
            }

            assert_eq!(next_state.is_valid(), true, "Invalid next state, one or more cells have an invalid state");
            if Grid::are_equal(&current_state, &next_state) {
                break;
            }

            current_state = next_state;
        }

        // Seating arrangement no longer changes, calculate how many seats are occupied
        let mut occupied_seats = 0;
        for row in 0..current_state.row_count() {
            for column in 0..current_state.column_count() {
                if current_state.get_cell_state(row, column) == CellState::Occupied {
                    occupied_seats += 1;
                }
            }
        }

        println!("Answer part one: {} seats end up being occupied", occupied_seats);
    }

    // Part two: ___
    fn solve_part_two(&self) {
        println!("Answer part two: {}", -1);
    }
}
