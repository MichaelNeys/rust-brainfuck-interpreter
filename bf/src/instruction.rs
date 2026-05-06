
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Instruction {
    Move(i64), // move pointer to right
    Add{count: i64}, // add count to cell at offest from pointer
    Print(u64), // prints the current memory cell as ascii
    Read(), // reads input to the current memory cell
    JumpToLeft(), // Jump to the next JumpToRight if current cell is 0
    JumpToRight(), // Jump to the next JumpToLeft if current cell is not 0
    FindEmptyRight(), // set pointer to the first empty cell to the right
    FindEmptyLeft(), // set pointer to the first empty cell to the left
    Reset(), // reset current cell
    Copy{offset: i32, multiplier: i64}, // copy the current cell value into offset
}




#[derive(Debug, PartialEq)]
pub struct InstructionList{
    pub current_instruction: usize,
    pub instructions: Vec<Instruction>
}


impl InstructionList{

    pub fn new(instructions: Vec<Instruction>) -> InstructionList{
        InstructionList{current_instruction: 0, instructions: instructions}
    }


    pub fn next_instruction(&mut self) -> Option<&Instruction>{
        let instruction = self.instructions.get(self.current_instruction);
        self.current_instruction += 1;
        instruction
    }

    pub fn execute_jump_to_right(&mut self) {
        let mut depth = 1;
        while depth > 0 {
            if self.current_instruction >= self.instructions.len() {
                panic!("Unbalanced brackets: could not find ']'");
            }
            match self.instructions[self.current_instruction] {
                Instruction::JumpToRight() => depth += 1,
                Instruction::JumpToLeft() => depth -= 1,
                _ => {}
            }
            if depth > 0 { self.current_instruction += 1; }
        }
    }

    pub fn execute_jump_to_left(&mut self) {
        self.current_instruction -= 2; 
        let mut depth = 1;
        while depth > 0 {
            match self.instructions[self.current_instruction] {
                Instruction::JumpToLeft() => depth += 1,
                Instruction::JumpToRight() => depth -= 1,
                _ => {}
            }
            if depth > 0 {
                if self.current_instruction == 0 {
                    panic!("Unbalanced brackets: could not find '['");
                }
                self.current_instruction -= 1;
            }
        }
    }

    pub fn is_at_end(&self) -> bool{
        self.current_instruction >= self.instructions.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::{Instruction, InstructionList};

    #[test]
    fn test_new() {
        assert_eq!(InstructionList::new(vec![]), InstructionList{current_instruction: 0, instructions: vec![]});
        assert_eq!(InstructionList::new(vec![Instruction::Add { count: 5}]), InstructionList{current_instruction: 0, instructions: vec![Instruction::Add { count: 5 }]});
    }

    #[test]
    fn test_next_instruction(){
        let mut list = InstructionList::new(vec![Instruction::Add { count: 5 }, Instruction::Add { count: 2}]);
        assert_eq!(list.next_instruction(), Some(&Instruction::Add { count: 5}));
        assert_eq!(list.next_instruction(), Some(&Instruction::Add { count: 2}));
    }

    #[test]
    fn test_jump_to_right(){
        let mut list = InstructionList::new(vec![Instruction::Add { count: 5 }, Instruction::Add { count: 2 }, Instruction::JumpToLeft(), Instruction::Print()]);
        list.execute_jump_to_right();
        assert_eq!(list.next_instruction(), Some(&Instruction::JumpToLeft()))
    }

    #[test]
    fn test_jump_to_left(){
        let mut list = InstructionList::new(vec![Instruction::JumpToRight(), Instruction::Add { count: 5 }, Instruction::Add { count: 2}, Instruction::Print()]);
        list.next_instruction();
        list.next_instruction();
        list.execute_jump_to_left();
        assert_eq!(list.next_instruction(), Some(&Instruction::JumpToRight()))
    }
}