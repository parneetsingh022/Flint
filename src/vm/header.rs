pub struct Header {
    pub magic: [u8; 4],   // Magic number (e.g., [b'F', b'L', b'N', b'T'])
    pub version: u8,      // version field
    pub code_start: u32,  // Offset where instructions begin
    pub data_start: u32,  // Offset where strings/constants begin
    pub size: u8
}

impl Header {
    pub fn new() -> Self {
        Self {
            magic: *b"FLNT", // Initializes with 4-byte magic string
            version: 0,      // Start with version 0
            code_start: 0,   // Placeholder: will be updated during assembly
            data_start: 0,   // Placeholder: will be updated during assembly
            size : 13,
        }
    }


    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.magic);
        bytes.push(self.version);
        bytes.extend(&self.code_start.to_be_bytes());
        bytes.extend(&self.data_start.to_be_bytes());
        bytes.push(self.size);
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let size : u8 = 13;
        if bytes.len() < size as usize{
            return Err("Buffer too small to contain a valid header".to_string());
        }

        let mut magic = [0u8; 4];
        magic.copy_from_slice(&bytes[0..4]);

        if &magic != b"FLNT" {
            return Err("Invalid magic number: Not a Flint binary".to_string());
        }

        let version = bytes[4];
        
        let code_start = u32::from_be_bytes(bytes[5..9].try_into().unwrap());
        let data_start = u32::from_be_bytes(bytes[9..13].try_into().unwrap());

        Ok(Self {
            magic,
            version,
            code_start,
            data_start,
            size,
        })
    }
}