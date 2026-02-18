use crate::vm::opcodes::op;

pub fn disassemble_bytecode(bytecode: Vec<u8>) -> String {
    let mut ip = 0;
    let mut asm = String::new();

    while ip < bytecode.len() {
        let cur = bytecode[ip]; // This IS your single byte opcode
        let name = op::get_name(cur);
        
        // Standard Prefix for every instruction
        let prefix = format!("{:04X}: {:02X}", ip, cur);

        match cur {
            // MULTI-BYTE: Handle opcodes that NEED extra bytes
            op::IPUSH => {
                let val = i32::from_be_bytes(bytecode[ip+1..ip+5].try_into().unwrap());
                asm.push_str(&format!("{} {:<10} {}\n", prefix, name, val));
                ip += 5; 
            }
            op::BIPUSH => {
                // SINGLE-BYTE OPERAND: Getting the byte at ip + 1
                let val = bytecode[ip + 1]; 
                asm.push_str(&format!("{} {:<10} {}\n", prefix, name, val as i8));
                ip += 2;
            }

           
            _ => {
                asm.push_str(&format!("{} {}\n", prefix, name));
                ip += 1;
            }
        }
    }
    asm
}