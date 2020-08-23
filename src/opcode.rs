use crate::cpu::Cpu;

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
}
