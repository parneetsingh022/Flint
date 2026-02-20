mod vm;

use crate::vm::opcodes::op;
use crate::vm::runner::VirtualMachine;
use crate::vm::disassembler::disassemble_bytecode;
use std::env; 

fn main() {
    let code = bytecode!(
        BIPUSH 0,

        DUP,
        
        BIPUSH 255,
        CMP,
        JGE 0x14,
        
        DUP,
        BIPUSH 1,
        ADD,
        
        JMP 2,
        
        HALT
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
