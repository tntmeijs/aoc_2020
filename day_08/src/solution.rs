use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

use super::instruction::Instruction;
use super::instruction::InstructionType;

pub struct Day08 {
    input: Vec<String>
}

impl Day08 {
    pub fn new() -> Day08 {
        Day08 { input: Vec::new() }
    }
}

fn parse_line_into_instruction(line: &str) -> Instruction {
    // Instructions are three characters wide, so split at the third character to get instruction - value pairs
    let (instruction, value) = line.split_at(3);

    // String to value
    let instruction_value = value.trim().parse::<i64>().unwrap();

    // String to type
    let instruction_type = match instruction {
        "acc" => InstructionType::ACC(instruction_value),
        "jmp" => InstructionType::JMP(instruction_value),
        _ => InstructionType::NOP
    };

    Instruction { hit_count: 0, instruction_type: instruction_type }
}

impl PuzzleTrait for Day08 {
    fn print_info(&self) {
        println!("DAY 8 - HANDHELD HALTING");
    }

    fn gather_input(&mut self) {
        self.input = read_input_as_vec("./input/day_08.txt");
    }

    // Part one: what value is the accumulator as soon as an instruction is executed twice?
    fn solve_part_one(&self) {
        let mut program_counter = 0i64;
        let mut accumulator = 0i64;

        // Read instructions into "ram"
        let mut instructions = Vec::new();
        for line in &self.input {
            instructions.push(parse_line_into_instruction(line));
        }

        loop {
            // Fetch an instruction
            let instruction = &mut instructions[program_counter as usize];

            let old_accumulator = accumulator;
            instruction.execute(&mut program_counter, &mut accumulator);

            if instruction.hit_count == 2 {
                println!("Answer part one: {} is the value of the accumulator before the same instruction is executed a second time", old_accumulator);
                break;
            }
        }
    }

    // Part two: change one jmp into nop or one nop into jmp, what is the value of the accumulator after the program terminates?
    fn solve_part_two(&self) {
        // Oof, brute forcing the solution is not pretty and introduces O(scary) complexity :(
        for index in 0..(self.input.len() - 1) {
            // Early out when no nop or jmp instruction exists at this index
            if !self.input[index].contains("jmp") && !self.input[index].contains("nop") {
                continue;
            }

            // Change the instruction at the current index
            // Either jmp turns into nop, or nop turns into jmp
            let mut modified_input = self.input.clone();
            let current_line = &modified_input[index];

            if current_line.contains("jmp") {
                modified_input[index] = current_line.replace("jmp", "nop");
            } else if current_line.contains("nop") {
                modified_input[index] = current_line.replace("nop", "jmp");
            }

            let mut program_counter = 0i64;
            let mut accumulator = 0i64;

            // Load instructions into "ram"
            let mut instructions = Vec::new();
            for line in &modified_input {
                instructions.push(parse_line_into_instruction(line));
            }

            loop {
                // Fetch an instruction
                let instruction = &mut instructions[program_counter as usize];
                instruction.execute(&mut program_counter, &mut accumulator);
    
                // Still an infinite loop, try to replace a different nop / jmp instruction
                if instruction.hit_count == 2 {
                    break;
                }

                // Program reached the end, we can now terminate
                if program_counter as usize >= instructions.len() {
                    println!("Answer part two: {} is the value of the accumulator after terminating", accumulator);
                    return;
                }
            }
        }
    }
}
