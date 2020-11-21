use crate::instruction::Instruction;
use crate::nes::Nes;

impl Nes {
    pub fn set_v_blank(&mut self) {
        self.ram[0x2002] = self.ram[0x2002] | 0x80
    }

    // clearVBlank VBlankを解除
    pub fn clear_v_blank(&mut self) {
        self.ram[0x2002] = self.ram[0x2002] & 0x7f
    }

    pub fn flag_n(&mut self, b: u8) {
        if b & 0x80 != 0 {
            self.cpu.p |= 0x80
        } else {
            self.cpu.p &= 0x7f
        }
    }

    pub fn flag_v(&mut self, b0: u8, b1: u8, b: u8) {
        if ((b0 >> 7) ^ (b1 >> 7) != 0) && (b1 != b) {
            self.cpu.p |= 0x40
        } else {
            self.cpu.p &= 0xbf
        }
    }

    pub fn flag_z(&mut self, b: u8) {
        if b == 0 {
            self.cpu.p |= 0x02
        } else {
            self.cpu.p &= 0xfd
        }
    }

    pub fn flag_i(&mut self, active: bool) {
        if active {
            self.cpu.p |= 0x04;
        } else {
            self.cpu.p &= !0x04;
        }
    }

    pub fn flag_c(&mut self, instruction: Instruction, b: u16) {
        if (b >> 8) != 0 {
            if let Instruction::ADC = instruction {
                self.set_c_flag()
            } else {
                self.clear_c_flag()
            }
        } else if let Instruction::ADC = instruction {
            self.clear_c_flag()
        } else {
            self.set_c_flag()
        }
    }

    pub fn flag_d(&mut self, active: bool) {
        if active {
            self.cpu.p |= 0x08
        } else {
            self.cpu.p &= 0xfb
        }
    }

    pub fn set_c_flag(&mut self) {
        self.cpu.p |= 0x01
    }

    pub fn clear_c_flag(&mut self) {
        self.cpu.p &= 0xfe
    }
}
