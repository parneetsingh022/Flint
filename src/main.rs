mod vm;

use crate::vm::opcodes::op_codes;

fn main() {
    println!("IPUSH {}", op_codes::IPUSH);
}
