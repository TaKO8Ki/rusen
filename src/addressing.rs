use crate::nes::Nes;

#[derive(Debug)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    Absolute,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    AbsoluteX,
    AbsoluteY,
    Relative,
    Indirect,
    IndirectX,
    IndirectY,
}

impl Nes {
    pub fn implied(&mut self) -> u16 {
        self.cpu.pc += 1;
        0
    }

    pub fn accumulator(&mut self) -> u16 {
        self.cpu.pc += 1;
        self.cpu.a as u16
    }

    pub fn immediate(&mut self) -> u16 {
        let addr = self.cpu.pc + 1;
        self.cpu.pc += 2;
        addr
    }

    pub fn zero_page(&mut self) -> u16 {
        let lower = self.fetch_code8(1);
        let upper = 0x00;
        let addr = upper << 8 | lower as u16;

        self.cpu.pc += 2;
        addr
    }

    pub fn zero_page_x(&mut self) -> u16 {
        let lower = self.fetch_code8(1) + self.cpu.x;
        let upper = 0x00;
        let addr = (upper as u16) << 8 | lower as u16;

        self.cpu.pc += 2;
        addr
    }

    pub fn zero_page_y(&mut self) -> u16 {
        let lower = self.fetch_code8(1) + self.cpu.y;
        let upper = 0x00;
        let addr = (upper as u16) << 8 | lower as u16;

        self.cpu.pc += 2;
        addr
    }

    pub fn absolute(&mut self) -> u16 {
        let lower = self.fetch_code8(1);
        let upper = self.fetch_code8(2);
        let addr = (upper as u16) << 8 | lower as u16;

        self.cpu.pc += 3;
        addr
    }

    pub fn absolute_x(&mut self) -> u16 {
        let lower = self.fetch_code8(1);
        let upper = self.fetch_code8(2);
        let addr = (upper as u16) << 8 | (lower + self.cpu.x) as u16;

        self.cpu.pc += 3;
        addr
    }

    pub fn absolute_y(&mut self) -> u16 {
        let lower = self.fetch_code8(1);
        let upper = self.fetch_code8(2);
        let addr = (upper as u16) << 8 | (lower + self.cpu.y) as u16;

        self.cpu.pc += 3;
        addr
    }

    pub fn relative(&mut self) -> u16 {
        let delta = self.fetch_code8(1);
        self.cpu.pc += 2;
        let addr = self.cpu.pc as i32 + (delta as i8) as i32;
        addr as u16
    }

    pub fn indexed_indirect(&mut self) -> u16 {
        let lower = self.fetch_code8(1) + self.cpu.x;
        let upper = 0x00;
        let addr = (upper << 8 as u16) | lower as u16;
        let lower = self.fetch_memory8(addr as u16);
        let upper = self.fetch_memory8((addr + 1) as u16);
        let addr = (upper as u16) << 8 | lower as u16;

        self.cpu.pc += 2;
        addr
    }

    pub fn indirect_indexed(&mut self) -> u16 {
        let lower = self.fetch_code8(1) + self.cpu.x;
        let upper = 0x00;
        let addr = (upper as u16) << 8 | lower as u16;
        let lower = self.fetch_memory8(addr as u16);
        let upper = self.fetch_memory8(addr as u16 + 1);
        let addr = (upper as u16) << 8 | lower as u16;

        self.cpu.pc += 2;
        addr
    }

    pub fn absolute_indirect(&mut self) -> u16 {
        let addr = self.absolute();
        let lower = self.fetch_memory8(addr);
        let upper = self.fetch_memory8(addr) + 1;
        let addr = (upper as u16) << 8 | lower as u16;
        addr as u16
    }
}
