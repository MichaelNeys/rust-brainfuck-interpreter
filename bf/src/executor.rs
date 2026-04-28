use crate::instruction::{Instruction, InstructionList};
use crate::memory_tape::MemoryTape;
use std::io::{Read, stdin};

pub struct Executor{
    instruction_list: InstructionList,
    memory: MemoryTape,
}


impl Executor{
    pub fn new(instruction_list: InstructionList) -> Executor{
        Executor{instruction_list, memory: MemoryTape::new()}
    }

    pub fn run(&mut self){
        while !self.instruction_list.is_at_end() {
            self.execute_instruction();
        }
        // flush last section
        println!();
    }

    fn execute_instruction(&mut self){
        let to_execute: &Instruction = self.instruction_list.next_instruction().unwrap();
        match to_execute {
            Instruction::Right(count) => for _ in 0..*count { self.memory.move_pointer_right();}
            Instruction::Left(count) => for _ in 0..*count { self.memory.move_pointer_left();}
            Instruction::Add {count, offset} => for _ in 0..*count { self.memory.add_at_pointer(*offset); }
            Instruction::Sub {count, offset} => for _ in 0..*count { self.memory.subtract_at_pointer(*offset); }
            Instruction::Print(count) => for _ in 0..*count {
                let char = self.memory.get_at_pointer(0) as char;
                print!("{char}");
            }
            Instruction::Read(count) => for _ in 0..*count {
                let mut stdin_handle = stdin().lock();
                let mut byte = [0_u8];
                stdin_handle.read_exact(&mut byte).unwrap();
                self.memory.set_at_pointer(byte[0], 0);
            }
            Instruction::JumpToLeft() => self.instruction_list.execute_jump_to_left(),
            Instruction::JumpToRight() => self.instruction_list.execute_jump_to_left(),
            Instruction::Reset() => {
                self.memory.set_at_pointer(0, 0);
            }
            Instruction::FindEmptyRight(_) => {
                while self.memory.get_at_pointer(0) != 0{
                    self.memory.move_pointer_right();
                }
            }
            Instruction::FindEmptyLeft(_) => {
                while self.memory.get_at_pointer(0) != 0{
                    self.memory.move_pointer_left();
                }
            }
        }
    }


}
