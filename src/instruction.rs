use crate::cpu::Cpu;

pub enum Instruction {
    ADC,
    SBC,
    AND,
    EOR,
    ORA,
    ASL,
    LSR,
    ROL,
    ROR,
    INC,
    INX,
    INY,
    DEC,
    DEX,
    DEY,
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,
    SEC,
    SED,
    SEI,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    JMP,
    JSR,
    RTI,
    RTS,
    BCC,
    BCS,
    BEQ,
    BMI,
    BNE,
    BPL,
    BVC,
    BVS,
    PHA,
    PHP,
    PLA,
    PLP,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    BRK,
    BIT,
    NOP,
}

impl Cpu {
    pub fn adc(&mut self, addr: u16) {
        let c_flag = self.register.p & 0x01;
        let a_flag = self.register.a;
        let value = (self.register.a + self.fetch_memory(addr) + c_flag) & (0xff);
        let value16 = self.register.a + self.fetch_memory(addr) + c_flag;
        self.register.a = value;
        self.flag_n(value);
        self.flag_v(a_flag, value, value16);
        self.flag_z(value);
        self.flag_c("ADC".to_string(), value16);
    }

    pub fn sbc(&mut self, addr: u16) {
        let not_c_flag = !self.register.p & 0x01;
        let a_flag = self.register.a;
        let value = (self.register.a - self.fetch_memory(addr) - not_c_flag) & (0xff);
        let value16 = self.register.a - self.fetch_memory(addr) - not_c_flag;
        self.register.a = value;
        self.flag_n(value);
        self.flag_v(a_flag, value, value16);
        self.flag_z(value);
        self.flag_c("SBC".to_string(), value16);
    }

    pub fn and(&mut self, addr: u16) {
        let value = self.register.a & self.fetch_memory(addr);
        self.register.a = value;
        self.flag_n(value);
        self.flag_z(value);
    }

    pub fn ora(&mut self, addr: u16) {
        let value = self.register.a | self.fetch_memory(addr);
        self.register.a = value;
        self.flag_n(value);
        self.flag_z(value);
    }

    pub fn eor(&mut self, addr: u16) {
        let value = self.register.a | self.fetch_memory(addr);
        self.register.a = value;
        self.flag_n(value);
        self.flag_z(value);
    }

    pub fn asl(&mut self, addr: u16, a: bool) {
        if a {
            if self.register.a & 0x80 > 0 {
                self.set_c_flag()
            } else {
                self.clear_c_flag()
            }
            self.register.a = self.register.a << 1;
            self.flag_n(self.register.a);
            self.flag_z(self.register.a);
        } else {
            let mut value = self.fetch_memory(addr);

            if value & 0x80 > 0 {
                self.set_c_flag()
            } else {
                self.clear_c_flag()
            }
            value = value << 1;
            self.set_memory(addr, value);
            self.flag_n(value);
            self.flag_z(value)
        }
    }

    pub fn lsr(&mut self, addr: u16, a: bool) {
        if a {
            if self.register.a & 0x01 > 0 {
                self.set_c_flag()
            } else {
                self.clear_c_flag()
            }
            self.register.a = self.register.a >> 1;
            self.flag_n(self.register.a);
            self.flag_z(self.register.a);
        } else {
            let mut value = self.fetch_memory(addr);
            if value & 0x01 > 0 {
                self.set_c_flag()
            } else {
                self.clear_c_flag()
            }
            value = value >> 1;
            self.set_memory(addr, value);
            self.flag_n(value);
            self.flag_z(value)
        }
    }

    pub fn rol(&mut self, addr: u16, a: bool) {
        if a {
            let c_flag = self.register.p & 0x01;
            if self.register.a & 0x80 > 0 {
                self.set_c_flag()
            } else {
                self.clear_c_flag()
            }

            self.register.a = self.register.a << 1;
            if c_flag > 0 {
                self.register.a |= 0x01
            } else {
                self.register.a &= 0xfe
            }

            self.flag_n(self.register.a);
            self.flag_z(self.register.a)
        } else {
            let mut value = self.fetch_memory(addr);
            let c_flag = self.register.p & 0x01;
            if value & 0x80 > 0 {
                self.set_c_flag()
            } else {
                self.clear_c_flag()
            }

            value = value << 1;
            if c_flag > 0 {
                value |= 0x01
            } else {
                value &= 0xfe
            }

            self.set_memory(addr, value);
            self.flag_n(value);
            self.flag_z(value);
        }
    }

    pub fn ror(&mut self, addr: u16, a: bool) {
        if a {
            let c_flag = self.register.p & 0x01;
            if self.register.a & 0x01 > 0 {
                self.set_c_flag()
            } else {
                self.clear_c_flag()
            }

            self.register.a = self.register.a >> 1;
            if c_flag > 0 {
                self.register.a |= 0x80
            } else {
                self.register.a &= 0x7f
            }

            self.flag_n(self.register.a);
            self.flag_z(self.register.a);
        } else {
            let mut value = self.fetch_memory(addr);
            let c_flag = self.register.p & 0x01;
            if value & 0x01 > 0 {
                self.set_c_flag()
            } else {
                self.clear_c_flag()
            }

            value = value >> 1;
            if c_flag > 0 {
                value |= 0x80
            } else {
                value &= 0x7f
            }

            self.set_memory(addr, value);
            self.flag_n(value);
            self.flag_z(value)
        }
    }

    pub fn bcc(&mut self, addr: u16) {
        let c_flag = self.register.p & 0x01;
        if c_flag == 0 {
            self.register.pc = addr
        }
    }

    pub fn bcs(&mut self, addr: u16) {
        let c_flag = self.register.p & 0x01;
        if c_flag > 0 {
            self.register.pc = addr
        }
    }

    pub fn beq(&mut self, addr: u16) {
        let z_flag = self.register.p & 0x02;
        if z_flag > 0 {
            self.register.pc = addr
        }
    }

    pub fn bne(&mut self, addr: u16) {
        let z_flag = self.register.p & 0x02;
        if z_flag == 0 {
            self.register.pc = addr
        }
    }

    pub fn bmi(&mut self, addr: u16) {
        let n_flag = self.register.p & 0x80;
        if n_flag > 0 {
            self.register.pc = addr
        }
    }

    pub fn bpl(&mut self, addr: u16) {
        let n_flag = self.register.p & 0x80;
        if n_flag == 0 {
            self.register.pc = addr
        }
    }

    pub fn bvc(&mut self, addr: u16) {
        let v_flag = self.register.p & 0x40;
        if v_flag == 0 {
            self.register.pc = addr
        }
    }
    pub fn bvs(&mut self, addr: u16) {
        let v_flag = self.register.p & 0x40;
        if v_flag > 0 {
            self.register.pc = addr
        }
    }

    pub fn bit(&mut self, addr: u16) {
        let value = self.fetch_memory(addr);
        self.flag_z(value & self.register.a);
        self.flag_n(value);
        if (value & 0x40) != 0 {
            self.register.p = self.register.p | 0x40
        } else {
            self.register.p = self.register.p & 0xbf
        }
    }

    pub fn jmp(&mut self, addr: u16) {
        self.register.pc = addr
    }

    pub fn jsr(&mut self, addr: u16) {
        let upper = (self.register.pc - 1) >> 8;
        let lower = self.register.pc - 1;
        self.set_memory(0x100 + self.register.s, upper);
        self.set_memory(0x100 + self.register.s - 1, lower);
        self.register.s -= 2;
        self.register.pc = addr;
    }

    pub fn rts(&mut self) {
        let lower = self.fetch_memory(0x100 + self.register.s + 1);
        self.register.s += 1;
        let upper = self.fetch_memory(0x100 + self.register.s + 1);
        self.register.s += 1;
        self.register.pc = (upper << 8) | lower;
        self.register.pc += 1;
    }

    pub fn brk(&mut self) {
        let iflag = self.register.p & 0x04;
        if iflag == 0 {
            self.register.p = self.register.p | 0x10;
            self.register.pc += 1;

            let upper0 = (self.register.pc) >> 8;
            let lower0 = self.register.pc;
            self.set_memory(0x100 + self.register.s, upper0);
            self.set_memory(0x100 + self.register.s - 1, lower0);
            self.set_memory(0x100 + self.register.s - 2, self.register.p);
            self.register.s -= 3;

            self.register.p = self.register.p | 0x04;

            let upper1 = self.fetch_memory(0xffff);
            let lower1 = self.fetch_memory(0xfffe);
            self.register.pc = (upper1 << 8) | lower1
        }
    }
}
