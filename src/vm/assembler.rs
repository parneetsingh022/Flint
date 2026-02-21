use crate::vm::opcodes::op;
use std::collections::HashMap;

pub struct Assembler {
    labels: HashMap<String, u32>,
}

impl Assembler {
    pub fn new() -> Self {
        Self { labels: HashMap::new() }
    }

    pub fn assemble(&mut self, input: &str) -> Result<Vec<u8>, String> {
        let lines: Vec<Vec<&str>> = input
            .lines()
            .map(|l| l.split_whitespace().collect())
            .filter(|l: &Vec<&str>| !l.is_empty() && !l[0].starts_with(';'))
            .collect();

        // --- PASS 1: Locate Labels ---
        let mut current_address = 0;
        for line in &lines {
            let first = line[0];
            let op_idx = if first.ends_with(':') {
                let label_name = first.trim_end_matches(':').to_string();
                self.labels.insert(label_name, current_address);
                1 
            } else { 
                0 
            };
            
            if op_idx < line.len() {
                let size = self.get_instruction_size(line[op_idx]);
                if size == 0 {
                    return Err(format!("Unknown instruction: {}", line[op_idx]));
                }
                current_address += size;
            }
        }

        // --- PASS 2: Generate Bytes ---
        let mut bytecode = Vec::new();
        for line in &lines {
            let op_idx = if line[0].ends_with(':') { 1 } else { 0 };
            if op_idx >= line.len() { continue; }

            let mnemonic = line[op_idx];
            let opcode = op::from_mnemonic(mnemonic)
                .ok_or_else(|| format!("Unknown mnemonic: {}", mnemonic))?;
            
            let info = op::get_info(opcode).unwrap(); // Safe because from_mnemonic succeeded
            bytecode.push(opcode);

            if info.size > 1 {
                if line.len() <= op_idx + 1 {
                    return Err(format!("Missing argument for {}", mnemonic));
                }
                let arg = line[op_idx + 1];
                self.encode_operand(&mut bytecode, arg, info.size)?;
            }
        }
        Ok(bytecode)
    }

    fn encode_operand(&self, bytecode: &mut Vec<u8>, arg: &str, size: u32) -> Result<(), String> {
        match size {
            2 => { // 1-byte operand (BIPUSH)
                let val = arg.parse::<u8>().map_err(|_| format!("Invalid u8: {}", arg))?;
                bytecode.push(val);
            }
            5 => { // 4-byte operand (IPUSH, Jumps, Load/Store)
                let val = if let Some(&addr) = self.labels.get(arg) {
                    addr
                } else {
                    arg.parse::<i32>()
                        .map(|v| v as u32)
                        .map_err(|_| format!("Invalid i32 or label: {}", arg))?
                };
                bytecode.extend(&val.to_be_bytes());
            }
            9 => { // 8-byte operand (FPUSH)
                let val = arg.parse::<f64>().map_err(|_| format!("Invalid f64: {}", arg))?;
                bytecode.extend(&val.to_be_bytes());
            }
            _ => {}
        }
        Ok(())
    }

    pub fn get_instruction_size(&self, mnemonic: &str) -> u32 {
        op::from_mnemonic(mnemonic)
            .and_then(|code| op::get_info(code))
            .map(|info| info.size)
            .unwrap_or(0)
    }
}


#[cfg(test)]
mod test_assembler {
    use super::*;

    #[test]
    fn test_assemble_basic_instructions() {
        let mut assembler = Assembler::new();
        let input = "
            BIPUSH 10
            BIPUSH 20
            ADD
            HALT
        ";
        
        // Add .expect() here to get the Vec<u8> out of the Result
        let bytecode = assembler.assemble(input).expect("Assembly failed");
        
        let expected = vec![
            op::BIPUSH, 10,
            op::BIPUSH, 20,
            op::ADD,
            op::HALT
        ];
        
        assert_eq!(bytecode, expected);
    }

    #[test]
    fn test_assemble_labels_and_jumps() {
        let mut assembler = Assembler::new();
        let input = "
            BIPUSH 10
            loop:
            BIPUSH 1
            SUB
            DUP
            BIPUSH 0
            CMP
            JG loop
            HALT
        ";
        
        let bytecode = assembler.assemble(input).expect("Assembly failed");
        
        assert_eq!(bytecode[0], op::BIPUSH);
        assert_eq!(bytecode[1], 10);
        
        let jg_pos = 9;
        assert_eq!(bytecode[jg_pos], op::JG);
        
        let addr = u32::from_be_bytes([
            bytecode[jg_pos + 1],
            bytecode[jg_pos + 2],
            bytecode[jg_pos + 3],
            bytecode[jg_pos + 4],
        ]);
        assert_eq!(addr, 2);
    }

    #[test]
    fn test_assemble_comments_and_whitespace() {
        let mut assembler = Assembler::new();
        let input = "
            ; This is a comment
            BIPUSH 42    ; Push value
            
            HALT         ; Stop
        ";
        
        let bytecode = assembler.assemble(input).expect("Assembly failed");
        let expected = vec![op::BIPUSH, 42, op::HALT];
        
        assert_eq!(bytecode, expected);
    }

    #[test]
    fn test_assemble_ipush_large_values() {
        let mut assembler = Assembler::new();
        let input = "IPUSH 500000";
        
        let bytecode = assembler.assemble(input).expect("Assembly failed");
        
        assert_eq!(bytecode[0], op::IPUSH);
        let val = i32::from_be_bytes([bytecode[1], bytecode[2], bytecode[3], bytecode[4]]);
        assert_eq!(val, 500000);
    }

    #[test]
    fn test_assemble_case_insensitivity() {
        let mut assembler = Assembler::new();
        let input = "bipush 10\nAdd\nhalt";
        
        let bytecode = assembler.assemble(input).expect("Assembly failed");
        let expected = vec![op::BIPUSH, 10, op::ADD, op::HALT];
        
        assert_eq!(bytecode, expected);
    }
    
    #[test]
    fn test_assemble_invalid_mnemonic() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble("NOT_AN_OPCODE 123");
        assert!(result.is_err());
    }


    #[test]
    fn test_assemble_fpush() {
        let mut assembler = Assembler::new();
        let input = "FPUSH 123.456";
        let bytecode = assembler.assemble(input).expect("Assembly failed");
        
        assert_eq!(bytecode[0], op::FPUSH);
        let val = f64::from_be_bytes(bytecode[1..9].try_into().unwrap());
        assert_eq!(val, 123.456);
    }

    #[test]
    fn test_assemble_store_load() {
        let mut assembler = Assembler::new();
        let input = "
            BIPUSH 10
            STORE 50
            LOAD 50
            HALT
        ";
        let bytecode = assembler.assemble(input).expect("Assembly failed");
        
        // Check STORE at index 2 (after BIPUSH 10)
        assert_eq!(bytecode[2], op::STORE);
        let addr = u32::from_be_bytes([bytecode[3], bytecode[4], bytecode[5], bytecode[6]]);
        assert_eq!(addr, 50);
    }

    #[test]
    fn test_assemble_label_only_line() {
        let mut assembler = Assembler::new();
        let input = "
            start:
            ; This label should point to address 0
            HALT
        ";
        let bytecode = assembler.assemble(input).expect("Assembly failed");
        
        // If the label-only line worked, HALT should be at index 0
        assert_eq!(bytecode[0], op::HALT);
    }

    #[test]
    fn test_assemble_forward_jump() {
        let mut assembler = Assembler::new();
        let input = "
            JMP target
            BIPUSH 99
            target:
            HALT
        ";
        let bytecode = assembler.assemble(input).expect("Assembly failed");
        
        // JMP is at 0, target is at 7 (5 bytes for JMP + 2 bytes for BIPUSH)
        let addr = u32::from_be_bytes([bytecode[1], bytecode[2], bytecode[3], bytecode[4]]);
        assert_eq!(addr, 7);
    }
}