#[cfg(test)]
mod test_opcode_basics {
    use flint::vm::runner::*;
    use flint::vm::opcodes::op;


    #[test]
    fn test_nop() {
        let code = vec![op::NOP, op::NOP, op::HALT];
        let mut vm = VirtualMachine::new(code);
        
        vm.execute();

        assert_eq!(vm.ip, 3);
        assert!(vm.stack.is_empty());
    }

    #[test]
    fn test_halt_stops_execution() {
        
        let code = vec![op::HALT, op::IPUSH, 0, 0, 0, 100];
        let mut vm = VirtualMachine::new(code);
        
        vm.execute();
        assert_eq!(vm.ip, 1);
        assert!(vm.stack.is_empty());
    }

    #[test]
    fn test_ipush_pushes_integer(){
        let code = vec![op::IPUSH, 0,0,0,120, 
                        op::IPUSH, 0,0,0x02,0x1b, //539
                        op::IPUSH, 0,0x45,0x2b,0x5c, //4533084
                        op::IPUSH, 0x7f,0xff,0xff,0xff,//2147483647
                        op::IPUSH, 0x80,0x00,0x00,0x00,//-2147483648
                        op::HALT];
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
        let code = vec![
            op::IPUSH, 0, 0, 0, 100, 
            op::IPUSH, 0, 0, 0, 200, 
            op::IPOP, 
            op::HALT
        ];
        let mut vm = VirtualMachine::new(code);
        
        vm.execute();

        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(100));
    }

    #[test]
    fn test_bipush_pushes_single_byte() {
        let code = vec![
            op::BIPUSH, 10, 
            op::BIPUSH, 127, 
            op::BIPUSH, 255, 
            op::HALT
        ];
        let mut vm = VirtualMachine::new(code);
        
        vm.execute();

        assert_eq!(vm.stack.len(), 3);
        assert_eq!(vm.stack[0], Value::Int(10));
        assert_eq!(vm.stack[1], Value::Int(127));
        assert_eq!(vm.stack[2], Value::Int(255));
    }

    #[test]
    fn test_swp_swaps_top_two_values() {
        let code = vec![
            op::BIPUSH, 10,
            op::BIPUSH, 20,
            op::SWP,
            op::HALT
        ];
        let mut vm = VirtualMachine::new(code);
        
        vm.execute();

        assert_eq!(vm.stack, vec![Value::Int(20), Value::Int(10)]);
    }

    #[test]
    fn test_dup_duplicates_top_value() {
        let code = vec![
            op::BIPUSH, 42,
            op::DUP,
            op::HALT
        ];
        let mut vm = VirtualMachine::new(code);
        
        vm.execute();

        assert_eq!(vm.stack, vec![Value::Int(42), Value::Int(42)]);
    }
}