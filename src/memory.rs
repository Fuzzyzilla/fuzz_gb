use crate::bitmath::join_u8;

pub struct Memory {
    data : [u8; 0xffff],
}

impl Memory {
    pub fn read(&self, addr : u16) -> u8 {
        self.data[addr as usize]
    }
    pub fn read_mut<'a>(&'a mut self, addr : u16) -> &'a mut u8 {
        &mut self.data[addr as usize]
    }
    pub fn write(&mut self, addr : u16, data : u8) -> () {
        self.data[addr as usize] = data;
    }
    pub fn write_u16(&mut self, addr : u16, data : u16) -> () {
        self.write(addr,                (data & 0xff) as u8);
        self.write(addr.wrapping_add(1),(data >> 8) as u8);
    }
    pub fn read_u16(&self, addr : u16) -> u16 {
        join_u8(
            self.read(addr),
            self.read(addr.wrapping_add(1))
        )
    }
}