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

use shared::puzzle_trait::PuzzleTrait;

fn main() {
    // List of pointers to the puzzle objects
    let mut puzzles: Vec<Box<dyn PuzzleTrait>> = Vec::new();

    // Instantiate all puzzles
    puzzles.push(Box::new(Day01::new()));
    puzzles.push(Box::new(Day02::new()));
    puzzles.push(Box::new(Day03::new()));
    puzzles.push(Box::new(Day04::new()));
    puzzles.push(Box::new(Day05::new()));
    puzzles.push(Box::new(Day06::new()));
    puzzles.push(Box::new(Day07::new()));
    puzzles.push(Box::new(Day08::new()));
    puzzles.push(Box::new(Day09::new()));
    puzzles.push(Box::new(Day10::new()));
    puzzles.push(Box::new(Day11::new()));
    puzzles.push(Box::new(Day12::new()));
    puzzles.push(Box::new(Day13::new()));
    puzzles.push(Box::new(Day14::new()));
    puzzles.push(Box::new(Day15::new()));
    puzzles.push(Box::new(Day16::new()));

    // Run all puzzles
    let puzzle_count = puzzles.len();
    for (index, puzzle) in puzzles.iter_mut().enumerate() {
        puzzle.print_info();
        puzzle.gather_input();
        puzzle.solve_part_one();
        puzzle.solve_part_two();

        // Separate puzzle answers
        if index < puzzle_count - 1 {
            println!();
        }
    }
}
