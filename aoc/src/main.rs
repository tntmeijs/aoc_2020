use day_one::solution::logic::DayOne;
use day_two::solution::logic::DayTwo;
use day_three::solution::logic::DayThree;
use shared::traits::puzzle_trait::PuzzleTrait;

// #TODO: Since these objects all implement PuzzleTrait we can put them in a Vec<PuzzleTrait>
fn main() {
    println!("Learning Rust with the Advent of Code!");
    let mut day_1 = DayOne::new();
    let mut day_2 = DayTwo::new();
    let mut day_3 = DayThree::new();

    day_1.print_info();
    day_1.gather_input();
    day_1.solve_part_one();
    day_1.solve_part_two();

    day_2.print_info();
    day_2.gather_input();
    day_2.solve_part_one();
    day_2.solve_part_two();

    day_3.print_info();
    day_3.gather_input();
    day_3.solve_part_one();
    day_3.solve_part_two();
}
