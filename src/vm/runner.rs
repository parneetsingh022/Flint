#[derive(Copy, Clone, Debug)]
pub enum Value{
    Int(i32),
    Float(f64)
}


pub struct VirtualMachine{
    pub code       : Vec<u8>,
    pub ip         : usize,
    pub stack      : Vec<Value>,
    pub locals     : Vec<Value>,
    pub constants  : Vec<Value>
}

impl VirtualMachine{
    pub fn new(code : Vec<u8>) -> Self{
        Self{
            code,
            ip: 0,
            stack:  Vec::with_capacity(1024),
            locals: Vec::new(),
            constants: Vec::new(),
        }
    }

    /// Reads the next byte from the bytecode and advances the instruction pointer.
    pub fn fetch(&mut self) -> u8 {
        let instruction = self.code[self.ip];

        //self.ip += 1;
    
        instruction
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_initialization() {
        let code = vec![0x01, 0x02, 0x03];
        let vm = VirtualMachine::new(code.clone());
        
        assert_eq!(vm.code, code);
        assert_eq!(vm.ip, 0);
        assert!(vm.stack.is_empty());
    }

    #[test]
    fn test_fetch_increments_ip() {
        let code = vec![10, 20, 30];
        let mut vm = VirtualMachine::new(code);

        assert_eq!(vm.fetch(), 10);
        assert_eq!(vm.ip, 1);
        
        assert_eq!(vm.fetch(), 20);
        assert_eq!(vm.ip, 2);
        
        assert_eq!(vm.fetch(), 30);
        assert_eq!(vm.ip, 3);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_fetch_past_end_panics() {
        let mut vm = VirtualMachine::new(vec![1]);
        vm.fetch(); // IP is now 1
        vm.fetch(); // This should panic because index 1 doesn't exist
    }
}