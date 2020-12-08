use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

#[allow(dead_code)]
struct Bag {
    color: String,
    bags: Vec<Bag>
}

#[allow(dead_code)]
fn get_next_bag (input: &Vec<String>, line: usize) -> Bag {
    // Split bag name from child names
    let parts = input[line].split("contain").collect::<Vec<&str>>();

    let bag_color = parts[0].trim().replace("bags", "").replace("bag", "");
    let mut child_bags = Vec::new();

    // Find child nodes
    if !parts[1].trim().starts_with("no other bag") {
        // Get child names and counts
        let children = parts[1].split(", ").collect::<Vec<&str>>();
        
        for child in &children {
            let pretty_child = child.trim().replace(".", "").replace("bags", "").replace("bag", "");

            // First character is the count
            let occurances = pretty_child.chars().nth(0).unwrap().to_digit(10).unwrap();

            // Skip the count and a space to get the child color
            let child_color = pretty_child[2..].to_string();

            for _i in 0..occurances {
                child_bags.push(Bag { color: child_color.clone(), bags: Vec::new() });
            }
        }
    }

    Bag { color: bag_color, bags: child_bags }
}

#[allow(dead_code)]
pub struct Day07 {
    input: Vec<String>,
    current_line: usize,
}

impl Day07 {
    pub fn new() -> Day07 {
        Day07 { input: Vec::new(), current_line: 0 }
    }
}

impl PuzzleTrait for Day07 {
    fn print_info(&self) {
        println!("DAY 7 - HANDY HAVERSACKS");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_07.txt");
    }

    // Part one: how many bag colors can contain at least one shiny gold bag?
    fn solve_part_one(&self) {
    }

    // Part two: ___
    fn solve_part_two(&self) {
        println!("Answer part two: {} ___", -1);
    }
}
