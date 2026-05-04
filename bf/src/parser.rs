use crate::instruction::{Instruction, InstructionList};

pub struct Parser;

impl Parser {

    fn naive_parse(code: &str) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        let chars: Vec<char> = code.chars().filter(|c| "+-<>.,[]".contains(*c)).collect();

        for char in chars {
            match char{
                '+' => instructions.push(Instruction::Add {count: 1}),
                '-' => instructions.push(Instruction::Add {count: -1}),
                '>' => instructions.push(Instruction::Move(1)),
                '<' => instructions.push(Instruction::Move(-1)),
                '.' => instructions.push(Instruction::Print()),
                ',' => instructions.push(Instruction::Read()),
                '[' => instructions.push(Instruction::JumpToRight()),
                ']' => instructions.push(Instruction::JumpToLeft()),
                _ => {}
            }
        }
        instructions
    }

    fn collapse_add(instructions: &[Instruction]) -> Vec<Instruction>{
        instructions.iter().fold(vec![], |mut acc, new| {
            match acc.last_mut() {
                Some(Instruction::Add { count: last_count}) => {
                    match new {
                        Instruction::Add {count} => {
                            // merge
                            *last_count = *last_count + count;
                        }
                        _ => acc.push(*new),
                    }
                }
                _ => {acc.push(*new)}
            }

            acc
        })
    }

    fn collapse_move(instructions: &[Instruction]) -> Vec<Instruction>{
        instructions.iter().fold(vec![], |mut acc, new| {
            match acc.last_mut() {
                Some(Instruction::Move(last_count)) => {
                    match new {
                        Instruction::Move (count) => {
                            *last_count = *last_count + count;
                        }
                        _ => acc.push(*new),
                    }
                }
                _ => {acc.push(*new)}
            }

            acc
        })
    }

    fn de_loop(instructions: &[Instruction]) -> Vec<Instruction>{
        instructions.iter().fold(vec![], |mut acc, new| {
            match new{
                Instruction::JumpToLeft() => {
                    let len = acc.len();
                    if len >= 2{
                        match (acc[len-2], acc[len-1]) {
                            (Instruction::JumpToRight(), Instruction::Add { count: -1 }) => {
                                acc.pop();
                                acc.pop();
                                acc.push(Instruction::Reset());
                            }
                            (Instruction::JumpToRight(), Instruction::Move(1)) => {
                                acc.pop();
                                acc.pop();
                                acc.push(Instruction::FindEmptyRight(1));
                            }
                            (Instruction::JumpToRight(), Instruction::Move(-1)) => {
                                acc.pop();
                                acc.pop();
                                acc.push(Instruction::FindEmptyLeft(1));
                            }
                            _ => {
                                acc.push(*new)
                            }
                        }
                    }else{
                        acc.push(*new);
                    }

                }
                _ => acc.push(*new)
            }

            acc
        })
    }

    fn de_loop_copy(instructions: &[Instruction]) -> Vec<Instruction>{
        let mut in_loop = false;
        let mut loop_start: usize = 0;
        let mut move_count = 0;
        let mut source_dec = 0;
        let mut copy_offsets: Vec<(i32, i64)> = vec![];
        let mut invalidated = false;
        let mut pushed = false;


        let mut result: Vec<Instruction> = vec![];

        let mut i = 0;

        while i < instructions.len(){
            match instructions[i] {
                Instruction::JumpToRight() => {
                    in_loop = true;
                    loop_start = result.len(); // starting instruction has not been added yet
                    move_count = 0;
                    invalidated = false;
                    copy_offsets = vec![];
                    source_dec = 0;
                }
                Instruction::Move(count) => {
                    move_count += count;
                }
                Instruction::Add {count} => {
                    if move_count == 0 {
                        source_dec += count;
                    }else{
                        copy_offsets.push((move_count as i32, count));
                    }

                }
                Instruction::JumpToLeft() => {
                    if in_loop && move_count == 0 && copy_offsets.len() > 0 && !invalidated && source_dec == -1 {
                        pushed = true;

                        result.drain(loop_start..result.len());
                        for (offset, count) in &copy_offsets{
                            result.push(Instruction::Copy {offset: *offset, multiplier: *count});
                        }
                        result.push(Instruction::Reset());
                    }

                    in_loop = false;
                    move_count = 0;
                    invalidated = false;
                    copy_offsets = vec![];
                    source_dec = 0;
                }
                _ => {
                    invalidated = true;
                }
            }
            if !pushed {
                result.push(instructions[i]);
            }else{
                pushed = false;
            }
            i+=1;
        }

        result
    }

    pub fn parse(code: &str) -> InstructionList {
        let mut instructions = Self::naive_parse(code);
        instructions = Self::collapse_add(&instructions);
        instructions = Self::collapse_move(&instructions);
        instructions = Self::de_loop(&instructions);
        instructions = Self::de_loop_copy(&instructions);
        /*while i < chars.len() {
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
                    if len >= 2 {
                        match (&instructions[len - 2], &instructions[len - 1]) {
                            (Instruction::JumpToRight(), Instruction::Sub { count: 1, offset: 0 }) => {
                                instructions.pop();
                                instructions.pop();
                                instructions.push(Instruction::Reset());
                            }
                            (Instruction::JumpToRight(), Instruction::Right(1)) => {
                                instructions.pop();
                                instructions.pop();
                                instructions.push(Instruction::FindEmptyRight(1));
                            }
                            (Instruction::JumpToRight(), Instruction::Left(1)) => {
                                instructions.pop();
                                instructions.pop();
                                instructions.push(Instruction::FindEmptyLeft(1));
                            }
                            _ => {
                                instructions.push(Instruction::JumpToLeft());
                            }
                        }
                    } else {
                        instructions.push(Instruction::JumpToLeft());
                    }
                },
                _ => {}
            }
            i += 1;
        }*/
        
        InstructionList::new(instructions)
    }
}