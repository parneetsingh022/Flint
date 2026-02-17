mod vm;

use crate::vm::opcodes::op;
use crate::vm::runner::VirtualMachine;

fn main() {
    let code : Vec<u8> = vec![
        op::NOP, 
        op::BIPUSH, 0x0a,
        op::BIPUSH, 0x34,
        //op::IPOP,
        op::HALT,
    ];
    let mut vm = VirtualMachine::new(code);

    vm.execute();
    println!("{:?}", vm.stack);
    
}
