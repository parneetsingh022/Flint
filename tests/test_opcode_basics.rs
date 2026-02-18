#[cfg(test)]
mod test_opcode_basics {
    use flint::vm::runner::VirtualMachine;
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
}