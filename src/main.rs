mod vm;

use crate::vm::opcodes::op;
use crate::vm::runner::VirtualMachine;

fn main() {
    let code = bytecode!(
        NOP, 
        BIPUSH 10,
        BIPUSH 52,
        ADD,
        SWP,
        HALT
    );
    let mut vm = VirtualMachine::new(code);

    vm.execute();
    println!("{:?}", vm.stack);
    
}
