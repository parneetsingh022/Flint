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
            POP,
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
    fn test_add_neg_integers() {
        let code = bytecode!(
            BIPUSH 10,
            NEG,
            BIPUSH 20,
            ADD,
            BIPUSH 40,
            NEG,
            BIPUSH 10,
            ADD,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        
        vm.execute();

        assert_eq!(vm.stack.len(), 2);
        assert_eq!(vm.stack[0], Value::Int(10));
        assert_eq!(vm.stack[1], Value::Int(-30));
    }

    #[test]
    fn test_add_integers() {
        let code = bytecode!(
            BIPUSH 10,
            BIPUSH 20,
            ADD,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        
        vm.execute();

        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(30));
    }

    #[test]
    fn test_sub_integers() {
        let code = bytecode!(
            BIPUSH 10,
            BIPUSH 20,
            SUB,
            BIPUSH 35,
            BIPUSH 10,
            SUB,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        
        vm.execute();

        assert_eq!(vm.stack.len(), 2);
        assert_eq!(vm.stack[0], Value::Int(-10));
        assert_eq!(vm.stack[1], Value::Int(25));
    }


    #[test]
    fn test_mul_integers() {
        let code = bytecode!(
            BIPUSH 6,
            BIPUSH 7,
            MUL,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        vm.execute();

        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(42));
    }

    #[test]
    fn test_div_integers() {
        let code = bytecode!(
            BIPUSH 100,
            BIPUSH 25,
            DIV,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        vm.execute();

        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(4));
    }

    #[test]
    fn test_mod_integers() {
        let code = bytecode!(
            BIPUSH 10,
            BIPUSH 3,
            MOD,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        vm.execute();

        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(1));
    }

    #[test]
    #[should_panic(expected = "Runtime Error: Division by zero")]
    fn test_div_by_zero() {
        let code = bytecode!(
            BIPUSH 10,
            BIPUSH 0,
            DIV,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        vm.execute();
    }

    #[test]
    fn test_complex_math_stack() {
        // (10 * 2) / (10 - 5) = 4
        let code = bytecode!(
            BIPUSH 10,
            BIPUSH 2,
            MUL,    // Stack: [20]
            BIPUSH 10,
            BIPUSH 5,
            SUB,    // Stack: [20, 5]
            DIV,    // Stack: [4]
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        vm.execute();

        assert_eq!(vm.stack[0], Value::Int(4));
    }

    #[test]
    fn test_add_mixed_float_int() {
        // 10.5 + 20 = 30.5
        let code = bytecode!(
            FPUSH 10.5, 
            BIPUSH 20,
            ADD,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        vm.execute();

        assert_eq!(vm.stack[0], Value::Float(30.5));
    }

    #[test]
    fn test_div_mixed_int_float() {
        // 10 / 2.5 = 4.0
        let code = bytecode!(
            BIPUSH 10,
            FPUSH 2.5,
            DIV,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        vm.execute();

        assert_eq!(vm.stack[0], Value::Float(4.0));
    }

    #[test]
    fn test_float_pure_math() {
        // 5.5 * 2.0 = 11.0
        let code = bytecode!(
            FPUSH 5.5,
            FPUSH 2.0,
            MUL,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        vm.execute();

        assert_eq!(vm.stack[0], Value::Float(11.0));
    }

    #[test]
    fn test_sub_float_int() {
        // 10.0 - 5 = 5.0
        let code = bytecode!(
            FPUSH 10.0,
            BIPUSH 5,
            SUB,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        vm.execute();

        assert_eq!(vm.stack[0], Value::Float(5.0));
    }

    #[test]
    #[should_panic(expected = "Runtime Error: Division by zero")]
    fn test_float_div_by_zero() {
        let code = bytecode!(
            FPUSH 10.0,
            FPUSH 0.0,
            DIV,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        vm.execute();
    }


    #[test]
    fn test_cmp_integers() {
        // Test Less Than (10 < 20) -> -1
        let code_lt = bytecode!(BIPUSH 10, BIPUSH 20, CMP, HALT);
        let mut vm_lt = VirtualMachine::new(code_lt);
        vm_lt.execute();
        assert_eq!(vm_lt.stack[0], Value::Int(-1));

        // Test Equal (15 == 15) -> 0
        let code_eq = bytecode!(BIPUSH 15, BIPUSH 15, CMP, HALT);
        let mut vm_eq = VirtualMachine::new(code_eq);
        vm_eq.execute();
        assert_eq!(vm_eq.stack[0], Value::Int(0));

        // Test Greater Than (30 > 10) -> 1
        let code_gt = bytecode!(BIPUSH 30, BIPUSH 10, CMP, HALT);
        let mut vm_gt = VirtualMachine::new(code_gt);
        vm_gt.execute();
        assert_eq!(vm_gt.stack[0], Value::Int(1));
    }

    #[test]
    fn test_cmp_mixed_types() {
        // Float < Int (5.5 < 10) -> -1
        let code_mixed = bytecode!(FPUSH 5.5, BIPUSH 10, CMP, HALT);
        let mut vm = VirtualMachine::new(code_mixed);
        vm.execute();
        assert_eq!(vm.stack[0], Value::Int(-1));
    }

    #[test]
    fn test_cmp_with_neg_opcode() {
        // Test: -5 > -10  => Result: 1
        // Logic: 5, NEG, 10, NEG, CMP
        let code = bytecode!(
            BIPUSH 5, 
            NEG,        // Stack: [-5]
            BIPUSH 10, 
            NEG,        // Stack: [-5, -10]
            CMP,        // Compare -5 and -10
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        vm.execute();

        // -5 is indeed greater than -10
        assert_eq!(vm.stack[0], Value::Int(1));
    }

    #[test]
    fn test_cmp_floats_equality() {
        // Float equality (0.5 == 0.5) -> 0
        let code_f_eq = bytecode!(FPUSH 0.5, FPUSH 0.5, CMP, HALT);
        let mut vm_f = VirtualMachine::new(code_f_eq);
        vm_f.execute();
        assert_eq!(vm_f.stack[0], Value::Int(0));
    }


    #[test]
    fn test_neg_mixed_float_int() {
        // 10.5 + 20 = 30.5
        let code = bytecode!(
            FPUSH 10.5,
            NEG,
            BIPUSH 20,
            NEG,
            HALT
        );
        let mut vm = VirtualMachine::new(code);
        vm.execute();

        assert_eq!(vm.stack[0], Value::Float(-10.5));
        assert_eq!(vm.stack[1], Value::Int(-20));
    }

}