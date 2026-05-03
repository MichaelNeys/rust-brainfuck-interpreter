use crate::instruction::{Instruction, InstructionList};

pub struct Parser;

impl Parser {
    pub fn parse(code: &str) -> InstructionList {
        let mut instructions = Vec::new();
        let chars: Vec<char> = code.chars().filter(|c| "+-<>.,[]".contains(*c)).collect();
        let mut i = 0;

        while i < chars.len() {
            match chars[i] {
                '+' | '-' => {
                    let mut count: i64 = 0;
                    while i < chars.len() && (chars[i] == '+' || chars[i] == '-') {
                        count += if chars[i] == '+' { 1 } else { -1 };
                        i += 1;
                    }
                    if count > 0 { instructions.push(Instruction::Add { count: count as u64, offset: 0 }); }
                    else if count < 0 { instructions.push(Instruction::Sub { count: count.abs() as u64, offset: 0 }); }
                    continue;
                }
                '>' | '<' => {
                    let mut shift: i64 = 0;
                    while i < chars.len() && (chars[i] == '>' || chars[i] == '<') {
                        shift += if chars[i] == '>' { 1 } else { -1 };
                        i += 1;
                    }
                    if shift > 0 { instructions.push(Instruction::Right(shift as u64)); }
                    else if shift < 0 { instructions.push(Instruction::Left(shift.abs() as u64)); }
                    continue;
                }
                '.' => instructions.push(Instruction::Print(1)),
                ',' => instructions.push(Instruction::Read(1)),
                '[' => instructions.push(Instruction::JumpToRight()),
                ']' => {
                    let len = instructions.len();
                    if len >= 2 
                        && instructions[len - 1] == (Instruction::Sub { count: 1, offset: 0 }) 
                        && instructions[len - 2] == Instruction::JumpToRight() 
                    {
                        instructions.pop();
                        instructions.pop();
                        instructions.push(Instruction::Reset());
                    } else {
                        instructions.push(Instruction::JumpToLeft());
                    }
                },
                _ => {}
            }
            i += 1;
        }
        
        InstructionList::new(instructions)
    }
}