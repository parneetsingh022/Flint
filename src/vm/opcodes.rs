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

    SWP,
    DUP,

    NEG,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD

    
}

#[macro_export]
macro_rules! bytecode {
    // Handle FPUSH (takes one f64 argument, 8 bytes)
    (FPUSH $val:expr $(, $($rest:tt)*)?) => {{
        let mut v = Vec::new();
        v.push(op::FPUSH);
        v.extend(&(($val) as f64).to_be_bytes());
        $( v.extend(bytecode!($($rest)*)); )?
        v
    }};

    // Handle IPUSH (takes one i32 argument, 4 bytes)
    (IPUSH $val:expr $(, $($rest:tt)*)?) => {{
        let mut v = Vec::new();
        v.push(op::IPUSH);
        v.extend(&(($val) as i32).to_be_bytes());
        $( v.extend(bytecode!($($rest)*)); )?
        v
    }};

    // Handle BIPUSH (takes one u8 argument, 1 byte)
    (BIPUSH $val:expr $(, $($rest:tt)*)?) => {{
        let mut v = Vec::new();
        v.push(op::BIPUSH);
        v.push(($val) as u8);
        $( v.extend(bytecode!($($rest)*)); )?
        v
    }};

    // Handle instructions with no arguments
    ($op:ident $(, $($rest:tt)*)?) => {{
        let mut v = Vec::new();
        v.push(op::$op);
        $( v.extend(bytecode!($($rest)*)); )?
        v
    }};

    () => { Vec::new() };
}