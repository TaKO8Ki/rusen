use crate::cpu::Cpu;

impl Cpu {
    pub fn flag_n(&mut self, b: u8) {
        if b & 0x80 != 0 {
            self.register.p = self.register.p | 0x80
        } else {
            self.register.p = self.register.p & 0x7f
        }
    }

    pub fn flag_v(&mut self, b0: u8, b1: u8, b: u8) {
        if ((b0 >> 7) ^ (b1 >> 7) != 0) && (b1 != b) {
            self.register.p = self.register.p | 0x40
        } else {
            self.register.p = self.register.p & 0xbf
        }
    }

    pub fn flag_z(&mut self, b: u8) {
        if b == 0 {
            self.register.p = self.register.p | 0x02
        } else {
            self.register.p = self.register.p & 0xfd
        }
    }

    pub fn flag_i(&mut self, active: bool) {
        if active {
            self.register.p = self.register.p | 0x04u8;
        } else {
            self.register.p = self.register.p & (!0x04u8);
        }
    }

    pub fn flag_c(&mut self, instruction: String, b: u8) {
        if (b as u16 >> 8) != 0 {
            if instruction == "ADC" {
                self.set_c_flag()
            } else {
                self.clear_c_flag()
            }
        } else {
            if instruction == "ADC" {
                self.clear_c_flag()
            } else {
                self.set_c_flag()
            }
        }
    }

    pub fn set_c_flag(&mut self) {
        self.register.p = self.register.p | 0x01
    }

    pub fn clear_c_flag(&mut self) {
        self.register.p = self.register.p & 0xfe
    }
}
