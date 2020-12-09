use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

use std::collections::HashSet;
use std::collections::BTreeMap;
use std::collections::VecDeque;

use shared::tree::Tree;

pub struct Day07 {
    input: Vec<String>
}

impl Day07 {
    pub fn new() -> Day07 {
        Day07 { input: Vec::new() }
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
        let mut tree = Tree::new(String::from("root"));
        let root_index = tree.find_root_index();

        for line in &self.input {
            // Split bag name from child names
            let parts = line.split("contain").collect::<Vec<&str>>();

            let bag_color = parts[0].replace("bags", "").replace("bag", "").trim().to_string();
            let mut child_bags = Vec::new();

            // Find child nodes
            if !parts[1].trim().starts_with("no other bag") {
                // Get child names and counts
                let children = parts[1].split(", ").collect::<Vec<&str>>();
            
                for child in &children {
                    let pretty_child = child.replace(".", "").replace("bags", "").replace("bag", "").trim().to_string();

                    // First character is the count
                    let occurances = pretty_child.chars().nth(0).unwrap().to_digit(10).unwrap();

                    // Skip the count and a space to get the child color
                    let child_color = pretty_child[2..].to_string();

                    for _i in 0..occurances {
                        child_bags.push(child_color.clone());
                    }
                }
            }

            // Does this bag color already exist in the tree?
            let mut found_indices = tree.find_all(bag_color.clone());
            if found_indices.is_empty() {
                // Bag does not exist in the tree yet, attach it to the root node
                found_indices.push(tree.insert(root_index, bag_color.clone()));
            }

            // Add all children to the bags in the tree
            for index in found_indices {
                for child in &child_bags {
                    tree.insert(index, child.clone());
                }
            }
        }

        // Find all bag colors that can hold at least one shiny gold bag
        let shiny_gold_bags = tree.find_all("shiny gold".to_string());
        let mut unique_colors = HashSet::new();

        // From the bottom of the tree, traverse upwards until the root node has been reached
        for bag_index in shiny_gold_bags {
            let mut current_node = tree.get_by_index(bag_index);
            loop {
                // Reached the top of the tree
                if current_node.parent_index.is_none() || current_node.parent_index.unwrap() == root_index {
                    break;
                }

                // Keep traversing towards the root node, saving all encountered colors in the process
                unique_colors.insert(current_node.data.clone());
                current_node = tree.get_by_index(current_node.parent_index.unwrap());
            }
        }

        let mut frontier = VecDeque::new();
        frontier.push_back(0);

        let mut nodes_by_depth = BTreeMap::new();
        for depth in 0..tree.max_depth + 1 {
            nodes_by_depth.insert(depth, Vec::new());
        }

        while !frontier.is_empty() {
            // Breadth-first traversal
            let index = frontier.pop_front().unwrap();
            let node = tree.get_by_index(index);
            let depth = node.depth;

            // Add children to be searched next
            for child in &node.children {
                frontier.push_back(*child);
            }

            // Save this node
            nodes_by_depth.get_mut(&depth).unwrap().push(node);
        }

        for (depth, nodes) in &nodes_by_depth {
            // Print padding
            for _i in 0..*depth {
                print!("    ");
            }

            for (index, node) in nodes.iter().enumerate() {
                print!("{}", node.data);

                if index < nodes.len() - 1 {
                    print!(" ");
                }
            }


            println!();
        }

        println!("Answer part one: {} bags can eventually contain at least one shiny gold bag", unique_colors.len());
    }

    // Part two: ___
    fn solve_part_two(&self) {
        println!("Answer part two: {} ___", -1);
    }
}
