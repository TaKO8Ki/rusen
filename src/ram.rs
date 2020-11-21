use crate::nes::Nes;

impl Nes {
    pub fn fetch_memory8(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    pub fn set_memory8(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value
    }
}
