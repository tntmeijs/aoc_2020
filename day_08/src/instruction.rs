pub enum InstructionType {
    NOP,        // No instruction
    ACC(i64),   // Change accumulator
    JMP(i64)    // Jump to offset
}

pub struct Instruction {
    pub hit_count: usize,
    pub instruction_type: InstructionType
}

impl Instruction {
    pub fn execute(&mut self, program_counter: &mut i64, accumulator: &mut i64) {
        let old_pc = *program_counter;

        match self.instruction_type {
            InstructionType::ACC(value) => *accumulator += value,
            InstructionType::JMP(offset) => *program_counter += offset,
            InstructionType::NOP => ()
        }

        // Program counter has not been modified, increment it by one to move to the next instruction
        if old_pc == *program_counter {
            *program_counter += 1;
        }

        self.hit_count += 1;
    }
}
