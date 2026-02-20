macro_rules! define_opcodes {
    // This part handles the recursion to assign numbers
    (@count $asm:ident, $output:ident, $ip:ident, $bytecode:ident, $val:expr, ) => {};
    
    ($($name:ident),*) => {
        pub mod op {
            // Internal trick to generate incrementing constants
            $(pub const $name: u8 = define_opcodes!(@inner $name);)*
            
            // Generate the lookup function
            pub fn get_name(code: u8) -> &'static str {
                let mut i = 0;
                $(
                    if code == i { return stringify!($name); }
                    i += 1;
                )*
                "UNKNOWN"
            }
        }
    };
}


macro_rules! opcodes {
    ($($name:ident),*) => {
        pub mod op {
            $(pub const $name: u8 = op_enum::$name as u8;)*

            #[repr(u8)]
            #[derive(Debug)]
            enum op_enum {
                $($name,)*
            }

            pub fn get_name(code: u8) -> &'static str {
                $(if code == op_enum::$name as u8 { return stringify!($name); })*
                "UNKNOWN"
            }
        }
    }
}

opcodes! {
    NOP,
    HALT,

    IPUSH,
    BIPUSH,
    FPUSH,
    POP,

    // 32 bit address
    STORE,
    LOAD,

    SWP,
    DUP,

    NEG,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,

    CMP,

    // 32 Bit Absolute Jumps
    JL,
    JLE,
    JG,
    JGE,
    JE,
    JNE,
    JMP,

    PRINT

}

#[macro_export]
macro_rules! bytecode {
    // Specific overrides for non-4-byte instructions
    (FPUSH $val:expr $(, $($rest:tt)*)?) => {{
        let mut v = Vec::new();
        v.push(op::FPUSH);
        v.extend(&(($val) as f64).to_be_bytes());
        $( v.extend(bytecode!($($rest)*)); )?
        v
    }};

    (BIPUSH $val:expr $(, $($rest:tt)*)?) => {{
        let mut v = Vec::new();
        v.push(op::BIPUSH);
        v.push(($val) as u8);
        $( v.extend(bytecode!($($rest)*)); )?
        v
    }};

    // Dispatchers for 4-byte instructions
    (IPUSH $v:expr $(, $($r:tt)*)?) => { bytecode!(@four IPUSH, $v, $(, $($r)*)?) };
    (JL $v:expr $(, $($r:tt)*)?)    => { bytecode!(@four JL, $v, $(, $($r)*)?) };
    (JLE $v:expr $(, $($r:tt)*)?)   => { bytecode!(@four JLE, $v, $(, $($r)*)?) };
    (JG $v:expr $(, $($r:tt)*)?)    => { bytecode!(@four JG, $v, $(, $($r)*)?) };
    (JGE $v:expr $(, $($r:tt)*)?)   => { bytecode!(@four JGE, $v, $(, $($r)*)?) };
    (JE $v:expr $(, $($r:tt)*)?)    => { bytecode!(@four JE, $v, $(, $($r)*)?) };
    (JNE $v:expr $(, $($r:tt)*)?)   => { bytecode!(@four JNE, $v, $(, $($r)*)?) };
    (JMP $v:expr $(, $($r:tt)*)?)   => { bytecode!(@four JMP, $v, $(, $($r)*)?) };

    (@four $op:ident, $val:expr, $(, $($rest:tt)*)?) => {{
        let mut v = Vec::new();
        v.push(op::$op);
        v.extend(&(($val as i32) as u32).to_be_bytes());
        $( v.extend(bytecode!($($rest)*)); )?
        v
    }};

    // Instructions with no arguments (HALT, POP, etc.)
    ($op:ident $(, $($rest:tt)*)?) => {{
        let mut v = Vec::new();
        v.push(op::$op);
        $( v.extend(bytecode!($($rest)*)); )?
        v
    }};

    () => { Vec::new() };
}