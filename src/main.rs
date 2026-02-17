mod vm;

use crate::vm::opcodes::op_codes;
use crate::vm::runner::VirtualMachine;

fn main() {
    let code : Vec<u8> = vec![op_codes::NOP, op_codes::IPUSH, op_codes::IPOP];
    let vm = VirtualMachine::new(code);
}
