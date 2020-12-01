use day_one::DayOne;
use shared::puzzle_input::PuzzleTrait;

fn main() {
    println!("Learning Rust with the Advent of Code!");
    let mut day_1 = DayOne::new();
    day_1.print_info();
    day_1.gather_input();
    day_1.solve_part_one();
    day_1.solve_part_two();
}
