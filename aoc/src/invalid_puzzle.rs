use shared::puzzle_trait::PuzzleTrait;

pub struct InvalidPuzzle;

impl InvalidPuzzle {
    pub fn new() -> InvalidPuzzle {
        InvalidPuzzle {}
    }
}

impl PuzzleTrait for InvalidPuzzle {
    fn print_info(&self) {
        println!("Puzzle not implemented");
    }

    fn gather_input(&mut self) {}

    fn solve_part_one(&self) {}

    fn solve_part_two(&self) {}
}
