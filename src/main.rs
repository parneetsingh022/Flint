mod vm;

use crate::vm::runner::VirtualMachine;
use crate::vm::disassembler::disassemble_bytecode;
use crate::vm::assembler::Assembler;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 1. Check for the filename argument
    if args.len() < 2 {
        eprintln!("Usage: flint <filename> [options]");
        eprintln!("Options: -d, --dis    Disassemble the code");
        eprintln!("         --raw        Print raw bytecode");
        process::exit(1);
    }

    let filename = &args[1];

    // Read the source file
    let source = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);
        process::exit(1);
    });

    // Assemble the string from the file
    let mut assembler = Assembler::new();
    let code = assembler.assemble(&source);

    // Handle flags
    let disassemble_mode = args.contains(&"--dis".to_string()) || args.contains(&"-d".to_string());
    let bytecode_mode = args.contains(&"--raw".to_string());

    if disassemble_mode {
        let dis = disassemble_bytecode(code);
        println!("--- DISASSEMBLY (File: {}) ---\n{}", filename, dis);
    } else if bytecode_mode {
        println!("--- Raw Bytecode ---");
        for chunk in code.chunks(10) {
            for byte in chunk {
                print!("{:02X} ", byte);
            }
            println!();
        }
    } else {
        // Run the VM normally
        let mut vm = VirtualMachine::new(code);
        vm.execute();

        println!("\n--- VM STATE ---");
        println!("Stack:  {:?}", vm.stack);
        println!("Memory: {:?}", vm.memory);
    }
}