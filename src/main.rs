mod vm;

use crate::vm::opcodes::op;
use crate::vm::runner::VirtualMachine;
use crate::vm::disassembler::disassemble_bytecode;
use crate::vm::assembler::Assembler;
use std::env; 

fn main() {

    let mut assembler = Assembler::new();

    // for (int i = 0; i < 10; i++){
    //     System.out.println(i);
    // }

    let source = "
        _start:
            BIPUSH 2
            STORE  0     ; (i)

        for_i_0_10:
            LOAD   0
            BIPUSH 40
            CMP
            jl loop_body
            jmp end_for_i_0_10
            loop_body:
                BIPUSH 1  ; True for prime
                STORE  2
                ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

                BIPUSH 2
                STORE  1     ; (j)

                for2_i_0_10:
                    LOAD   1
                    LOAD   0
                    BIPUSH 1
                    SUB
                    CMP
                    jl loop2_body
                    jmp end_for2_i_0_10
                    loop2_body:
                        ;###############
                        LOAD 0
                        LOAD 1
                        MOD
                        BIPUSH 0
                        CMP
                        jne prime
                            BIPUSH 0
                            STORE  2
                            JMP end_for2_i_0_10
                        prime:
                        ;###############

                    incr_for2_i_0_10:
                        LOAD 1
                        BIPUSH 1
                        ADD
                        STORE 1
                        jmp for2_i_0_10

                end_for2_i_0_10:
                    LOAD 2
                    BIPUSH 1
                    CMP
                    jne incr_for_i_0_10

                    LOAD 0
                    PRINT

                ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

            incr_for_i_0_10:
                LOAD 0
                BIPUSH 1
                ADD
                STORE 0
                jmp for_i_0_10

        end_for_i_0_10:
            



        _end:
            HALT
    ";


    let code = assembler.assemble(source);


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

        println!("--- VM STACK ---\n{:?}\n\n--- VM MEMORY ---\n{:?}\n", vm.stack, vm.memory);
    }
}
