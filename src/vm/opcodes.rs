
pub mod op {
    pub const NOP      : u8   = 0;
    pub const HALT     : u8   = 1;
    pub const IPUSH    : u8   = 2;
    pub const IPOP     : u8   = 3;
    pub const BIPUSH   : u8   = 4;  // PUSH a single byte as integer
    pub const SWP      : u8   = 5;
    pub const DUP      : u8   = 6;
}