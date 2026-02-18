use crate::vm::opcodes::op;

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

    /// Executes the virtual machine
    pub fn execute(&mut self){
        let mut cur_op : u8;
        while self.ip < self.code.len(){
            cur_op = self.fetch();

            match cur_op {
                op::NOP => continue,
                op::HALT => break,
                op::IPUSH => self.handle_ipush(),
                op::IPOP => self.handle_ipop(),
                op::BIPUSH => self.handle_bipush(),
                op::SWP => self.handle_swp(),
                op::DUP => self.handle_dup(),
                op::NEG => self.handle_neg(),
                op::ADD => self.handle_add(),
                op::SUB => self.handle_sub(),
                _ => panic!("Unknown opcode: {}", cur_op),
            }
        }
    }


    pub fn handle_ipush(&mut self){
        let start = self.ip;
        let end = self.ip + 4;
        let bytes = &self.code[start..end];

        // Convert bytes to i32 (using Big Endian)
        let value = i32::from_be_bytes(bytes.try_into().expect("Bytecode ended prematurely"));

        // Move the IP forward by 4
        self.ip += 4;

        self.push(Value::Int(value));
    }

    pub fn handle_ipop(&mut self){
        self.pop();
    }

    pub fn handle_bipush(&mut self){
        let data = self.fetch() as i32;
        self.push(Value::Int(data));
    }
    pub fn handle_swp(&mut self){
        let a = self.pop();
        let b = self.pop();
        self.push(a);
        self.push(b);
    }

    pub fn handle_dup(&mut self){
        let a = self.pop();
        self.push(a);
        self.push(a);
    }

    pub fn handle_neg(&mut self){
        let a = self.pop();

        let result = match a  {
            Value::Int(v1) => Value::Int(-v1),
            Value::Float(v1) => Value::Float(-v1),
            _ => panic!("Type error: Negation only supported for integers and float"),
        };

        self.push(result);
    }

    pub fn handle_add(&mut self){
        let a = self.pop();
        let b = self.pop();

        let result = match(a,b) {
            (Value::Int(v1) , Value::Int(v2)) => Value::Int(v1+v2),
            (Value::Float(v1) , Value::Float(v2)) => Value::Float(v1+v2),
            (Value::Int(v1), Value::Float(v2)) => Value::Float(v1 as f64 + v2),
            (Value::Float(v1), Value::Int(v2)) => Value::Float(v1 + v2 as f64),
            _ => panic!("Type error: Addition only supported for integers and float"),
        };

        self.push(result);
    }

    pub fn handle_sub(&mut self){
        let a = self.pop();
        let b = self.pop();

        let result = match(a,b) {
            (Value::Int(v1) , Value::Int(v2)) => Value::Int(v2 - v1),
            (Value::Float(v1) , Value::Float(v2)) => Value::Float(v2 - v1),
            (Value::Int(v1), Value::Float(v2)) => Value::Float(v2 - v1 as f64),
            (Value::Float(v1), Value::Int(v2)) => Value::Float(v2 as f64 - v1),
            _ => panic!("Type error: Subtraction only supported for integers and float"),
        };

        self.push(result);
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