use crate::cpu::Cpu;

impl Cpu {
    pub fn flag_n(&mut self, b: u16) {
        if b & 0x80 != 0 {
            self.register.p = self.register.p | 0x80
        } else {
            self.register.p = self.register.p & 0x7f
        }
    }
}
