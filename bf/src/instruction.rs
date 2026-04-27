enum Instruction {
    Right(u32), // move pointer to right
    Left(u32), // move ponter to left
    Add{count: u32, offset: u32}, // add 1 to cell at offest from pointer
    Sub{count: u32, offset: u32}, // subtract 1 from cell at offest form pointer
    Print(u32), // prints the current memory cell as ascii
    Read(u32), // reads input to the current memory cell
    JumpLeft(u32), // Jump to the next JumpRight if current cell is 0
    JumpRight(u32), // Jump to the next JumpLeft if current cell is not 0
    FindEmptyRight(u32), // set pointer to the first empty cell to the right
    FindEmptyLeft(u32), // set pointer to the first empty cell to the left
    Reset(), // reset current cell
}