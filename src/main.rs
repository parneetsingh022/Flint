mod vm;

use crate::vm::opcodes::op;
use crate::vm::runner::VirtualMachine;

fn main() {
    let code = bytecode!(
        BIPUSH 20,
        NEG,
        BIPUSH 10,
        ADD,
        HALT
    );
    let mut vm = VirtualMachine::new(code);

    vm.execute();
    println!("{:?}", vm.stack);
    
}
