use crate::instruction::{Instruction, InstructionList};
use crate::memory_tape::MemoryTape;
use std::io::{self, ErrorKind, Read, Write};

pub struct Executor<R: Read> {
    instruction_list: InstructionList,
    memory: MemoryTape,
    input: R,
}

impl<R: Read> Executor<R> {
    pub fn new(instruction_list: InstructionList, input: R) -> Executor<R> {
        Executor {
            instruction_list,
            memory: MemoryTape::new(),
            input,
        }
    }

    pub fn run(mut self) {
        while !self.instruction_list.is_at_end() {
            self.execute_instruction();
        }
        // flush last section
        io::stdout().flush().unwrap();
    }

    fn execute_instruction(&mut self) {
        let to_execute: &Instruction = self.instruction_list.next_instruction().unwrap();

        //println!("Instruction: {:?}, Memory: {}, Pointer: {}", to_execute, self.memory.get_at_pointer(0), self.memory.pointer);
        match to_execute {
            Instruction::Move(count) => self.memory.move_pointer(*count),
            Instruction::Add { count, offset } => self.memory.add_at_pointer(*count, *offset),
            Instruction::Print(count) => {
                for _ in 0..*count {
                    let char = self.memory.get_at_pointer(0) as char;
                    print!("{char}");
                }
            }
            Instruction::Read() => {
                let mut byte = [0_u8];
                match self.input.read_exact(&mut byte) {
                    Ok(_) => {
                        self.memory.set_at_pointer(byte[0], 0);
                    }
                    Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                        // EOF
                        self.memory.set_at_pointer(0, 0);
                    }
                    _ => {
                        panic!("Could not read form file!");
                    }
                }
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
            Instruction::FindEmptyRight() => {
                while self.memory.get_at_pointer(0) != 0 {
                    self.memory.move_pointer(1);
                }
            }
            Instruction::FindEmptyLeft() => {
                while self.memory.get_at_pointer(0) != 0 {
                    self.memory.move_pointer(-1);
                }
            }
            Instruction::Copy { offset, multiplier } => {
                self.memory
                    .add_at_pointer(multiplier * self.memory.get_at_pointer(0) as i64, *offset);
            }
            Instruction::CopyChunk {
                offset,
                length,
                multiplier,
            } => {
                self.memory.copy_chunk(*offset, *length, *multiplier);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::executor::Executor;
    use crate::instruction::{Instruction, InstructionList};
    

    fn create_instruction_list(instructions: Vec<Instruction>) -> InstructionList {
        InstructionList::new(instructions) 
    }

    fn create_executor(instructions: Vec<Instruction>, input: &[u8]) -> Executor<&[u8]> {
        let list = create_instruction_list(instructions);
        Executor::new(list, input)
    }

    #[test]
    fn test_initialization() {
        let exec = create_executor(vec![], b"");
        assert_eq!(exec.memory.get_at_pointer(0), 0);
    }

    #[test]
    fn test_add_instruction() {
        let mut exec = create_executor(vec![
            Instruction::Add { count: 5, offset: 0 },
        ], b"");
        
        exec.execute_instruction();
        assert_eq!(exec.memory.get_at_pointer(0), 5);
    }

    #[test]
    fn test_add_offset_instruction() {
        let mut exec = create_executor(vec![
            Instruction::Add { count: 5, offset: 2 },
        ], b"");
        
        exec.execute_instruction();
        assert_eq!(exec.memory.get_at_pointer(2), 5);
    }

    #[test]
    fn test_move_instruction() {
        let mut exec = create_executor(vec![
            Instruction::Move(1),
            Instruction::Add { count: 10, offset: 0 },
            Instruction::Move(-1)
        ], b"");

        exec.execute_instruction();
        exec.execute_instruction();
        assert_eq!(exec.memory.get_at_pointer(0), 10);
        
        exec.execute_instruction();
        assert_eq!(exec.memory.get_at_pointer(0), 0);
    }

    #[test]
    fn test_read_instruction() {
        let mut exec = create_executor(vec![Instruction::Read()], b"A");
        
        exec.execute_instruction();
        
        assert_eq!(exec.memory.get_at_pointer(0), 65); // ASCII waarde van A
    }

    #[test]
    fn test_read_instruction_eof() {
        let mut exec = create_executor(vec![
            Instruction::Add { count: 10, offset: 0 },
            Instruction::Read()
        ], b"");
        
        exec.execute_instruction();
        assert_eq!(exec.memory.get_at_pointer(0), 10);
        
        exec.execute_instruction();
        assert_eq!(exec.memory.get_at_pointer(0), 0); //EOF zet cell naar 0
    }

    #[test]
    fn test_jump_to_right_when_zero() {
        let mut exec = create_executor(vec![
            Instruction::JumpToRight(), 
            Instruction::Add { count: 9, offset: 0 },
            Instruction::JumpToLeft(),
            Instruction::Add { count: 4, offset: 0 },
        ], b"");

        exec.execute_instruction();
        // hier wordt die Add 9 geskipt want we jumpen omdat cell 0 0 is
        assert_eq!(exec.memory.get_at_pointer(0), 0);
        exec.execute_instruction();
        assert_eq!(exec.memory.get_at_pointer(0), 4);
    }

    #[test]
    fn test_jump_to_left_loops_when_not_zero() {
        let mut exec = create_executor(vec![
            Instruction::Add { count: 2, offset: 0 },
            Instruction::JumpToRight(),
            Instruction::Add { count: -1, offset: 0 },
            Instruction::Move(1),
            Instruction::Add { count: 1, offset: 0 },
            Instruction::Move(-1),
            Instruction::JumpToLeft(),
        ], b"");

        exec.execute_instruction(); // cell 0 = 2
        exec.execute_instruction(); // niet jumpen want is nie nul
        assert_eq!(exec.memory.get_at_pointer(0), 2);
        exec.execute_instruction(); // decrement cell 0 naar 1
        assert_eq!(exec.memory.get_at_pointer(0), 1);
        exec.execute_instruction(); // opschuiven
        exec.execute_instruction(); // increment cell 1 naar 1
        assert_eq!(exec.memory.get_at_pointer(0), 1);
        exec.execute_instruction(); // terug opschuiven
        exec.execute_instruction(); // jumpen links want cell 0 is nie 0
        exec.execute_instruction(); // decrement cell 0 naar 0
        assert_eq!(exec.memory.get_at_pointer(0), 0);
        exec.execute_instruction(); // opschuiven
        exec.execute_instruction(); // increment cell 1 naar 2
        assert_eq!(exec.memory.get_at_pointer(0), 2);
        exec.execute_instruction(); // terug opschuiven
        exec.execute_instruction(); // niet jumpen want cell 0 is 0
        assert_eq!(exec.memory.get_at_pointer(0), 0);
        assert_eq!(exec.memory.get_at_pointer(1), 2); // cell 1 is nu wel 2 en die staart 1 move naar rechts van de huidige
    }

    #[test]
    fn test_reset_instruction() {
        let mut exec = create_executor(vec![
            Instruction::Add { count: 42, offset: 0 },
            Instruction::Reset()
        ], b"");

        exec.execute_instruction();
        assert_eq!(exec.memory.get_at_pointer(0), 42);

        exec.execute_instruction();
        assert_eq!(exec.memory.get_at_pointer(0), 0); // Reset zet cell naar 0
    }

    #[test]
    fn test_find_empty_right() {
        let mut exec = create_executor(vec![
            Instruction::Add { count: 1, offset: 0 },
            Instruction::Move(1),
            Instruction::Add { count: 2, offset: 0 },
            Instruction::Move(-1),
            Instruction::FindEmptyRight(),
            Instruction::Move(-1)
        ], b"");

        exec.execute_instruction();
        exec.execute_instruction();
        exec.execute_instruction();
        exec.execute_instruction();
        
        exec.execute_instruction(); // FindEmptyRight
        
        assert_eq!(exec.memory.get_at_pointer(0), 0); // zou nu op de eerste lege cell moete staan

        exec.execute_instruction();
        assert_eq!(exec.memory.get_at_pointer(0), 2); // 1 naar links zou dan 2 moeten bevatten
    }

    #[test]
    fn test_find_empty_left() {
        let mut exec = create_executor(vec![
            Instruction::Add { count: 3, offset: 0 },
            Instruction::Move(1),
            Instruction::Add { count: 2, offset: 0 },
            Instruction::Move(-1),
            Instruction::FindEmptyLeft(),
            Instruction::Move(1)
        ], b"");

        exec.execute_instruction();
        exec.execute_instruction();
        exec.execute_instruction();
        exec.execute_instruction();
        
        exec.execute_instruction(); // FindEmptyLeft
        
        assert_eq!(exec.memory.get_at_pointer(0), 0); // zou nu op de eerste lege cell moete staan

        exec.execute_instruction();
        assert_eq!(exec.memory.get_at_pointer(0), 3); // 1 naar rechts zou dan 3 moeten bevatten
    }

    #[test]
    fn test_copy_instruction() {
        let mut exec = create_executor(vec![
            Instruction::Add { count: 5, offset: 0 },
            Instruction::Copy { offset: 2, multiplier: 3 }
        ], b"");

        exec.execute_instruction();
        exec.execute_instruction();
        
        assert_eq!(exec.memory.get_at_pointer(0), 5);
        assert_eq!(exec.memory.get_at_pointer(2), 15);
    }

    #[test]
    fn test_copy_chunk_instruction() {
        let mut exec = create_executor(vec![
            Instruction::Add { count: 5, offset: 0 },
            Instruction::Add { count: 6, offset: 1 },
            Instruction::Add { count: 7, offset: 2 },
            Instruction::CopyChunk { offset: (10), length: (3), multiplier: (1) }
        ], b"");

        exec.execute_instruction();
        exec.execute_instruction();
        exec.execute_instruction();
        exec.execute_instruction();
        
        assert_eq!(exec.memory.get_at_pointer(10), 5);
        assert_eq!(exec.memory.get_at_pointer(11), 6);
        assert_eq!(exec.memory.get_at_pointer(12), 7);
    }

    #[test]
    fn test_copy_chunk_multiplier_instruction() {
        let mut exec = create_executor(vec![
            Instruction::Add { count: 5, offset: 0 },
            Instruction::Add { count: 6, offset: 1 },
            Instruction::Add { count: 7, offset: 2 },
            Instruction::CopyChunk { offset: (5), length: (3), multiplier: (2) }
        ], b"");

        exec.execute_instruction();
        exec.execute_instruction();
        exec.execute_instruction();
        exec.execute_instruction();
        
        assert_eq!(exec.memory.get_at_pointer(5), 10);
        assert_eq!(exec.memory.get_at_pointer(6), 12);
        assert_eq!(exec.memory.get_at_pointer(7), 14);
    }
}