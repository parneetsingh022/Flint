#[cfg(test)]
mod test_opcode_basics {
    use super::*;


    #[test]
    fn test_nop() {
        let code = vec![op::NOP, op::NOP, op::HALT];
        let mut vm = VirtualMachine::new(code);
        
        vm.execute();

        assert_eq!(vm.ip, 3);
        assert!(vm.stack.is_empty());
    }
}