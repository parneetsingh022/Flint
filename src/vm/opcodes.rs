
pub mod op {
    pub const NOP      : u8   = 0;
    pub const HALT     : u8   = 1;
    pub const IPUSH    : u8   = 2;
    pub const IPOP     : u8   = 3;
    pub const BIPUSH   : u8   = 4;  // PUSH a single byte as integer
    pub const SWP      : u8   = 5;
    pub const DUP      : u8   = 6;
    pub const NEG      : u8   = 7;
    pub const ADD      : u8   = 8;
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