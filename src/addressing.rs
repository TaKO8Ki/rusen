use crate::cpu::Cpu;

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

impl Cpu {
    pub fn implied(&mut self) -> u16 {
        self.register.pc += 1;
        0
    }

    pub fn accumulator(&mut self) -> u16 {
        self.register.pc += 1;
        self.register.a as u16
    }

    pub fn immediate(&mut self) -> u16 {
        let addr = self.register.pc + 1;
        self.register.pc += 2;
        addr
    }

    pub fn zero_page(&mut self) -> u16 {
        let lower = self.fetch_code8(1);
        let upper = 0x00;
        let addr = upper << 8 | lower as u16;

        self.register.pc += 2;
        addr
    }

    pub fn zero_page_x(&mut self) -> u16 {
        let lower = self.fetch_code8(1) + self.register.x;
        let upper = 0x00;
        let addr = (upper as u16) << 8 | lower as u16;

        self.register.pc += 2;
        addr
    }

    pub fn zero_page_y(&mut self) -> u16 {
        let lower = self.fetch_code8(1) + self.register.y;
        let upper = 0x00;
        let addr = (upper as u16) << 8 | lower as u16;

        self.register.pc += 2;
        addr
    }

    pub fn absolute(&mut self) -> u16 {
        let lower = self.fetch_code8(1);
        let upper = self.fetch_code8(2);
        let addr = (upper as u16) << 8 | lower as u16;

        self.register.pc += 3;
        addr
    }

    pub fn absolute_x(&mut self) -> u16 {
        let lower = self.fetch_code8(1);
        let upper = self.fetch_code8(2);
        let addr = (upper as u16) << 8 | (lower + self.register.x) as u16;

        self.register.pc += 3;
        addr
    }

    pub fn absolute_y(&mut self) -> u16 {
        let lower = self.fetch_code8(1);
        let upper = self.fetch_code8(2);
        let addr = (upper as u16) << 8 | (lower + self.register.y) as u16;

        self.register.pc += 3;
        addr
    }

    pub fn relative(&mut self) -> u16 {
        let lower = self.fetch_code8(1);
        let upper = self.register.pc + 2;
        let addr = upper << 8 | (lower + self.register.y) as u16;

        self.register.pc += 2;
        addr
    }

    pub fn indexed_indirect(&mut self) -> u16 {
        let lower = self.fetch_code8(1) + self.register.x;
        let upper = 0x00;
        let addr = (upper << 8 as u16) | lower as u16;
        let lower = self.fetch_memory8(addr as u16);
        let upper = self.fetch_memory8((addr + 1) as u16);
        let addr = (upper as u16) << 8 | lower as u16;

        self.register.pc += 2;
        addr
    }

    pub fn indirect_indexed(&mut self) -> u16 {
        let lower = self.fetch_code8(1) + self.register.x;
        let upper = 0x00;
        let addr = (upper as u16) << 8 | lower as u16;
        let lower = self.fetch_memory8(addr as u16);
        let upper = self.fetch_memory8(addr as u16 + 1);
        let addr = (upper as u16) << 8 | lower as u16;

        self.register.pc += 2;
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
