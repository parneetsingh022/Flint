
macro_rules! define_instructions {
    ($(($name:ident, $size:expr)),* $(,)?) => {
        pub mod op {
            $(pub const $name: u8 = op_enum::$name as u8;)*

            #[repr(u8)]
            #[derive(Debug, Clone, Copy, PartialEq)]
            enum op_enum {
                $($name,)*
            }

            pub struct InstructionInfo {
                pub name: &'static str,
                pub size: u32,
            }

            pub fn get_info(code: u8) -> Option<InstructionInfo> {
                $(
                    if code == op_enum::$name as u8 {
                        return Some(InstructionInfo {
                            name: stringify!($name),
                            size: $size,
                        });
                    }
                )*
                None
            }

            pub fn from_mnemonic(m: &str) -> Option<u8> {
                $(
                    if stringify!($name) == m.to_uppercase() {
                        return Some(op_enum::$name as u8);
                    }
                )*
                None
            }
        }
    }
}

define_instructions! {
    // Basic Control
    (NOP,    1),
    (HALT,   1),

    // Stack Operations & Constants
    (IPUSH,  5), // Opcode + 4-byte i32
    (BIPUSH, 2), // Opcode + 1-byte u8
    (FPUSH,  9), // Opcode + 8-byte f64
    (POP,    1),
    (SWP,    1),
    (DUP,    1),

    // Memory Operations
    (STORE,  5), // Opcode + 4-byte address
    (LOAD,   5), // Opcode + 4-byte address

    // Arithmetic
    (NEG,    1),
    (ADD,    1),
    (SUB,    1),
    (MUL,    1),
    (DIV,    1),
    (MOD,    1),

    // Comparison
    (CMP,    1),

    // Control Flow (32-bit Absolute Jumps)
    (JL,     5),
    (JLE,    5),
    (JG,     5),
    (JGE,    5),
    (JE,     5),
    (JNE,    5),
    (JMP,    5),

    // I/O
    (PRINT,  1)
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