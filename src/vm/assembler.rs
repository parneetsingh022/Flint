use crate::vm::opcodes::op;
use std::collections::HashMap;

pub struct Assembler {
    labels: HashMap<String, u32>,
}

impl Assembler {
    pub fn new() -> Self {
        Self { labels: HashMap::new() }
    }

    pub fn assemble(&mut self, input: &str) -> Vec<u8> {
        let lines: Vec<Vec<&str>> = input
            .lines()
            .map(|l| l.split_whitespace().collect())
            .filter(|l: &Vec<&str>| !l.is_empty() && !l[0].starts_with(';')) // Ignore empty lines & comments
            .collect();

        // --- PASS 1: Locate Labels ---
        let mut current_address = 0;
        for line in &lines {
            let first = line[0];
            if first.ends_with(':') {
                let label_name = first.trim_end_matches(':').to_string();
                self.labels.insert(label_name, current_address);
                // If a line is JUST a label, don't increment address
                if line.len() == 1 { continue; }
            }
            
            // Calculate size of the instruction on this line
            // We assume the opcode is either the first word or the word after the label
            let op_idx = if first.ends_with(':') { 1 } else { 0 };
            if op_idx < line.len() {
                current_address += self.get_instruction_size(line[op_idx]);
            }
        }

        // --- PASS 2: Generate Bytes ---
        let mut bytecode = Vec::new();
        for line in &lines {
            let op_idx = if line[0].ends_with(':') { 1 } else { 0 };
            if op_idx >= line.len() { continue; }

            let mnemonic = line[op_idx].to_uppercase();
            match mnemonic.as_str() {
                // No-argument instructions
                "HALT" => bytecode.push(op::HALT),
                "NEG"  => bytecode.push(op::NEG),
                "ADD"  => bytecode.push(op::ADD),
                "SUB"  => bytecode.push(op::SUB),
                "MUL"  => bytecode.push(op::MUL),
                "DIV"  => bytecode.push(op::DIV),
                "MOD"  => bytecode.push(op::MOD),
                "CMP"  => bytecode.push(op::CMP),
                "DUP"  => bytecode.push(op::DUP),
                "POP"  => bytecode.push(op::POP),
                "SWP"  => bytecode.push(op::SWP),
                "PRINT"  => bytecode.push(op::PRINT),

                // 1-byte argument (u8)
                "BIPUSH" => {
                    bytecode.push(op::BIPUSH);
                    bytecode.push(line[op_idx + 1].parse::<u8>().unwrap());
                }

                // 4-byte argument (i32/u32)
                "IPUSH"  => {
                    bytecode.push(op::IPUSH);
                    let val = line[op_idx + 1].parse::<i32>().unwrap();
                    bytecode.extend(&(val as u32).to_be_bytes());
                }

                "LOAD" | "STORE" => {
                    let opcode = if mnemonic == "LOAD" { op::LOAD } else { op::STORE };
                    bytecode.push(opcode);
                    let addr = line[op_idx + 1].parse::<u32>().unwrap();
                    bytecode.extend(&addr.to_be_bytes());
                }

                // Control Flow (Labels)
                "JMP" | "JE" | "JNE" | "JL" | "JLE" | "JG" | "JGE" => {
                    let opcode = match mnemonic.as_str() {
                        "JMP" => op::JMP, "JE" => op::JE, "JNE" => op::JNE,
                        "JL" => op::JL, "JLE" => op::JLE, "JG" => op::JG,
                        "JGE" => op::JGE, _ => unreachable!(),
                    };
                    bytecode.push(opcode);
                    let target = line[op_idx + 1];
                    let addr = *self.labels.get(target).expect("Unknown label");
                    bytecode.extend(&addr.to_be_bytes());
                }
                _ => panic!("Unknown mnemonic: {}", mnemonic),
            }
        }
        bytecode
    }

    fn get_instruction_size(&self, mnemonic: &str) -> u32 {
        match mnemonic.to_uppercase().as_str() {
            // --- 1 Byte (Opcode only) ---
            "NOP" | "HALT" | "POP" | "SWP" | "DUP" | 
            "NEG" | "ADD" | "SUB" | "MUL" | "DIV" | 
            "MOD" | "CMP" | "PRINT"=> 1,

            // --- 2 Bytes (Opcode + u8) ---
            "BIPUSH" => 2,

            // --- 5 Bytes (Opcode + 32-bit value/address) ---
            "IPUSH" | "JL" | "JLE" | "JG" | "JGE" | 
            "JE" | "JNE" | "JMP" | "LOAD" | "STORE" => 5,

            // --- 9 Bytes (Opcode + 64-bit float) ---
            "FPUSH" => 9,

            _ => 0,
        }
    }
}