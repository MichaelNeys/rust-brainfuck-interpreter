use crate::instruction::{Instruction, InstructionList};
use crate::memory_tape::MemoryTape;
use std::io::{Read};

pub struct Executor<R: Read>{
    instruction_list: InstructionList,
    memory: MemoryTape,
    input: R
}


impl<R: Read> Executor<R>{
    pub fn new(instruction_list: InstructionList, input: R) -> Executor<R>{
        Executor{instruction_list, memory: MemoryTape::new(), input}
    }

    pub fn run(mut self){
        while !self.instruction_list.is_at_end() {
            self.execute_instruction();
        }
        // flush last section
        println!();
    }

    fn execute_instruction(&mut self){
        let to_execute: &Instruction = self.instruction_list.next_instruction().unwrap();

        //println!("Instruction: {:?}, Memory: {}, Pointer: {}", to_execute, self.memory.get_at_pointer(0), self.memory.pointer);
        match to_execute {
            Instruction::Move(count) => self.memory.move_pointer(*count),
            Instruction::Add {count} => self.memory.add_at_pointer(*count, 0),
            Instruction::Print() => {
                let char = self.memory.get_at_pointer(0) as char;
                print!("{char}");
            }
            Instruction::Read() => {
                let mut byte = [0_u8];
                self.input.read_exact(&mut byte).unwrap();
                self.memory.set_at_pointer(byte[0], 0);
            }
            Instruction::JumpToRight() => {
                if self.memory.get_at_pointer(0) == 0 {
                    self.instruction_list.execute_jump_to_right();
                }
            }
            Instruction::JumpToLeft() => {
                if self.memory.get_at_pointer(0) != 0 {
                    self.instruction_list.execute_jump_to_left();
                }
            }
            Instruction::Reset() => {
                self.memory.set_at_pointer(0, 0);
            }
            Instruction::FindEmptyRight(_) => {
                while self.memory.get_at_pointer(0) != 0{
                    self.memory.move_pointer(1);
                }
            }
            Instruction::FindEmptyLeft(_) => {
                while self.memory.get_at_pointer(0) != 0{
                    self.memory.move_pointer(-1);
                }
            }
            Instruction::Copy {offset, multiplier} => {
                self.memory.add_at_pointer(multiplier * self.memory.get_at_pointer(0) as i64, *offset);
            }
        }
    }


}
