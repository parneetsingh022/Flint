use crate::vm::opcodes::op;
use crate::vm::header::Header;

macro_rules! read_bytes {
    ($self:ident, $ty:ty) => {{
        let size = std::mem::size_of::<$ty>();
        let start = $self.ip;
        let end = $self.ip + size;
        
        let bytes = &$self.code[start..end];
        let value = <$ty>::from_be_bytes(bytes.try_into().expect("Bytecode ended prematurely"));
        
        $self.ip += size; // Automatically advance the instruction pointer
        value
    }};
}



#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Value{
    Int(i32),
    Float(f64),
    Char(u8),
}


pub struct VirtualMachine{
    pub code       : Vec<u8>,
    pub ip         : usize,
    pub stack      : Vec<Value>,
    pub memory     : Vec<Value>,
    pub constants  : Vec<Value>,
    pub running    : bool
}

impl VirtualMachine{
    pub fn new(code : Vec<u8>) -> Self{
        let header = Header::from_bytes(&code).expect("Failed to parse Flint header");
        Self{
            code,
            ip: (header.code_start as usize)+1,
            stack:  Vec::with_capacity(1024),
            memory: Vec::new(),
            constants: Vec::new(),
            running: true
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

    fn compare_f64(&self, v1: f64, v2: f64) -> i32 {
        if v1 < v2 {
            -1
        } else if v1 > v2 {
            1
        } else if v1 == v2 {
            0
        } else {
            // This happens if v1 or v2 is NaN
            // We return -2 to signal "Unordered/Error"
            -2 
        }
    }
    
    /// Executes the virtual machine
    pub fn execute(&mut self){
        let mut cur_op : u8;
        while self.ip < self.code.len() && self.running {
            cur_op = self.fetch();

            match cur_op {
                op::NOP => continue,
                op::HALT => {self.running = false},
                op::IPUSH => self.handle_ipush(),
                op::FPUSH => self.handle_fpush(),
                op::POP => self.handle_pop(),
                op::BIPUSH => self.handle_bipush(),
                op::SWP => self.handle_swp(),
                op::DUP => self.handle_dup(),
                op::NEG => self.handle_neg(),
                op::ADD => self.handle_add(),
                op::SUB => self.handle_sub(),
                op::MUL => self.handle_mul(),
                op::DIV => self.handle_div(),
                op::MOD => self.handle_mod(),
                op::CMP => self.handle_cmp(),
                op::JL  => self.handle_jl(),
                op::JLE  => self.handle_jle(),
                op::JG  => self.handle_jg(),
                op::JGE  => self.handle_jge(),
                op::JE  => self.handle_je(),
                op::JNE  => self.handle_jne(),
                op::JMP  => self.handle_jmp(),
                op::STORE => self.handle_store(),
                op::LOAD => self.handle_load(),
                op::PRINT => self.handle_print(),
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

    pub fn handle_fpush(&mut self) {
        let start = self.ip;
        let end = self.ip + 8;
        let bytes = &self.code[start..end];

        // Convert 8 bytes to f64 (using Big Endian)
        let value = f64::from_be_bytes(bytes.try_into().expect("Bytecode ended prematurely"));

        // Move the IP forward by 8 bytes
        self.ip += 8;

        self.push(Value::Float(value));
    }

    pub fn handle_pop(&mut self){
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

    pub fn handle_mul(&mut self) {
        let (b, a) = (self.pop(), self.pop());
        let result = match (a, b) {
            (Value::Int(v1), Value::Int(v2)) => Value::Int(v1 * v2),
            (Value::Float(v1), Value::Float(v2)) => Value::Float(v1 * v2),
            (Value::Int(v1), Value::Float(v2)) => Value::Float(v1 as f64 * v2),
            (Value::Float(v1), Value::Int(v2)) => Value::Float(v1 * v2 as f64),
            _ => panic!("Type error: Multiplication only supported for numeric types"),
        };
        self.push(result);
    }

    pub fn handle_div(&mut self) {
        let b = self.pop();
        let a = self.pop();

        let result = match (a, b) {
            (Value::Int(v1), Value::Int(v2)) => {
                if v2 == 0 { panic!("Runtime Error: Division by zero"); }
                Value::Int(v1 / v2)
            }
            (Value::Float(v1), Value::Float(v2)) => {
                if v2 == 0.0 { panic!("Runtime Error: Division by zero"); }
                Value::Float(v1 / v2)
            }
            (Value::Int(v1), Value::Float(v2)) => {
                if v2 == 0.0 { panic!("Runtime Error: Division by zero"); }
                Value::Float(v1 as f64 / v2)
            }
            (Value::Float(v1), Value::Int(v2)) => {
                if v2 == 0 { panic!("Runtime Error: Division by zero"); }
                Value::Float(v1 / v2 as f64)
            }
            _ => panic!("Type error: Division only supported for numeric types"),
        };
        self.push(result);
    }

    pub fn handle_mod(&mut self) {
        let b = self.pop();
        let a = self.pop();

        let result = match (a, b) {
            (Value::Int(v1), Value::Int(v2)) => {
                if v2 == 0 { panic!("Runtime Error: Integer modulo by zero"); }
                Value::Int(v1 % v2)
            }
            (Value::Float(v1), Value::Float(v2)) => {
                if v2 == 0.0 { panic!("Runtime Error: Float modulo by zero"); }
                Value::Float(v1 % v2)
            }
            (Value::Int(v1), Value::Float(v2)) => {
                if v2 == 0.0 { panic!("Runtime Error: Float modulo by zero"); }
                Value::Float(v1 as f64 % v2)
            }
            (Value::Float(v1), Value::Int(v2)) => {
                if v2 == 0 { panic!("Runtime Error: Integer modulo by zero"); }
                Value::Float(v1 % v2 as f64)
            }

            _ => panic!("Type error: Modulo only supported for numeric types"),
        };
        self.push(result);
    }

    pub fn handle_cmp(&mut self) {
        let b = self.pop();
        let a = self.pop();

        let res = match (a, b) {
            // Integer vs Integer
            (Value::Int(v1), Value::Int(v2)) => {
                if v1 < v2 { -1 } else if v1 > v2 { 1 } else { 0 }
            }
            // Float vs Float
            (Value::Float(v1), Value::Float(v2)) => self.compare_f64(v1, v2),
            // Mixed: Int vs Float
            (Value::Int(v1), Value::Float(v2)) => self.compare_f64(v1 as f64, v2),
            // Mixed: Float vs Int
            (Value::Float(v1), Value::Int(v2)) => self.compare_f64(v1, v2 as f64),
            
            _ => panic!("Type error: CMP only supported for numeric types"),
        };

        self.push(Value::Int(res));
    }

    pub fn handle_jl(&mut self) {
        let address = read_bytes!(self, u32);

        if let Value::Int(cmp_result) = self.pop() {
            if cmp_result == -1 {
                self.ip = address as usize;
            }
        } else {
            panic!("Type error: JL expects an integer on the stack from a CMP operation");
        }
    }


    pub fn handle_jle(&mut self) {
        let address = read_bytes!(self, u32);

        if let Value::Int(cmp_result) = self.pop() {
            if cmp_result == -1 || cmp_result == 0 {
                self.ip = address as usize;
            }
        } else {
            panic!("Type error: JLE expects an integer on the stack from a CMP operation");
        }
    }

    pub fn handle_jg(&mut self) {
        let address = read_bytes!(self, u32);

        if let Value::Int(cmp_result) = self.pop() {
            if cmp_result == 1 {
                self.ip = address as usize;
            }
        } else {
            panic!("Type error: JG expects an integer on the stack from a CMP operation");
        }
    }


    pub fn handle_jge(&mut self) {
        let address = read_bytes!(self, u32);

        if let Value::Int(cmp_result) = self.pop() {
            if cmp_result == 1 || cmp_result == 0 {
                self.ip = address as usize;
            }
        } else {
            panic!("Type error: JGE expects an integer on the stack from a CMP operation");
        }
    }

    pub fn handle_je(&mut self) {
        let address = read_bytes!(self, u32);

        if let Value::Int(cmp_result) = self.pop() {
            if cmp_result == 0  {
                self.ip = address as usize;
            }
        } else {
            panic!("Type error: JE expects an integer on the stack from a CMP operation");
        }
    }

    pub fn handle_jne(&mut self) {
        let address = read_bytes!(self, u32);

        if let Value::Int(cmp_result) = self.pop() {
            if cmp_result != 0  {
                self.ip = address as usize;
            }
        } else {
            panic!("Type error: JNE expects an integer on the stack from a CMP operation");
        }
    }

    pub fn handle_jmp(&mut self) {
        let address = read_bytes!(self, u32);
        self.ip = address as usize;
    }

    pub fn handle_store(&mut self) {
        let address = read_bytes!(self, u32) as usize;

        if let Some(value) = self.stack.pop() {
            if address >= self.memory.len() {
                self.memory.resize(address + 1, Value::Int(0));
            }
            self.memory[address] = value;
        } else {
            panic!("Runtime Error: Stack underflow during STORE");
        }
    }


    pub fn handle_load(&mut self) {
        let address = read_bytes!(self, u32) as usize;
        if address < self.memory.len() {
            let value = self.memory[address];
            self.stack.push(value);
        } else {
            eprintln!("Runtime Error: Access to uninitialized or out-of-bounds address: {}", address);
            self.running = false;
        }
    }

    pub fn handle_print(&mut self) {
        let item = self.pop();
        
        match item {
            Value::Int(val) => {
                println!("{}", val);
            }
            Value::Float(val) => {
                println!("{:.2}", val); 
            }
            Value::Char(c) => {
                println!("{}", c as char);
            }
        }
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