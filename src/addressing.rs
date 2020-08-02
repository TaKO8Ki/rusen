use crate::cpu::Cpu;

impl Cpu {
    pub fn implied(&mut self) -> Option<u16> {
        self.register.pc += 1;
        return None;
    }

    pub fn accumulator(&mut self) -> Option<u16> {
        self.register.pc += 1;
        return None;
    }

    pub fn immediate(&mut self) -> Option<u16> {
        let addr = self.register.pc + 1;
        self.register.pc += 2;
        return Some(addr);
    }

    pub fn zero_page(&mut self) -> Option<u16> {
        let lower = self.fetch(1);
        let upper = 0x00;
        let addr = upper << 8 | lower;

        self.register.pc += 2;
        Some(addr)
    }

    pub fn zero_page_x(&mut self) -> Option<u16> {
        let lower = self.fetch(1) + self.register.x;
        let upper = 0x00;
        let addr = upper << 8 | lower;

        self.register.pc += 2;
        Some(addr)
    }

    pub fn zero_page_y(&mut self) -> Option<u16> {
        let lower = self.fetch(1) + self.register.y;
        let upper = 0x00;
        let addr = upper << 8 | lower;

        self.register.pc += 2;
        Some(addr)
    }

    pub fn absolute(&mut self) -> Option<u16> {
        let lower = self.fetch(1);
        let upper = self.fetch(2);
        let addr = upper << 8 | lower;

        self.register.pc += 3;
        Some(addr)
    }

    pub fn absolute_x(&mut self) -> Option<u16> {
        let lower = self.fetch(1);
        let upper = self.fetch(2);
        let addr = upper << 8 | lower + self.register.x;

        self.register.pc += 3;
        Some(addr)
    }

    pub fn absolute_y(&mut self) -> Option<u16> {
        let lower = self.fetch(1);
        let upper = self.fetch(2);
        let addr = upper << 8 | lower + self.register.y;

        self.register.pc += 3;
        Some(addr)
    }

    pub fn relative(&mut self) -> Option<u16> {
        let lower = self.register.pc + 1;
        let upper = self.register.pc + 2;
        let addr = upper << 8 | lower + self.register.y;

        self.register.pc += 2;
        Some(addr)
    }
}
