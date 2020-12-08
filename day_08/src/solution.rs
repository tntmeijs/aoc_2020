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

        println!("Answer part one: {}", -1);
    }

    // Part two: ___
    fn solve_part_two(&self) {
            println!("Answer part two: {}", -1);
    }
}
