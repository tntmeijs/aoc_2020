use day_01::solution::Day01;
use day_02::solution::Day02;
use day_03::solution::Day03;
use day_04::solution::Day04;
use day_05::solution::Day05;
use day_06::solution::Day06;
use shared::puzzle_trait::PuzzleTrait;

// #TODO: Since these objects all implement PuzzleTrait we can put them in a Vec<PuzzleTrait>
fn main() {
    let mut day_01 = Day01::new();
    let mut day_02 = Day02::new();
    let mut day_03 = Day03::new();
    let mut day_04 = Day04::new();
    let mut day_05 = Day05::new();
    let mut day_06 = Day06::new();

    day_01.print_info();
    day_01.gather_input();
    day_01.solve_part_one();
    day_01.solve_part_two();

    println!();

    day_02.print_info();
    day_02.gather_input();
    day_02.solve_part_one();
    day_02.solve_part_two();

    println!();

    day_03.print_info();
    day_03.gather_input();
    day_03.solve_part_one();
    day_03.solve_part_two();

    println!();

    day_04.print_info();
    day_04.gather_input();
    day_04.solve_part_one();
    day_04.solve_part_two();

    println!();

    day_05.print_info();
    day_05.gather_input();
    day_05.solve_part_one();
    day_05.solve_part_two();
    
    println!();

    day_06.print_info();
    day_06.gather_input();
    day_06.solve_part_one();
    day_06.solve_part_two();
}
