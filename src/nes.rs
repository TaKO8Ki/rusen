use crate::cpu::Cpu;
use crate::ppu::Ppu;

pub struct Nes {
    pub cpu: Cpu,
    pub ppu: Ppu,
    pub ram: [u8; 0x10000],
}

impl Default for Nes {
    fn default() -> Self {
        Nes {
            cpu: Cpu::default(),
            ppu: Ppu::default(),
            ram: [0; 0x10000],
        }
    }
}
