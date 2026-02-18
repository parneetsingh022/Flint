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
    NOP,    // 0
    HALT,   // 1
    IPUSH,  // 2
    IPOP,   // 3
    BIPUSH, // 4
    SWP,    // 5
    DUP,    // 6
    NEG,    // 7
    ADD,    // 8
    SUB
}

#[macro_export]
macro_rules! bytecode {
    // 1. Handle IPUSH (takes one i32 argument)
    (IPUSH $val:expr $(, $($rest:tt)*)?) => {{
        let mut v = Vec::new();
        v.push(op::IPUSH);
        v.extend(&($val as i32).to_be_bytes());
        $( v.extend(bytecode!($($rest)*)); )?
        v
    }};

    // 2. Handle BIPUSH (takes one u8 argument)
    (BIPUSH $val:expr $(, $($rest:tt)*)?) => {{
        let mut v = Vec::new();
        v.push(op::BIPUSH);
        v.push($val as u8);
        $( v.extend(bytecode!($($rest)*)); )?
        v
    }};

    // 3. Handle instructions with no arguments (HALT, NOP, SWP, DUP, IPOP)
    ($op:ident $(, $($rest:tt)*)?) => {{
        let mut v = Vec::new();
        v.push(op::$op);
        $( v.extend(bytecode!($($rest)*)); )?
        v
    }};

    // 4. Base case: nothing left
    () => { Vec::new() };
}