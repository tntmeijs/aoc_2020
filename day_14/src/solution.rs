use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

use std::collections::HashMap;

pub struct Day14 {
    input: Vec<String>
}

impl Day14 {
    pub fn new() -> Day14 {
        Day14 { input: Vec::new() }
    }
}

#[allow(dead_code)]
fn calculate_unique_masks(masks: Vec<String>) -> Vec<String> {
    let mut new_masks: Vec<String> = Vec::new();

    for mask in masks {
        for (index, character) in mask.chars().enumerate() {
            if character == 'X' {
                // Create two new masks based on the old mask
                let mut mask_one = mask.chars().collect::<Vec<_>>();
                let mut mask_two = mask.chars().collect::<Vec<_>>();
    
                // Only possible combination is a zero or a one
                mask_one[index] = '0';
                mask_two[index] = '1';

                // Save new masks
                new_masks.push(mask_one.into_iter().collect());
                new_masks.push(mask_two.into_iter().collect());
            }
        }
    }

    // Check if there are still any floating bits in the masks
    let mut found_floating_bit = false;
    for mask in &new_masks {
        for character in mask.chars() {
            if character == 'X' {
                found_floating_bit = true;
            }
        }
    }

    if found_floating_bit {
        new_masks = calculate_unique_masks(new_masks);
    }

    new_masks
}

impl PuzzleTrait for Day14 {
    fn print_info(&self) {
        println!("DAY 14 - DOCKING DATA");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_14.txt");
    }

    // Part one: what is the sum of all values left in memory after it completes?
    fn solve_part_one(&self) {
        let mut addres_storage = HashMap::new();
        let mut zero_indices = Vec::new();
        let mut one_indices = Vec::new();

        for line in &self.input {
            if line.starts_with("mask") {
                zero_indices.clear();
                one_indices.clear();

                for (index, character) in line[7..].chars().enumerate() {
                    let bit_index = line.len() - 8 - index;
                    match character {
                        '0' => zero_indices.push(bit_index),
                        '1' => one_indices.push(bit_index),
                        _ => ()
                    };
                }
            } else {
                let address = line[4..].split(']').collect::<Vec<_>>()[0].trim().parse::<u64>().unwrap();
                let mut value_from_input = line.split('=').collect::<Vec<_>>()[1].trim().parse::<u64>().unwrap();

                for index in &zero_indices {
                    value_from_input &= !(1 << index);
                }

                for index in &one_indices {
                    value_from_input |= 1 << index;
                }

                addres_storage.insert(address, value_from_input);
            }
        }

        let mut answer = 0;
        for entry in &addres_storage {
            answer += entry.1;
        }

        println!("Answer part one: {} is the sum of all values left in memory", answer);
    }

    // Part two: what is the sum of all values left in memory after it completes?
    fn solve_part_two(&self) {
        // let mut addres_storage = HashMap::new();
        // let mut value_zero_indices = Vec::new();
        // let mut value_one_indices = Vec::new();
        // let mut mask = "";

        // for line in &self.input {
        //     if line.starts_with("mask") {
        //         value_zero_indices.clear();
        //         value_one_indices.clear();
        //         mask = &line[7..];

        //         for (index, character) in mask.chars().enumerate() {
        //             let bit_index = mask.len() - 1 - index;
        //             match character {
        //                 '0' => value_zero_indices.push(bit_index),
        //                 '1' => value_one_indices.push(bit_index),
        //                 _ => ()
        //             };
        //         }
        //     } else {
        //         // Calculate all possible address variations
        //         let address_masks = calculate_unique_masks(vec![mask.to_string()]);

        //         // Generate all addresses to write to
        //         let mut addresses = Vec::new();
        //         let address = line[4..].split(']').collect::<Vec<_>>()[0].trim().parse::<u64>().unwrap();
        //         for address_mask in &address_masks {
        //             let mut address_zero_indices = Vec::new();
        //             let mut address_one_indices = Vec::new();

        //             for (index, character) in address_mask.chars().enumerate() {
        //                 let bit_index = mask.len() - 1 - index;
        //                 match character {
        //                     '0' => address_zero_indices.push(bit_index),
        //                     '1' => address_one_indices.push(bit_index),
        //                     _ => ()
        //                 };
        //             }

        //             let mut address_copy = address.clone();
        //             for index in &address_zero_indices {
        //                 address_copy &= !(1 << index);
        //             }

        //             for index in &address_one_indices {
        //                 address_copy |= 1 << index;
        //             }

        //             addresses.push(address_copy);
        //         }

        //         // Calculate the final value to write using the mask
        //         let mut value_from_input = line.split('=').collect::<Vec<_>>()[1].trim().parse::<u64>().unwrap();
        //         for index in &value_zero_indices {
        //             value_from_input &= !(1 << index);
        //         }

        //         for index in &value_one_indices {
        //             value_from_input |= 1 << index;
        //         }

        //         // Write the same value to every address
        //         for unique_address in addresses {
        //             addres_storage.insert(unique_address, value_from_input);
        //         }
        //     }
        // }

        // let mut answer = 0;
        // for entry in &addres_storage {
        //     answer += entry.1;
        // }

        println!("Answer part two: {} is the sum of all values left in memory", -1);
    }
}
