mod invalid_puzzle;
use invalid_puzzle::InvalidPuzzle;

use day_01::solution::Day01;
use day_02::solution::Day02;
use day_03::solution::Day03;
use day_04::solution::Day04;
use day_05::solution::Day05;
use day_06::solution::Day06;
use day_07::solution::Day07;
use day_08::solution::Day08;
use day_09::solution::Day09;
use day_10::solution::Day10;
use day_11::solution::Day11;
use day_12::solution::Day12;
use day_13::solution::Day13;
use day_14::solution::Day14;
use day_15::solution::Day15;
use day_16::solution::Day16;
use day_22::solution::Day22;

use shared::puzzle_trait::PuzzleTrait;

use std::collections::HashMap;

fn main() {
    // All puzzles
    let mut puzzles: HashMap<u32, Box<dyn PuzzleTrait>> = HashMap::new();
    puzzles.insert(0, Box::new(InvalidPuzzle::new()));
    puzzles.insert(1, Box::new(Day01::new()));
    puzzles.insert(2, Box::new(Day02::new()));
    puzzles.insert(3, Box::new(Day03::new()));
    puzzles.insert(4, Box::new(Day04::new()));
    puzzles.insert(5, Box::new(Day05::new()));
    puzzles.insert(6, Box::new(Day06::new()));
    puzzles.insert(7, Box::new(Day07::new()));
    puzzles.insert(8, Box::new(Day08::new()));
    puzzles.insert(9, Box::new(Day09::new()));
    puzzles.insert(10, Box::new(Day10::new()));
    puzzles.insert(11, Box::new(Day11::new()));
    puzzles.insert(12, Box::new(Day12::new()));
    puzzles.insert(13, Box::new(Day13::new()));
    puzzles.insert(14, Box::new(Day14::new()));
    puzzles.insert(15, Box::new(Day15::new()));
    puzzles.insert(16, Box::new(Day16::new()));
    puzzles.insert(22, Box::new(Day22::new()));

    // There is always one argument: the executable, all others are optional
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() == 1 {
        // Run all puzzles if no command-line arguments have been specified
        for key_value in &mut puzzles {
            key_value.1.print_info();
            key_value.1.gather_input();
            key_value.1.solve_part_one();
            key_value.1.solve_part_two();
        }
    } else if args.len() == 2 {
        let parse_result = args[1].parse::<u32>();
        if parse_result.is_err() {
            panic!("Invalid puzzle day specified, please use a number (u32)");
        }

        let mut puzzle_index = parse_result.unwrap();
        if puzzle_index == 0 || puzzle_index > 25 {
            panic!("Invalid puzzle day specified, please use a number (u32) between 1 and 25");
        }

        if !puzzles.contains_key(&puzzle_index) {
            puzzle_index = 0;
        }

        // Run the specified puzzle
        let puzzle = puzzles.get_mut(&puzzle_index).unwrap();
        puzzle.print_info();
        puzzle.gather_input();
        puzzle.solve_part_one();
        puzzle.solve_part_two();
    }
}
