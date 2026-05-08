use crate::instruction::{Instruction, InstructionList};

pub struct Parser;

impl Parser {

    pub fn parse(code: &str) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        let chars: Vec<char> = code.chars().filter(|c| "+-<>.,[]".contains(*c)).collect();

        for char in chars {
            match char {
                '+' => instructions.push(Instruction::Add {
                    count: 1,
                    offset: 0,
                }),
                '-' => instructions.push(Instruction::Add {
                    count: -1,
                    offset: 0,
                }),
                '>' => instructions.push(Instruction::Move(1)),
                '<' => instructions.push(Instruction::Move(-1)),
                '.' => instructions.push(Instruction::Print(1)),
                ',' => instructions.push(Instruction::Read()),
                '[' => instructions.push(Instruction::JumpToRight()),
                ']' => instructions.push(Instruction::JumpToLeft()),
                _ => {}
            }
        }
        instructions
    }
}
