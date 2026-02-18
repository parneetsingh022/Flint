#[cfg(test)]
mod test_opcode_basics {
    use flint::vm::runner::*;
    use flint::vm::opcodes::*;
    use flint::bytecode;

    #[test]
    fn test_nop() {
        let code = bytecode!(NOP, NOP, HALT);
        let mut vm = VirtualMachine::new(code);
        
        vm.execute();

        assert_eq!(vm.ip, 3);
        assert!(vm.stack.is_empty());
    }

    #[test]
    fn test_halt_stops_execution() {
        
        let code = bytecode!(HALT, IPUSH  100);
        let mut vm = VirtualMachine::new(code);
        
        vm.execute();
        assert_eq!(vm.ip, 1);
        assert!(vm.stack.is_empty());
    }

    #[test]
    fn test_ipush_pushes_integer(){
        let code = bytecode!(
            IPUSH 120,
            IPUSH 539,
            IPUSH 4533084,
            IPUSH 2147483647,  // Max i32
            IPUSH -2147483648, // Min i32
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        vm.execute();

        assert_eq!(vm.ip, 26);
        assert_eq!(vec![
            Value::Int(120), 
            Value::Int(539),
            Value::Int(4533084),
            Value::Int(2147483647),
            Value::Int(-2147483648),
        ], vm.stack);
    }

    #[test]
    fn test_ipop_removes_top_of_stack() {
        let code = bytecode!(
            IPUSH 100,
            IPUSH 200,
            IPOP,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        
        vm.execute();

        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(100));
    }

    #[test]
    fn test_bipush_pushes_single_byte() {
        let code = bytecode!(
            BIPUSH 10, 
            BIPUSH 127, 
            BIPUSH 255, 
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        
        vm.execute();

        assert_eq!(vm.stack.len(), 3);
        assert_eq!(vm.stack[0], Value::Int(10));
        assert_eq!(vm.stack[1], Value::Int(127));
        assert_eq!(vm.stack[2], Value::Int(255));
    }

    #[test]
    fn test_swp_swaps_top_two_values() {
        let code = bytecode!(
            BIPUSH 10,
            BIPUSH 20,
            SWP,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        
        vm.execute();

        assert_eq!(vm.stack, vec![Value::Int(20), Value::Int(10)]);
    }

    #[test]
    fn test_dup_duplicates_top_value() {
        let code = bytecode!(
            BIPUSH 42,
            DUP,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        
        vm.execute();

        assert_eq!(vm.stack, vec![Value::Int(42), Value::Int(42)]);
    }

    #[test]
    fn test_add_integers() {
        // Pushes 10 and 20, then adds them
        let code = bytecode!(
            BIPUSH 10,
            BIPUSH 20,
            ADD,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        
        vm.execute();

        // The stack should contain exactly one value: 30
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(30));
    }
}