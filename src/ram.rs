use crate::cpu::Cpu;

impl Cpu {
    pub fn fetch_memory(&self, address: u16) -> u16 {
        let value = self.ram[address as usize];
        value
    }

    pub fn set_memory(&mut self, addr: u16, value: u16) {
        self.ram[addr as usize] = value
    }
}
