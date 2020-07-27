use crate::cpu::Cpu;

impl Cpu {
    pub fn fetch_memory8(&self, address: u16) -> u16 {
        let value = self.ram[address as usize];
        value
    }
}
