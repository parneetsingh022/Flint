mod vm;

use crate::vm::opcodes::op;
use crate::vm::runner::VirtualMachine;

fn main() {
    let code = bytecode!(
        NOP, 
        BIPUSH 10,  // 0x0a
        BIPUSH 52,  // 0x34
        SWP,
        HALT
    );
    let mut vm = VirtualMachine::new(code);

    vm.execute();
    println!("{:?}", vm.stack);
    
}
