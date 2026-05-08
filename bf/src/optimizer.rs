use crate::instruction::{Instruction, InstructionList};


pub struct Optimizer;


impl Optimizer {
    fn collapse_add(instructions: &[Instruction]) -> Vec<Instruction> {
        instructions.iter().fold(vec![], |mut acc, new| {
            match acc.last_mut() {
                Some(Instruction::Add {
                         count: last_count,
                         offset: last_offset,
                     }) => {
                    match new {
                        Instruction::Add { count, offset } if *offset == *last_offset => {
                            // merge
                            *last_count += count;
                        }
                        _ => acc.push(*new),
                    }
                }
                _ => acc.push(*new),
            }

            acc
        })
    }

    fn collapse_move(instructions: &[Instruction]) -> Vec<Instruction> {
        instructions.iter().fold(vec![], |mut acc, new| {
            match acc.last_mut() {
                Some(Instruction::Move(last_count)) => match new {
                    Instruction::Move(count) => {
                        *last_count += count;
                    }
                    _ => acc.push(*new),
                },
                _ => acc.push(*new),
            }

            acc
        })
    }

    fn collapse_print(instructions: &[Instruction]) -> Vec<Instruction> {
        instructions.iter().fold(vec![], |mut acc, new| {
            match acc.last_mut() {
                Some(Instruction::Print(last_count)) => match new {
                    Instruction::Print(count) => {
                        *last_count += count;
                    }
                    _ => acc.push(*new),
                },
                _ => acc.push(*new),
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
                    result.push(Instruction::CopyChunk {
                        offset,
                        length,
                        multiplier,
                    });
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

    fn collapse_move_add_move(instructions: &[Instruction]) -> Vec<Instruction> {
        let mut i = 0;
        let mut result: Vec<Instruction> = vec![];

        while i < instructions.len() {
            if let Instruction::Move(first_count) = instructions[i]
                && i + 2 < instructions.len()
            {
                match (instructions[i + 1], instructions[i + 2]) {
                    (Instruction::Add { count, offset }, Instruction::Move(second_count))
                    if first_count == -second_count =>
                        {
                            // we can just use offset instead of moving twice
                            result.push(Instruction::Add {
                                count,
                                offset: offset + first_count,
                            });
                            // go to the next insruction
                            i += 3;
                            continue;
                        }
                    _ => {}
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
                Instruction::FindEmptyLeft() => match acc.last_mut() {
                    Some(Instruction::FindEmptyLeft()) => {}
                    _ => acc.push(*new),
                },
                Instruction::FindEmptyRight() => match acc.last_mut() {
                    Some(Instruction::FindEmptyRight()) => {}
                    _ => acc.push(*new),
                },
                _ => acc.push(*new),
            }
            acc
        })
    }

    fn de_loop(instructions: &[Instruction]) -> Vec<Instruction> {
        instructions.iter().fold(vec![], |mut acc, new| {
            match new {
                Instruction::JumpToLeft() => {
                    let len = acc.len();
                    if len >= 2 {
                        match (acc[len - 2], acc[len - 1]) {
                            (
                                Instruction::JumpToRight(),
                                Instruction::Add {
                                    count: -1,
                                    offset: 0,
                                },
                            ) => {
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
                            _ => acc.push(*new),
                        }
                    } else {
                        acc.push(*new);
                    }
                }
                _ => acc.push(*new),
            }

            acc
        })
    }

    fn de_loop_copy(instructions: &[Instruction]) -> Vec<Instruction> {
        let mut in_loop = false;
        let mut loop_start: usize = 0;
        let mut total_moves = 0;
        let mut source_dec = 0;
        let mut copy_offsets: Vec<(i64, i64)> = vec![];
        let mut invalidated = false;
        let mut pushed = false;

        let mut result: Vec<Instruction> = vec![];

        let mut i = 0;

        while i < instructions.len() {
            match instructions[i] {
                Instruction::JumpToRight() => {
                    in_loop = true;
                    loop_start = result.len(); // starting instruction has not been added yet
                    total_moves = 0;
                    invalidated = false;
                    copy_offsets = vec![];
                    source_dec = 0;
                }
                Instruction::Move(count) => {
                    total_moves += count;
                }
                Instruction::Add { count, offset: 0 } => {
                    if total_moves == 0 {
                        source_dec += count;
                    } else {
                        copy_offsets.push((total_moves, count));
                    }
                }
                Instruction::JumpToLeft() => {
                    if in_loop
                        && total_moves == 0
                        && !copy_offsets.is_empty()
                        && !invalidated
                        && source_dec == -1
                    {
                        pushed = true;

                        result.drain(loop_start..result.len());
                        for (offset, count) in &copy_offsets {
                            result.push(Instruction::Copy {
                                offset: *offset,
                                multiplier: *count,
                            });
                        }
                        result.push(Instruction::Reset());
                    }

                    in_loop = false;
                    total_moves = 0;
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
            } else {
                pushed = false;
            }
            i += 1;
        }

        result
    }

    fn remove_redundant(instructions: &[Instruction]) -> Vec<Instruction> {
        instructions.iter().fold(vec![], |mut acc, new| match new {
            Instruction::Add {
                count: 0,
                offset: _,
            } => acc,
            Instruction::Move(0) => acc,
            Instruction::Copy {
                offset: _,
                multiplier,
            } if *multiplier == 0 => acc,
            _ => {
                acc.push(*new);
                acc
            }
        })
    }

    pub fn optimize(instructions: Vec<Instruction>) -> Vec<Instruction> {
        let mut instructions = instructions;


        let mut old_list = vec![];

        while old_list != instructions {
            old_list = instructions.clone();
            instructions = Self::collapse_add(&instructions);
            instructions = Self::collapse_move(&instructions);
            instructions = Self::collapse_print(&instructions);
            instructions = Self::collapse_find_empty(&instructions);
            instructions = Self::de_loop(&instructions);
            instructions = Self::de_loop_copy(&instructions);
            instructions = Self::remove_redundant(&instructions);
            instructions = Self::collapse_chunk_copy(&instructions);
            instructions = Self::collapse_move_add_move(&instructions);
        }

        instructions
    }

}

