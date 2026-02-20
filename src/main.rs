mod vm;

use crate::vm::opcodes::op;
use crate::vm::runner::VirtualMachine;
use crate::vm::disassembler::disassemble_bytecode;
use crate::vm::assembler::Assembler;
use std::env; 

fn main() {

    let mut assembler = Assembler::new();

    let source = "
        _start:
            BIPUSH 2    ; Counter 
        even_loop:
            DUP
            IPUSH 2
            MOD
            IPUSH 0
            CMP
            JNE odd
        even:
            DUP
        odd:
            IPUSH 1
            ADD
            ; check if we reached item 10
            DUP
            IPUSH 10
            CMP
            JGE end
            JMP even_loop
        
        end:
            HALT
    ";


    let code = assembler.assemble(source);
    let mut vm = VirtualMachine::new(code.clone());
    vm.execute();


    let args: Vec<String> = env::args().collect();
    
    let disassemble_mode = args.contains(&"--dis".to_string()) || args.contains(&"-d".to_string());
    let bytecode_mode = args.contains(&"--raw".to_string()) || args.contains(&"-d".to_string());

    if disassemble_mode {
        let dis = disassemble_bytecode(code);
        println!("--- DISASSEMBLY ---\n{}", dis);
    } else if bytecode_mode{
        println!("--- Raw Bytecode ---");

        for chunk in code.chunks(10) {
            for byte in chunk {
                print!("{:02X} ", byte);
            }
            println!();
        }
    }else {
        // Run the VM normally
        let mut vm = VirtualMachine::new(code);
        vm.execute();

        println!("--- VM STACK ---\n{:?}", vm.stack);
    }
}
