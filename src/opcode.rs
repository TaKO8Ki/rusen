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
}
