use crate::vm::opcodes::op;

pub fn disassemble_bytecode(bytecode: Vec<u8>) -> String {
    let mut ip = 0;
    let mut asm = String::new();

    while ip < bytecode.len() {
        let cur = bytecode[ip];
        
        // Retrieve metadata from our centralized opcode definition
        let info = match op::get_info(cur) {
            Some(i) => i,
            None => {
                asm.push_str(&format!("{:04X}: {:02X} UNKNOWN\n", ip, cur));
                ip += 1;
                continue;
            }
        };

        let prefix = format!("{:04X}: {:02X}", ip, cur);
        let name = info.name;

        match info.size {
            1 => {
                // No arguments (e.g., ADD, HALT, POP)
                asm.push_str(&format!("{} {}\n", prefix, name));
                ip += 1;
            }
            2 => {
                // 1-byte argument (e.g., BIPUSH)
                let val = bytecode[ip + 1];
                asm.push_str(&format!("{} {:<10} {}\n", prefix, name, val as i8));
                ip += 2;
            }
            5 => {
                // 4-byte argument (e.g., IPUSH, JMP, LOAD, STORE)
                let bytes = &bytecode[ip + 1..ip + 5];
                let val = u32::from_be_bytes(bytes.try_into().unwrap());
                
                if cur == op::IPUSH {
                    asm.push_str(&format!("{} {:<10} {}\n", prefix, name, val as i32));
                } else {
                    // Use {:<8} to give the decimal value a consistent 8-character width
                    // This ensures the (0xXX) part starts at the same column every time
                    asm.push_str(&format!("{} {:<10} {:<8} (0x{:02X})\n", prefix, name, val, val));
                }
                ip += 5;
            }
            9 => {
                // 8-byte argument (e.g., FPUSH)
                let bytes = &bytecode[ip + 1..ip + 9];
                let val = f64::from_be_bytes(bytes.try_into().unwrap());
                asm.push_str(&format!("{} {:<10} {:.4}\n", prefix, name, val));
                ip += 9;
            }
            _ => {
                asm.push_str(&format!("{} {}\n", prefix, name));
                ip += 1;
            }
        }
    }
    asm
}


#[cfg(test)]
mod test_disassembler {
    use super::*;
    use crate::vm::opcodes::op;

    #[test]
    fn test_disassemble_single_byte_instructions() {
        let bytecode = vec![op::ADD, op::HALT];
        let result = disassemble_bytecode(bytecode);

        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 2);

        // Dynamically create the expected strings based on actual opcode values
        let add_expected = format!("0000: {:02X} ADD", op::ADD);
        let halt_expected = format!("0001: {:02X} HALT", op::HALT);

        assert!(lines[0].contains(&add_expected), "Expected line to contain '{}', but got '{}'", add_expected, lines[0]);
        assert!(lines[1].contains(&halt_expected), "Expected line to contain '{}', but got '{}'", halt_expected, lines[1]);
    }

    #[test]
    fn test_disassemble_two_byte_instruction() {
        // Testing 2-byte instruction: BIPUSH 10
        let bytecode = vec![op::BIPUSH, 10];
        let result = disassemble_bytecode(bytecode);

        assert!(result.contains("0000: 03 BIPUSH     10"));
    }

    #[test]
    fn test_disassemble_five_byte_signed_instruction() {
        // Testing IPUSH with a negative value to verify signed formatting
        let val: i32 = -500;
        let mut bytecode = vec![op::IPUSH];
        bytecode.extend(&val.to_be_bytes());
        
        let result = disassemble_bytecode(bytecode);
        assert!(result.contains("0000: 02 IPUSH      -500"));
    }

    #[test]
    fn test_disassemble_five_byte_aligned_instruction() {
        // Testing alignment for LOAD/STORE/Jumps
        // Format: {prefix} {name:<10} {val:<8} (0x{val:02X})
        let mut bytecode = vec![op::LOAD];
        bytecode.extend(&10u32.to_be_bytes());
        
        let result = disassemble_bytecode(bytecode);
        
        // Check for specific spacing: LOAD (4 chars) + 6 spaces = 10 total width
        // Then 10 (2 chars) + 6 spaces = 8 total width
        assert!(result.contains("LOAD       10       (0x0A)"));
    }

    #[test]
    fn test_disassemble_nine_byte_instruction() {
        // Testing 9-byte instruction: FPUSH 42.5
        let val: f64 = 42.5;
        let mut bytecode = vec![op::FPUSH];
        bytecode.extend(&val.to_be_bytes());

        let result = disassemble_bytecode(bytecode);
        assert!(result.contains("0000: 04 FPUSH      42.5000"));
    }

    #[test]
    fn test_disassemble_unknown_opcode() {
        // Testing an opcode that doesn't exist (e.g., 0xFF)
        let bytecode = vec![0xFF];
        let result = disassemble_bytecode(bytecode);

        assert!(result.contains("0000: FF UNKNOWN"));
    }

    #[test]
    fn test_disassemble_complex_sequence() {
        // Combining multiple types to ensure IP increments correctly
        let mut bytecode = vec![op::BIPUSH, 5]; // 2 bytes
        bytecode.push(op::ADD);                 // 1 byte
        bytecode.push(op::STORE);               // 5 bytes
        bytecode.extend(&20u32.to_be_bytes());
        bytecode.push(op::HALT);                // 1 byte

        let result = disassemble_bytecode(bytecode);
        let lines: Vec<&str> = result.lines().collect();

        assert_eq!(lines.len(), 4);
        assert!(lines[0].starts_with("0000:")); // BIPUSH
        assert!(lines[1].starts_with("0002:")); // ADD
        assert!(lines[2].starts_with("0003:")); // STORE
        assert!(lines[3].starts_with("0008:")); // HALT
    }

    #[test]
    fn test_disassemble_alignment_consistency() {
        let mut bytecode = vec![op::LOAD];
        bytecode.extend(&5u32.to_be_bytes());
        bytecode.push(op::STORE);
        bytecode.extend(&500u32.to_be_bytes());

        let result = disassemble_bytecode(bytecode);
        let lines: Vec<&str> = result.lines().collect();

        // Find the index of the '(' character in both lines
        let pos1 = lines[0].find('(').unwrap();
        let pos2 = lines[1].find('(').unwrap();

        assert_eq!(pos1, pos2, "Hex offsets (0xXX) are not aligned vertically");
    }

    #[test]
    fn test_disassemble_large_address_hex() {
        let mut bytecode = vec![op::JMP];
        bytecode.extend(&1000u32.to_be_bytes()); // 0x03E8
        
        let result = disassemble_bytecode(bytecode);
        // Should show (0x3E8) or (0x03E8) depending on your width
        assert!(result.contains("(0x3E8)") || result.contains("(0x03E8)"));
    }

    #[test]
    fn test_disassemble_ipush_vs_jmp_format() {
        let mut bytecode = vec![op::IPUSH];
        bytecode.extend(&(-10i32).to_be_bytes());
        bytecode.push(op::JMP);
        bytecode.extend(&10u32.to_be_bytes());

        let result = disassemble_bytecode(bytecode);
        let lines: Vec<&str> = result.lines().collect();

        assert!(lines[0].contains("-10"));
        assert!(!lines[0].contains("(0x")); // IPUSH should not have hex suffix
        assert!(lines[1].contains("(0x0A)")); // JMP should have hex suffix
    }

    #[test]
    fn test_disassemble_fpush_precision() {
        let mut bytecode = vec![op::FPUSH];
        bytecode.extend(&1.2345678f64.to_be_bytes());

        let result = disassemble_bytecode(bytecode);
        // Should round to 1.2346 or 1.2345 based on Rust's default formatting
        assert!(result.contains("1.2346")); 
    }

    #[test]
    fn test_disassemble_empty_bytecode() {
        let result = disassemble_bytecode(vec![]);
        assert_eq!(result, "");
    }
}