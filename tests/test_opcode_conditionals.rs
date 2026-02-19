#[cfg(test)]
mod test_opcode_conditionals {
    use flint::vm::runner::*;
    use flint::vm::opcodes::*;
    use flint::bytecode;

    #[test]
    fn test_je_jumps_when_equal() {
        // Setup: 10 == 10, then JE to address 17 (HALT)
        // If it fails, it hits IPUSH 999
        let code = bytecode!(
            BIPUSH 10,
            BIPUSH 10,
            CMP,      // Result: 0 (Equal)
            JE 17,    // JE + 4 byte address = 5 bytes. 
            IPUSH 999, // This is at address 12
            HALT       // This is at address 17
        );
        let mut vm = VirtualMachine::new(code);
        vm.execute();

        // If jump worked, stack should not have 999
        assert!(vm.stack.is_empty());
        assert_eq!(vm.ip, 17); // IP at 17 + fetch() moves it to 18
    }

    #[test]
    fn test_jne_jumps_when_not_equal() {
        // Setup: 10 != 20, then JNE to HALT
        let code = bytecode!(
            BIPUSH 10,
            BIPUSH 20,
            CMP,       // Result: -1 (Not Equal)
            JNE 17,
            IPUSH 999,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        vm.execute();

        assert!(vm.stack.is_empty());
    }

    #[test]
    fn test_jl_jumps_when_less() {
        // Setup: 5 < 10 -> Result -1
        let code = bytecode!(
            BIPUSH 5,
            BIPUSH 10,
            CMP,
            JL 17,
            IPUSH 999,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        vm.execute();

        assert!(vm.stack.is_empty());
    }

    #[test]
    fn test_jg_jumps_when_greater() {
        // Setup: 20 > 10 -> Result 1
        let code = bytecode!(
            BIPUSH 20,
            BIPUSH 10,
            CMP,
            JG 17,
            IPUSH 999,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        vm.execute();

        assert!(vm.stack.is_empty());
    }

    #[test]
    fn test_jle_jumps_when_equal() {
        // JLE should trigger on 0 (equal)
        let code = bytecode!(
            BIPUSH 15,
            BIPUSH 15,
            CMP,
            JLE 17,
            IPUSH 999,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        vm.execute();

        assert!(vm.stack.is_empty());
    }

    #[test]
    fn test_jge_jumps_when_greater() {
        // JGE should trigger on 1 (greater)
        let code = bytecode!(
            BIPUSH 50,
            BIPUSH 10,
            CMP,
            JGE 17,
            IPUSH 999,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        vm.execute();

        assert!(vm.stack.is_empty());
    }

    #[test]
    fn test_conditional_fallthrough() {
        // Test that if condition is NOT met, code continues normally
        // 10 > 50 is false, so JL should NOT jump
        let code = bytecode!(
            BIPUSH 10,
            BIPUSH 50,
            CMP,       // Result: -1 (Less)
            JG 17,     // Should NOT jump to HALT
            BIPUSH 42, // Should execute this
            HALT,      // Address 17
            HALT       // Extra halt for safety
        );
        let mut vm = VirtualMachine::new(code);
        vm.execute();

        // Stack should contain 42 because JG fallthrough worked
        assert_eq!(vm.stack, vec![Value::Int(42)]);
    }
}