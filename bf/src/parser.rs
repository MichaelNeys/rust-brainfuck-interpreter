use crate::instruction::{Instruction, InstructionList};
use crate::instruction::Instruction::CopyChunk;

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
                '.' => instructions.push(Instruction::Print(1)),
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

    fn collapse_print(instructions: &[Instruction]) -> Vec<Instruction>{
        instructions.iter().fold(vec![], |mut acc, new| {
            match acc.last_mut() {
                Some(Instruction::Print(last_count)) => {
                    match new {
                        Instruction::Print (count) => {
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
    
    fn collapse_chunk_copy(instructions: &[Instruction]) -> Vec<Instruction> {
        let mut i = 0;
        let mut result: Vec<Instruction> = vec![];

        while i < instructions.len() {
            if let Instruction::Copy { offset, multiplier } = instructions[i] {
                let mut temp_i = i;
                let mut length = 0;
                let mut matched = false;

                while temp_i + 1 < instructions.len() {
                    if temp_i + 2 < instructions.len()
                        && instructions[temp_i] == (Instruction::Copy { offset, multiplier })
                        && instructions[temp_i + 1] == Instruction::Reset()
                        && instructions[temp_i + 2] == Instruction::Move(1)
                    {
                        length += 1;
                        temp_i += 3;
                    } else if instructions[temp_i] == (Instruction::Copy { offset, multiplier })
                        && instructions[temp_i + 1] == Instruction::Reset()
                    {
                        length += 1;
                        temp_i += 2;
                        matched = true;
                        break;
                    } else {
                        break;
                    }
                }

                if matched {
                    result.push(Instruction::CopyChunk { offset, length, multiplier });
                    // CopyChunk does not include moving the pointer so we add it (this will often be optimized away in the next pass)
                    result.push(Instruction::Move(length as i64 - 1));
                    i = temp_i;
                    continue;
                }
            }

            result.push(instructions[i]);
            i += 1;
        }
        result
}

    fn collapse_find_empty(instructions: &[Instruction]) -> Vec<Instruction> {
        instructions.iter().fold(vec![], |mut acc, new| {
            match new {
                Instruction::FindEmptyLeft() => {
                    match acc.last_mut() {
                        Some(Instruction::FindEmptyLeft()) => {}
                        _ => acc.push(*new),
                    }
                }
                Instruction::FindEmptyRight() => {
                    match acc.last_mut() {
                        Some(Instruction::FindEmptyRight()) => {}
                        _ => acc.push(*new),
                    }
                }
                _ => acc.push(*new)
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
                                acc.push(Instruction::FindEmptyRight());
                            }
                            (Instruction::JumpToRight(), Instruction::Move(-1)) => {
                                acc.pop();
                                acc.pop();
                                acc.push(Instruction::FindEmptyLeft());
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

    fn remove_redundant(instructions: &[Instruction]) -> Vec<Instruction> {
        instructions.iter().fold(vec![], |mut acc, new| {
            match new {
                Instruction::Add { count } if *count == 0 => acc,
                Instruction::Move(count) if *count == 0 => acc,
                Instruction::Copy { offset:_, multiplier } if *multiplier == 0 => acc,
                _ => {
                    acc.push(*new);
                    acc
                }
            }
        })
    }

    pub fn parse(code: &str) -> InstructionList {
        let mut instructions = Self::naive_parse(code);

        let mut old_len = instructions.len() + 1;

        while old_len > instructions.len() {
            old_len = instructions.len();
            instructions = Self::collapse_add(&instructions);
            instructions = Self::collapse_move(&instructions);
            instructions = Self::collapse_print(&instructions);
            instructions = Self::collapse_find_empty(&instructions);
            instructions = Self::de_loop(&instructions);
            instructions = Self::de_loop_copy(&instructions);
            instructions = Self::remove_redundant(&instructions);
            instructions = Self::collapse_chunk_copy(&instructions);
        }


        
        InstructionList::new(instructions)
    }
}