use crate::cpu::Cpu;

impl Cpu {
    pub fn flag_n(&mut self, b: u16) {
        if b & 0x80 != 0 {
            self.register.p = self.register.p | 0x80
        } else {
            self.register.p = self.register.p & 0x7f
        }
    }

    pub fn flag_v(&mut self, b0: u16, b1: u16, b: u16) {
        if ((b0 >> 7) ^ (b1 >> 7) != 0) && (b1 != b) {
            self.register.p = self.register.p | 0x40
        } else {
            self.register.p = self.register.p & 0xbf
        }
    }

    pub fn flag_z(&mut self, b: u16) {
        if b == 0 {
            self.register.p = self.register.p | 0x02
        } else {
            self.register.p = self.register.p & 0xfd
        }
    }

    pub fn flag_c(&mut self, instruction: String, b: u16) {
        if (b >> 8) != 0 {
            if instruction == "ADC" {
                self.register.p = self.register.p | 0x01
            } else {
                self.register.p = self.register.p & 0xfe
            }
        } else {
            if instruction == "ADC" {
                self.register.p = self.register.p & 0xfe
            } else {
                self.register.p = self.register.p | 0x01
            }
        }
    }
}
