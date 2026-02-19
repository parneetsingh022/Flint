mod vm;

use crate::vm::opcodes::op;
use crate::vm::runner::VirtualMachine;
use crate::vm::disassembler::disassemble_bytecode;
use std::env; 

fn main() {
    let code = bytecode!(
        IPUSH 5,
        JMP 0x0f,
        IPUSH -1,
        HALT,
        IPUSH 309
    );

    let args: Vec<String> = env::args().collect();
    
    let disassemble_mode = args.contains(&"--dis".to_string()) || args.contains(&"-d".to_string());

    if disassemble_mode {
        let dis = disassemble_bytecode(code);
        println!("--- DISASSEMBLY ---\n{}", dis);
    } else {
        // Run the VM normally
        let mut vm = VirtualMachine::new(code);
        vm.execute();

        println!("--- VM STACK ---\n{:?}", vm.stack);
    }
}
