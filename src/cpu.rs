use crate::addressing;
use crate::variable::INSTRUCTIONS;

const prg_rom_page_size: u16 = 16 * 1024;
const chr_rom_page_size: u16 = 8 * 1024;

pub struct Cpu {
    pub register: Register,
    pub ram: [u16; 0x10000],
}

pub struct Register {
    pub a: u16,
    pub x: u16,
    pub y: u16,
    pub s: u16,
    pub p: u16,
    pub sp: u16,
    pub pc: u16,
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            register: Register::default(),
            ram: [0x999; 0x10000],
        }
    }
}

impl Default for Register {
    fn default() -> Self {
        Register {
            a: 0x999,
            x: 0x999,
            y: 0x999,
            s: 0x999,
            p: 0x999,
            sp: 0x999,
            pc: 0x999,
        }
    }
}

impl Cpu {
    pub fn new(&mut self) {
        self.register.s = 0xfd;
        self.register.p = 0x34;
        let lower = self.fetch_memory8(0xfffc);
        let upper = self.fetch_memory8(0xfffd);
    }

    fn load(&mut self, rom: Vec<u16>) {
        let prg_addr = 0x0010;
        let prg_page = rom[4];

        let chr_addr = prg_addr + prg_page * prg_rom_page_size;
        let chr_page = rom[5];

        let prg_bytes = rom
            .get(prg_addr as usize..(prg_addr + prg_page * prg_rom_page_size) as usize)
            .unwrap();
        let chr_bytes =
            rom.get(chr_addr as usize..(chr_addr + chr_page * chr_rom_page_size) as usize);

        for byte in prg_bytes {
            self.ram[(0x8000 + byte) as usize] = prg_bytes[*byte as usize];
            if prg_page == 1 {
                self.ram[(0x8000 + byte + 0x4000) as usize] = prg_bytes[*byte as usize]
            }
        }
    }

    fn read(addr: String) {}

    pub fn fetch(&self, index: u16) -> u16 {
        self.ram[(self.register.pc + index) as usize]
    }

    fn reset() {}

    fn exec(&mut self) {
        let pre_pc = self.register.pc;
        let opecode = self.fetch(0) as usize;
        let (instruction, addressing) = (INSTRUCTIONS[opecode][0], INSTRUCTIONS[opecode][1]);

        let addr = match addressing {
            "impl" => self.implied(),
            "A" => self.accumulator(),
            "#" => self.immediate(),
            "zpg" => self.zero_page(),
            "zpg.X" => self.zero_page_x(),
            "zpg.Y" => self.zero_page_y(),
            "abs" => self.absolute(),
            "abs.X" => self.absolute_x(),
            "abs.Y" => self.absolute_y(),
            "rel" => self.relative(),
            _ => None,
        };
    }
}
