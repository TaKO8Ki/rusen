use crate::cpu::Cpu;

impl Cpu {
    pub fn fetch_memory8(&self, address: u16) -> u8 {
        let value = self.ram[address as usize];
        value
    }

    pub fn set_memory8(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value
    }
}
