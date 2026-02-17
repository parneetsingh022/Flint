#[derive(Copy, Clone, Debug, PartialEq)]
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

        self.ip += 1;
    
        instruction
    }

    /// Add an item in the stack
    pub fn push(&mut self, value : Value) {
        self.stack.push(value);
    }
    
    /// Removes and returns item from stack
    pub fn pop(&mut self) -> Value{
        self.stack.pop().expect("Stack underflow!")
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

    #[test]
    fn test_stack_push_pop_integrity(){
        let mut vm = VirtualMachine::new(vec![]);
        assert_eq!(vm.stack, vec![]);
        vm.push(Value::Int(10));
        vm.push(Value::Int(20));
        vm.push(Value::Float(43.2));
        assert_eq!(vm.stack, vec![Value::Int(10), Value::Int(20), Value::Float(43.2)]);
        assert_eq!(vm.pop(), Value::Float(43.2));
        assert_eq!(vm.pop(), Value::Int(20));
        vm.push(Value::Float(56.23));
        assert_eq!(vm.pop(), Value::Float(56.23));
        assert_eq!(vm.pop(), Value::Int(10));
        assert!(vm.stack.is_empty(), "Stack should be empty after all pops");
    }

    #[test]
    #[should_panic(expected = "Stack underflow!")]
    fn test_stack_underflow_panics() {
        let mut vm = VirtualMachine::new(vec![]);
        
        vm.pop();
    }
}