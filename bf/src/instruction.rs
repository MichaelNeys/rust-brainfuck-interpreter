use std::collections::HashMap;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Instruction {
    Move(i64), // move pointer to right
    Add {
        count: i64,
        offset: i64,
    }, // add count to cell at offest from pointer
    Print(u64), // prints the current memory cell as ascii
    Read(),    // reads input to the current memory cell
    JumpToLeft(), // Jump to the next JumpToRight if current cell is 0
    JumpToRight(), // Jump to the next JumpToLeft if current cell is not 0
    FindEmptyRight(), // set pointer to the first empty cell to the right
    FindEmptyLeft(), // set pointer to the first empty cell to the left
    Reset(),   // reset current cell
    Copy {
        offset: i64,
        multiplier: i64,
    }, // copy the current cell value into offset
    CopyChunk {
        offset: i64,
        length: u32,
        multiplier: i64,
    },
}

#[derive(Debug, PartialEq)]
pub struct InstructionList {
    current_instruction: usize,
    instructions: Vec<Instruction>,
    jump_table: HashMap<usize, usize>,
}

fn build_jump_table(instructions: &[Instruction]) -> HashMap<usize, usize> {
    let mut jump_table: HashMap<usize, usize> = HashMap::new();

    let mut open_stack: Vec<usize> = vec![];

    for (current, instruction) in instructions.iter().enumerate() {
        match instruction {
            Instruction::JumpToRight() => open_stack.push(current),
            Instruction::JumpToLeft() => {
                let open_index = open_stack
                    .pop()
                    .expect("Unbalanced brackets: could not find '['");
                jump_table.entry(open_index).insert_entry(current);
                jump_table.entry(current).insert_entry(open_index);
            }
            _ => {}
        }
    }

    if !open_stack.is_empty() {
        panic!("Unbalanced brackets: could not find ']'");
    }

    jump_table
}

impl InstructionList {
    pub fn new(instructions: Vec<Instruction>) -> InstructionList {
        let jump_table: HashMap<usize, usize> = build_jump_table(&instructions);
        InstructionList {
            current_instruction: 0,
            instructions,
            jump_table,
        }
    }

    pub fn next_instruction(&mut self) -> Option<&Instruction> {
        let instruction = self.instructions.get(self.current_instruction);
        self.current_instruction += 1;
        instruction
    }

    pub fn execute_jump_to_right(&mut self) {
        let pointer = self.current_instruction - 1;
        let destination = self
            .jump_table
            .get(&pointer)
            .expect("Fatal error: location of jump right instruction not found");
        self.current_instruction = *destination;
    }

    pub fn execute_jump_to_left(&mut self) {
        let pointer = self.current_instruction - 1;
        let destination = self
            .jump_table
            .get(&pointer)
            .expect("Fatal error: location of jump left instruction not found");
        self.current_instruction = *destination;
    }

    pub fn is_at_end(&self) -> bool {
        self.current_instruction >= self.instructions.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::{Instruction, InstructionList};
    use std::collections::HashMap;

    #[test]
    fn test_new() {
        assert_eq!(
            InstructionList::new(vec![]),
            InstructionList {
                current_instruction: 0,
                instructions: vec![],
                jump_table: HashMap::from([])
            }
        );
        assert_eq!(
            InstructionList::new(vec![Instruction::Add {
                count: 5,
                offset: 0
            }]),
            InstructionList {
                current_instruction: 0,
                instructions: vec![Instruction::Add {
                    count: 5,
                    offset: 0
                }],
                jump_table: HashMap::from([])
            }
        );
    }

    #[test]
    fn test_next_instruction() {
        let mut list = InstructionList::new(vec![
            Instruction::Add {
                count: 5,
                offset: 0,
            },
            Instruction::Add {
                count: 2,
                offset: 0,
            },
        ]);
        assert_eq!(
            list.next_instruction(),
            Some(&Instruction::Add {
                count: 5,
                offset: 0
            })
        );
        assert_eq!(
            list.next_instruction(),
            Some(&Instruction::Add {
                count: 2,
                offset: 0
            })
        );
    }

    #[test]
    fn test_jump_to_right() {
        let mut list = InstructionList::new(vec![
            Instruction::Add {
                count: 5,
                offset: 0,
            },
            Instruction::Add {
                count: 2,
                offset: 0,
            },
            Instruction::JumpToRight(),
            Instruction::JumpToLeft(),
            Instruction::Print(1),
        ]);
        list.next_instruction();
        list.next_instruction();
        list.next_instruction();
        list.execute_jump_to_right();
        assert_eq!(list.next_instruction(), Some(&Instruction::JumpToLeft()))
    }

    #[test]
    fn test_jump_to_left() {
        let mut list = InstructionList::new(vec![
            Instruction::JumpToRight(),
            Instruction::Add {
                count: 5,
                offset: 0,
            },
            Instruction::JumpToLeft(),
            Instruction::Add {
                count: 2,
                offset: 0,
            },
            Instruction::Print(1),
        ]);
        list.next_instruction();
        list.next_instruction();
        list.next_instruction();
        list.execute_jump_to_left();
        assert_eq!(list.next_instruction(), Some(&Instruction::JumpToRight()))
    }
}
