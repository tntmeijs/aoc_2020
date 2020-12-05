use day_01::solution::DayOne;
use day_02::solution::DayTwo;
use day_03::solution::DayThree;
use day_04::solution::DayFour;
use shared::puzzle_trait::PuzzleTrait;

// #TODO: Since these objects all implement PuzzleTrait we can put them in a Vec<PuzzleTrait>
fn main() {
    let mut day_1 = DayOne::new();
    let mut day_2 = DayTwo::new();
    let mut day_3 = DayThree::new();
    let mut day_4 = DayFour::new();

    day_1.print_info();
    day_1.gather_input();
    day_1.solve_part_one();
    day_1.solve_part_two();

    println!();

    day_2.print_info();
    day_2.gather_input();
    day_2.solve_part_one();
    day_2.solve_part_two();

    println!();

    day_3.print_info();
    day_3.gather_input();
    day_3.solve_part_one();
    day_3.solve_part_two();

    println!();

    day_4.print_info();
    day_4.gather_input();
    day_4.solve_part_one();
    day_4.solve_part_two();
}
