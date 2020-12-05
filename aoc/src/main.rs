use day_01::solution::Day01;
use day_02::solution::Day02;
use day_03::solution::Day03;
use day_04::solution::Day04;
use shared::puzzle_trait::PuzzleTrait;

// #TODO: Since these objects all implement PuzzleTrait we can put them in a Vec<PuzzleTrait>
fn main() {
    let mut day_1 = Day01::new();
    let mut day_2 = Day02::new();
    let mut day_3 = Day03::new();
    let mut day_4 = Day04::new();

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
