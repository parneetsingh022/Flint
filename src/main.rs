mod vm;

use crate::vm::opcodes::op;
use crate::vm::runner::VirtualMachine;
use crate::vm::disassembler::disassemble_bytecode;

fn main() {
    let code = bytecode!(
        IPUSH 5,
        IPUSH 10,
        SUB,
        HALT
    );

    let mut vm = VirtualMachine::new(code);
    vm.execute();

    println!("{:?}", vm.stack);
    
    // let dis = disassemble_bytecode(code);
    // println!("{}",dis);

    
}
