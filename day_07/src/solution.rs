use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

use shared::tree::Tree;

// #[allow(dead_code)]
// fn get_next_bag (input: &Vec<String>, line: usize) -> Bag {
//     // Split bag name from child names
//     let parts = input[line].split("contain").collect::<Vec<&str>>();
//
//     let bag_color = parts[0].trim().replace("bags", "").replace("bag", "");
//     let mut child_bags = Vec::new();
//
//     // Find child nodes
//     if !parts[1].trim().starts_with("no other bag") {
//         // Get child names and counts
//         let children = parts[1].split(", ").collect::<Vec<&str>>();
//      
//         for child in &children {
//             let pretty_child = child.trim().replace(".", "").replace("bags", "").replace("bag", "");
//
//             // First character is the count
//             let occurances = pretty_child.chars().nth(0).unwrap().to_digit(10).unwrap();
//
//             // Skip the count and a space to get the child color
//             let child_color = pretty_child[2..].to_string();
//
//             for _i in 0..occurances {
//                 child_bags.push(Bag { color: child_color.clone(), bags: Vec::new() });
//             }
//         }
//     }
//
//     Bag { color: bag_color, bags: child_bags }
// }

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
        // Generate data to store in the tree below
        let node_0_data = "root node";
        let node_1_data = "node 1";
        let node_2_data = "node 2";
        let node_3_data = "node 3";
        let node_4_data = "node 4";
        let node_5_data = "node 5";
        let node_6_data = "target node";
        
        // Construct a tree
        let mut tree = Tree::new(node_0_data);

        // Add some data
        let node_0_index = 0;
        let node_1_index = tree.insert(node_0_index, node_1_data);
        let node_2_index = tree.insert(node_0_index, node_2_data);
        let node_3_index = tree.insert(node_1_index, node_3_data);
        let node_4_index = tree.insert(node_1_index, node_4_data);
        let node_5_index = tree.insert(node_2_index, node_5_data);
        let node_6_index = tree.insert(node_5_index, node_6_data);

        // Find a node with the text "target node"
        let mut path = Vec::new();
        let results = tree.find_all("target node");
        if results.is_empty() {
            println!("ERROR: Not found");
        } else {
            let mut node = tree.get_by_index(results[0]);
            loop {
                if node.parent_index.is_none() {
                    break;
                }

                let parent_index = node.parent_index.unwrap();
                path.push(parent_index);
                node = tree.get_by_index(parent_index);
            }
        }

        print!("Target: {} ", results[0]);
        for (index, value) in path.iter().enumerate() {
            print!("{}", value);

            if index < path.len() - 1 {
                print!(" ");
            }
        }
        println!();
    }

    // Part two: ___
    fn solve_part_two(&self) {
        println!("Answer part two: {} ___", -1);
    }
}
