use crate::executor::Executor;
use crate::instruction::{Instruction, InstructionList};

pub mod instruction;
pub mod memory_tape;
pub mod executor;

fn main() {
    let instructions: Vec<Instruction> = vec![Instruction::Add{count: 75, offset: 0}, Instruction::Print(1)];
    let instruction_list: InstructionList = InstructionList::new(instructions);

    let mut executor: Executor = Executor::new(instruction_list);

    executor.run();
}
