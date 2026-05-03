use std::io::{stdin, StdinLock};
use crate::executor::Executor;
use crate::instruction::{Instruction, InstructionList};

pub mod instruction;
pub mod memory_tape;
pub mod executor;

fn main() {
    let instructions: Vec<Instruction> = vec![Instruction::Read(1), Instruction::Print(5)];
    let instruction_list: InstructionList = InstructionList::new(instructions);

    let stdin_handle = stdin().lock();
    let executor: Executor<StdinLock> = Executor::new(instruction_list, stdin_handle);

    executor.run();
}
