pub mod puzzle_input {
    use std::fs;

    pub fn read_input_as_vec(path: &str) -> Vec<String> {
        let content  = fs::read_to_string(path).expect("Unable to read file into a string");
        
        let mut input = Vec::new();
        for line in content.lines() {
            input.push(line.to_string());
        }
        
        input
    }

    pub trait PuzzleTrait {
        fn print_info(&self);
        fn gather_input(&mut self);
        fn solve_part_one(&self);
        fn solve_part_two(&self);
    }
}
