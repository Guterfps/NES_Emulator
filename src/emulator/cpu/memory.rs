pub struct Memory {
    pub mem: [u8; MEMORY_SIZE],
}

const MEMORY_SIZE: usize = 0xFFFF;

impl Memory {
    pub fn new() -> Self {
        Memory {
            mem: [0; MEMORY_SIZE],
        }
    }

    pub fn mem_read(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) {
        self.mem[addr as usize] = data;
    }

    pub fn mem_read_u16(&mut self, pos: u16) -> u16 {
        u16::from_le_bytes([self.mem_read(pos), self.mem_read(pos + 1)])
    }

    pub fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let bytes = data.to_le_bytes();
        self.mem_write(pos, bytes[0]);
        self.mem_write(pos + 1, bytes[1]);
    }
}
