use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

#[derive(Clone)]
struct Bag {
    color: String,
    bags: Vec<Bag>
}

impl Bag {
    pub fn new() -> Bag {
        Bag { color: "".to_string(), bags: Vec::new() }
    }
}

pub struct Day07 {
    bag_tree: Vec<Bag>
}

impl Day07 {
    pub fn new() -> Day07 {
        Day07 { bag_tree: Vec::new() }
    }
}

fn pretty_print_tree(root: &Bag, depth: usize) {
    let mut root_line = " ".repeat(depth);
    root_line += &root.color;

    println!("{}", root.color);
}

impl PuzzleTrait for Day07 {
    fn print_info(&self) {
        println!("DAY 7 - HANDY HAVERSACKS");
    }

    fn gather_input(&mut self) {
        let raw_input = read_input_as_vec("./input/day_07.txt");

        for line in &raw_input {
            // Each line has the following format:
            // bag name contain count bag name, count bag name, [...]
            // First, split on "contains" to split the line up into root bag and children
            let parts = line.split("contain").collect::<Vec<&str>>();

            // Get the root bag color from the first part
            let root_bag_color = parts[0].replace("bags", "").trim().to_string();

            // Second part contains all children: count name, count name, [...]
            let children = parts[1]
                .split(',')                         // Split on commas
                .map(|bag| bag.replace('.', ""))    // Remove periods
                .map(|bag| bag.replace("bags", "")) // Remove "bags"
                .map(|bag| bag.replace("bag", ""))  // Remove "bag" (important to do this after "bags" to avoid leaving the 's' in the result)
                .map(|bag| bag.trim().to_string())  // Remove whitespace
                .collect::<Vec<String>>();          // Collect as a copy of the string

            let mut child_bags = Vec::new();

            for child_desc in &children {
                if child_desc.starts_with("no other") {
                    break;
                }

                // The number of children is always a single character
                let count = child_desc
                    .chars()        // Iterate over characters
                    .nth(0)         // Grab first character
                    .unwrap()       // Get the actual character instead of the Option<T>
                    .to_digit(10)   // Convert to base 10
                    .unwrap();      // Another Option<T> to value

                // Now that the number has been extracted, the first two characters (number and a space) can be removed
                let color = child_desc[2..].to_string();

                // Add all child bags to the container
                for _index in 0..count {
                    child_bags.push(Bag { color: color.to_string(), bags: Vec::new() });
                }
            }

            // Attempt to copy the data to the tree
            let mut copied_once = false;
            for root_index in 0..self.bag_tree.len() {
                let root = &mut self.bag_tree[root_index];
                for bag_index in 0..root.bags.len() {
                    let bag = &mut root.bags[bag_index];

                    // Bag exists, clone this data to the bag to form a tree-like structure
                    if bag.color == root_bag_color {
                        bag.bags = child_bags.clone();
                        copied_once = true;
                    }
                }
            }

            // None of the existing entries contain this bag, treat it as a new root bag
            if !copied_once {
                self.bag_tree.push(Bag { color: root_bag_color, bags: child_bags });
            }
        }

        // Pretty-print
        for root in &self.bag_tree {
            pretty_print_tree(root, 0);
        }
    }

    // Part one: how many bag colors can contain at least one shiny gold bag?
    fn solve_part_one(&self) {
        println!("Answer part one: {} bags can contain at least one shiny gold bag", -1);
    }

    // Part two: ___
    fn solve_part_two(&self) {
        println!("Answer part two: {} ___", -1);
    }
}
