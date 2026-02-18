mod vm;

use crate::vm::opcodes::op;
use crate::vm::runner::VirtualMachine;
use crate::vm::disassembler::disassemble_bytecode;

fn main() {
    let code = bytecode!(
        IPUSH 100,
        BIPUSH 34,
        IPUSH 394,
        NOP,
        ADD
    );

    
    let dis = disassemble_bytecode(code);
    println!("{}",dis);

    
}
