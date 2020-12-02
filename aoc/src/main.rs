use day_one::solution::logic::DayOne;
use day_two::solution::logic::DayTwo;
use shared::traits::puzzle_trait::PuzzleTrait;

fn main() {
    println!("Learning Rust with the Advent of Code!");
    let mut day_1 = DayOne::new();
    let mut day_2 = DayTwo::new();

    day_1.print_info();
    day_1.gather_input();
    day_1.solve_part_one();
    day_1.solve_part_two();

    day_2.print_info();
    day_2.gather_input();
    day_2.solve_part_one();
    day_2.solve_part_two();
}
