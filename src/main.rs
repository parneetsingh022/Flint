mod vm;

use crate::vm::opcodes::op;
use crate::vm::runner::VirtualMachine;
use crate::vm::disassembler::disassemble_bytecode;
use crate::vm::assembler::Assembler;
use std::env; 

fn main() {

    let mut assembler = Assembler::new();

    let source = "
        BIPUSH 1    ; Counter 
    ";

    let code = assembler.assemble(source);
    let mut vm = VirtualMachine::new(code.clone());
    vm.execute();

    // Results in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    println!("{:?}", vm.stack);

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
