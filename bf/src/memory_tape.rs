

#[derive(PartialEq, Debug)]
pub struct MemoryTape{
    positive_data: Vec<u8>,
    negative_data: Vec<u8>,

    pointer: i64
}




impl MemoryTape{
    pub fn new() -> MemoryTape{
        MemoryTape{ positive_data: vec![], negative_data: vec![], pointer: 0}
    }


    pub fn set_pointer(&mut self, pointer: i64) {
        self.pointer = pointer
    }
    pub fn set_at_pointer(&mut self, value: u8, offset: i32){
        let offset_pointer = self.pointer - offset as i64;

        if self.pointer >= 0{
            // positive list
            if offset_pointer as usize >= self.positive_data.len() {
                self.positive_data.resize(offset_pointer as usize + 1, 0);
            }
            self.positive_data[offset_pointer as usize] = value;
        }else{
            let location: usize = (-offset_pointer as usize) - 1;
            // negative list
            if location >= self.positive_data.len() {
                self.negative_data.resize(location + 1, 0);
            }
            self.negative_data[location] = value
        }
    }

    pub fn move_pointer_left(&mut self){
        self.pointer -= 1;
    }

    pub fn move_pointer_right(&mut self){
        self.pointer += 1;
    }

    pub fn get_at_pointer(&self, offset: i32) -> u8{

        let offset_pointer = self.pointer - offset as i64;

        if offset_pointer >= 0{
            // positive list
            if offset_pointer as usize >= self.positive_data.len(){
                0
            }else{
                self.positive_data[offset_pointer as usize]
            }
        }else{
            let location: usize = (-offset_pointer as usize) - 1;
            // negative list
            if location >= self.negative_data.len(){
                0
            }else{
                self.negative_data[location]
            }
        }
    }

    pub fn add_at_pointer(&mut self, offset: i32){
        self.set_at_pointer(self.get_at_pointer(offset).wrapping_add(1), offset);
    }

    pub fn subtract_at_pointer(&mut self, offset: i32){
        self.set_at_pointer(self.get_at_pointer(offset).wrapping_sub(1), offset);
    }
}

#[cfg(test)]
mod tests{
    use crate::memory_tape::MemoryTape;

    #[test]
    fn test_new(){
        assert_eq!(MemoryTape::new(), MemoryTape{positive_data: vec![], negative_data: vec![], pointer: 0});
    }

    #[test]
    fn test_move_pointer(){
        let mut tape = MemoryTape::new();
        tape.move_pointer_left();
        assert_eq!(tape.pointer, -1);
        tape.move_pointer_right();
        tape.move_pointer_right();
        assert_eq!(tape.pointer, 1);
    }
    #[test]
    fn test_get(){
        let mut tape = MemoryTape::new();
        assert_eq!(tape.get_at_pointer(0), 0);
        tape.move_pointer_right();
        assert_eq!(tape.get_at_pointer(0), 0);
    }


    #[test]
    fn test_add_subtract(){
        let mut tape = MemoryTape::new();
        tape.add_at_pointer(0);
        assert_eq!(tape.get_at_pointer(0), 1);
        tape.subtract_at_pointer(0);
        tape.subtract_at_pointer(0);
        assert_eq!(tape.get_at_pointer(0), 255);
    }

    #[test]
    fn test_new_value(){
        let mut tape = MemoryTape::new();
        tape.move_pointer_right();
        assert_eq!(tape.get_at_pointer(0), 0);
        tape.add_at_pointer(0);
        assert_eq!(tape.get_at_pointer(0), 1);
    }
}