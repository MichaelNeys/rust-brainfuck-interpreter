
#[derive(PartialEq, Debug)]
pub enum Instruction {
    Right(u64), // move pointer to right
    Left(u64), // move ponter to left
    Add{count: u64, offset: i32}, // add 1 to cell at offest from pointer
    Sub{count: u64, offset: i32}, // subtract 1 from cell at offest form pointer
    Print(u64), // prints the current memory cell as ascii
    Read(u64), // reads input to the current memory cell
    JumpToLeft(), // Jump to the next JumpToRight if current cell is 0
    JumpToRight(), // Jump to the next JumpToLeft if current cell is not 0
    FindEmptyRight(u64), // set pointer to the first empty cell to the right
    FindEmptyLeft(u64), // set pointer to the first empty cell to the left
    Reset(), // reset current cell
}




#[derive(Debug, PartialEq)]
pub struct InstructionList{
    current_instruction: usize,
    instructions: Vec<Instruction>
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

    pub fn execute_jump_to_left(&mut self) {
        while !matches!(self.instructions[self.current_instruction], Instruction::JumpToRight()){
            if self.current_instruction == 0{
                panic!("Could not find '[' before beginning of program!")
            }
            self.current_instruction -= 1;
        }
    }

    pub fn execute_jump_to_right(&mut self){
        while !matches!(self.instructions[self.current_instruction], Instruction::JumpToLeft()){
            if self.current_instruction >= self.instructions.len() - 1{
                panic!("Could not find ']' before end of program!")
            }
            self.current_instruction += 1;
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
        assert_eq!(InstructionList::new(vec![Instruction::Add { count: 5, offset: 0 }]), InstructionList{current_instruction: 0, instructions: vec![Instruction::Add { count: 5, offset: 0 }]});
    }

    #[test]
    fn test_next_instruction(){
        let mut list = InstructionList::new(vec![Instruction::Add { count: 5, offset: 0 }, Instruction::Add { count: 2, offset: 0 }]);
        assert_eq!(list.next_instruction(), Some(&Instruction::Add { count: 5, offset: 0 }));
        assert_eq!(list.next_instruction(), Some(&Instruction::Add { count: 2, offset: 0 }));
    }

    #[test]
    fn test_jump_to_right(){
        let mut list = InstructionList::new(vec![Instruction::Add { count: 5, offset: 0 }, Instruction::Add { count: 2, offset: 0 }, Instruction::JumpToLeft(), Instruction::Print(1)]);
        list.execute_jump_to_right();
        assert_eq!(list.next_instruction(), Some(&Instruction::JumpToLeft()))
    }

    #[test]
    fn test_jump_to_left(){
        let mut list = InstructionList::new(vec![Instruction::JumpToRight(), Instruction::Add { count: 5, offset: 0 }, Instruction::Add { count: 2, offset: 0 }, Instruction::Print(1)]);
        list.next_instruction();
        list.next_instruction();
        list.execute_jump_to_left();
        assert_eq!(list.next_instruction(), Some(&Instruction::JumpToRight()))
    }
}