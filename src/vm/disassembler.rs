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
            // Opcode with 32-Bit Operand
            op::IPUSH   => {
                let val = i32::from_be_bytes(bytecode[ip+1..ip+5].try_into().unwrap());
                asm.push_str(&format!("{} {:<10} {}\n", prefix, name, val));
                ip += 5; 
            }
            
            // Opcode with 32-Bit Operand (Unsigned)
            op::JL | op::JG | op::JLE | op::JGE | op::JE | op::JNE | op::JMP | op::LOAD | op::STORE  => {
                let val = u32::from_be_bytes(bytecode[ip+1..ip+5].try_into().unwrap());
                asm.push_str(&format!("{} {:<10} {} (0x{:02X})\n", prefix, name, val, val));
                ip += 5; 
            }
    
            op::FPUSH => {
                // FPUSH takes an 8-byte (64-bit) f64 immediate operand
                let start = ip + 1;
                let end = ip + 9; // ip + 1 opcode byte + 8 data bytes
                let val = f64::from_be_bytes(bytecode[start..end].try_into().unwrap());
                
                asm.push_str(&format!("{} {:<10} {:.4}\n", prefix, name, val));
                ip += 9; // Advance IP past the opcode and the 8-byte operand
            }

            // Opcode with 8 bit
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