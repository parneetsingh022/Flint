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
}