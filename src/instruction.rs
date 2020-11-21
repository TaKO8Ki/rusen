use crate::nes::Nes;

const PPU_ADDR: u16 = 0x2006;
const PPU_DATA: u16 = 0x2007;
const SPRITE_DMA: u16 = 0x4014;

#[derive(Debug)]
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

impl Nes {
    pub fn adc(&mut self, addr: u16) {
        let c_flag = self.cpu.p & 0x01;
        let a_flag = self.cpu.a;
        let value = self.cpu.a + self.fetch_memory8(addr) + c_flag;
        self.cpu.a = value;
        self.flag_n(value);
        self.flag_v(a_flag, value, value);
        self.flag_z(value);
        self.flag_c(Instruction::ADC, value as u16);
    }

    pub fn sbc(&mut self, addr: u16) {
        let not_c_flag = !self.cpu.p & 0x01;
        let a_flag = self.cpu.a;
        let value = self.cpu.a - self.fetch_memory8(addr) - not_c_flag;
        self.cpu.a = value;
        self.flag_n(value);
        self.flag_v(a_flag, value, value);
        self.flag_z(value);
        self.flag_c(Instruction::SBC, value as u16);
    }

    pub fn and(&mut self, addr: u16) {
        let value = self.cpu.a & self.fetch_memory8(addr);
        self.cpu.a = value;
        self.flag_n(value);
        self.flag_z(value);
    }

    pub fn ora(&mut self, addr: u16) {
        let value = self.cpu.a | self.fetch_memory8(addr);
        self.cpu.a = value;
        self.flag_n(value);
        self.flag_z(value);
    }

    pub fn eor(&mut self, addr: u16) {
        let value = self.cpu.a | self.fetch_memory8(addr);
        self.cpu.a = value;
        self.flag_n(value);
        self.flag_z(value);
    }

    pub fn asl(&mut self, _addr: u8) {
        if self.cpu.a & 0x80 > 0 {
            self.set_c_flag()
        } else {
            self.clear_c_flag()
        }
        self.cpu.a <<= 1;
        self.flag_n(self.cpu.a);
        self.flag_z(self.cpu.a);
    }

    pub fn lsr(&mut self, _addr: u8) {
        if self.cpu.a & 0x01 > 0 {
            self.set_c_flag()
        } else {
            self.clear_c_flag()
        }
        self.cpu.a >>= 1;
        self.flag_n(self.cpu.a);
        self.flag_z(self.cpu.a);
    }

    pub fn rol(&mut self, _addr: u8) {
        let c_flag = self.cpu.p & 0x01;
        if self.cpu.a & 0x80 > 0 {
            self.set_c_flag()
        } else {
            self.clear_c_flag()
        }

        self.cpu.a <<= 1;
        if c_flag > 0 {
            self.cpu.a |= 0x01
        } else {
            self.cpu.a &= 0xfe
        }

        self.flag_n(self.cpu.a);
        self.flag_z(self.cpu.a)
    }

    pub fn ror(&mut self, _addr: u8) {
        let c_flag = self.cpu.p & 0x01;
        if self.cpu.a & 0x01 > 0 {
            self.set_c_flag()
        } else {
            self.clear_c_flag()
        }

        self.cpu.a >>= 1;
        if c_flag > 0 {
            self.cpu.a |= 0x80
        } else {
            self.cpu.a &= 0x7f
        }

        self.flag_n(self.cpu.a);
        self.flag_z(self.cpu.a);
    }

    pub fn rti(&mut self) {
        let sr = self.fetch_memory8(0x100 + self.cpu.s + 1);
        self.cpu.s += 1;
        self.cpu.p = sr;
        let lower = self.fetch_memory8(0x100 + self.cpu.s + 1);
        self.cpu.s += 1;
        let upper = self.fetch_memory8(0x100 + self.cpu.s + 1);
        self.cpu.s += 1;
        self.cpu.pc = (upper as u16) << 8 | lower as u16;
        self.ram[0x2000] |= 0x80;
    }

    pub fn bcc(&mut self, addr: u16) {
        let c_flag = self.cpu.p & 0x01;
        if c_flag == 0 {
            self.cpu.pc = addr
        }
    }

    pub fn bcs(&mut self, addr: u16) {
        let c_flag = self.cpu.p & 0x01;
        if c_flag > 0 {
            self.cpu.pc = addr
        }
    }

    pub fn beq(&mut self, addr: u16) {
        let z_flag = self.cpu.p & 0x02;
        if z_flag > 0 {
            self.cpu.pc = addr
        }
    }

    pub fn bne(&mut self, addr: u16) {
        let z_flag = self.cpu.p & 0x02;
        if z_flag == 0 {
            self.cpu.pc = addr
        }
    }

    pub fn bmi(&mut self, addr: u16) {
        let n_flag = self.cpu.p & 0x80;
        if n_flag > 0 {
            self.cpu.pc = addr
        }
    }

    pub fn bpl(&mut self, addr: u16) {
        let n_flag = self.cpu.p & 0x80;
        if n_flag == 0 {
            self.cpu.pc = addr
        }
    }

    pub fn bvc(&mut self, addr: u16) {
        let v_flag = self.cpu.p & 0x40;
        if v_flag == 0 {
            self.cpu.pc = addr
        }
    }
    pub fn bvs(&mut self, addr: u16) {
        let v_flag = self.cpu.p & 0x40;
        if v_flag > 0 {
            self.cpu.pc = addr
        }
    }

    pub fn bit(&mut self, addr: u16) {
        let value = self.fetch_memory8(addr);
        self.flag_z(value & self.cpu.a);
        self.flag_n(value);
        if (value & 0x40) != 0 {
            self.cpu.p |= 0x40
        } else {
            self.cpu.p &= 0xbf
        }
    }

    pub fn jmp(&mut self, addr: u16) {
        self.cpu.pc = addr
    }

    pub fn jsr(&mut self, addr: u16) {
        let upper = (self.cpu.pc - 1) >> 8;
        let lower = self.cpu.pc - 1;
        self.set_memory8(0x100 + self.cpu.s, upper as u8);
        self.set_memory8(0x100 + self.cpu.s - 1, lower as u8);
        self.cpu.s -= 2;
        self.cpu.pc = addr as u16;
    }

    pub fn rts(&mut self) {
        let lower = self.fetch_memory8(0x100 + self.cpu.s as u16 + 1);
        self.cpu.s += 1;
        let upper = self.fetch_memory8(0x100 + self.cpu.s as u16 + 1);
        self.cpu.s += 1;
        self.cpu.pc = ((upper as u16) << 8 | lower as u16) as u16;
        self.cpu.pc += 1;
    }

    pub fn brk(&mut self) {
        let iflag = self.cpu.p & 0x04;
        if iflag == 0 {
            self.cpu.p |= 0x10;
            self.cpu.pc += 1;

            let upper0 = (self.cpu.pc) >> 8;
            let lower0 = self.cpu.pc;
            self.set_memory8(0x100 + self.cpu.s as u16, upper0 as u8);
            self.set_memory8(0x100 + self.cpu.s as u16 - 1, lower0 as u8);
            self.set_memory8(0x100 + self.cpu.s as u16 - 2, self.cpu.p);
            self.cpu.s -= 3;

            self.cpu.p |= 0x04;

            let upper1 = self.fetch_memory8(0xffff);
            let lower1 = self.fetch_memory8(0xfffe);
            self.cpu.pc = ((upper1 as u16) << 8) as u16 | lower1 as u16
        }
    }

    pub fn txs(&mut self) {
        self.cpu.s = self.cpu.x as u16 | 0x0100;
        self.flag_n(self.cpu.s as u8);
        self.flag_z(self.cpu.s as u8)
    }

    pub fn sei(&mut self) {
        self.cpu.p |= 0x04
    }

    pub fn lda(&mut self, addr: u16) {
        if addr == 0x2007 {
            self.cpu.a = self.ppu.ram[self.ppu.ptr as usize];
            self.ppu.ptr += self.get_vram_delta()
        } else {
            self.cpu.a = self.fetch_memory8(addr);
        }
        self.flag_n(self.cpu.a);
        self.flag_z(self.cpu.a);
    }

    pub fn ldx(&mut self, addr: u16) {
        if addr == 0x2007 {
            self.cpu.x = self.ppu.ram[self.ppu.ptr as usize];
            self.ppu.ptr += self.get_vram_delta()
        } else {
            self.cpu.x = self.fetch_memory8(addr);
        }
        self.flag_n(self.cpu.x);
        self.flag_z(self.cpu.x);
    }

    pub fn ldy(&mut self, addr: u16) {
        if addr == 0x2007 {
            self.cpu.y = self.ppu.ram[self.ppu.ptr as usize];
            self.ppu.ptr += self.get_vram_delta()
        } else {
            self.cpu.y = self.fetch_memory8(addr);
        }
        self.flag_n(self.cpu.y);
        self.flag_z(self.cpu.y);
    }

    pub fn sta(&mut self, addr: u16) {
        match addr {
            0x2004 => {
                self.ppu.s_ram[self.ram[0x2003] as usize] = self.cpu.a;
                self.ram[0x2003] += 1
            }
            0x2005 => {
                if self.ppu.scroll_flag {
                    self.ppu.scroll[1] = self.cpu.a;
                } else {
                    self.ppu.scroll[0] = self.cpu.a;
                    self.ppu.scroll_flag = true
                }
            }
            PPU_ADDR => self.ppu.ptr = self.ppu.ptr << 8 | self.cpu.a as u16,
            PPU_DATA => self.set_vram(self.cpu.a),
            SPRITE_DMA => {
                let start = (self.cpu.a as u16) << 8;
                for i in 0..256 {
                    self.ppu.s_ram[i] = self.fetch_memory8(start as u16 + i as u16)
                }
            }
            0x2009 => (),
            _ => (),
        };
        self.set_memory8(addr, self.cpu.a)
    }

    pub fn stx(&mut self, addr: u16) {
        match addr {
            0x2004 => {
                self.ppu.s_ram[self.ram[0x2003] as usize] = self.cpu.x;
                self.ram[0x2003] += 1
            }
            0x2005 => {
                if self.ppu.scroll_flag {
                    self.ppu.scroll[1] = self.cpu.x;
                } else {
                    self.ppu.scroll[0] = self.cpu.x;
                    self.ppu.scroll_flag = true
                }
            }
            PPU_ADDR => self.ppu.ptr = self.ppu.ptr << 8 | self.cpu.x as u16,
            PPU_DATA => self.set_vram(self.cpu.x),
            SPRITE_DMA => {
                let start = (self.cpu.x as u16) << 8;
                for i in 0..256 {
                    self.ppu.s_ram[i] = self.fetch_memory8(start as u16 + i as u16)
                }
            }
            0x2009 => (),
            _ => (),
        };
        self.set_memory8(addr, self.cpu.x)
    }

    pub fn sty(&mut self, addr: u16) {
        match addr {
            0x2004 => {
                self.ppu.s_ram[self.ram[0x2003] as usize] = self.cpu.y;
                self.ram[0x2003] += 1
            }
            0x2005 => {
                if self.ppu.scroll_flag {
                    self.ppu.scroll[1] = self.cpu.y;
                } else {
                    self.ppu.scroll[0] = self.cpu.y;
                    self.ppu.scroll_flag = true
                }
            }
            PPU_ADDR => self.ppu.ptr = self.ppu.ptr << 8 | self.cpu.y as u16,
            PPU_DATA => self.set_vram(self.cpu.y),
            SPRITE_DMA => {
                let start = (self.cpu.y as u16) << 8;
                for i in 0..256 {
                    self.ppu.s_ram[i] = self.fetch_memory8(start as u16 + i as u16)
                }
            }
            0x2009 => (),
            _ => (),
        };
        self.set_memory8(addr, self.cpu.y)
    }

    pub fn inx(&mut self) {
        self.cpu.x += 1;
        self.flag_n(self.cpu.x);
        self.flag_z(self.cpu.x);
    }

    pub fn dex(&mut self) {
        self.cpu.x -= 1;
        self.flag_n(self.cpu.x);
        self.flag_z(self.cpu.x);
    }

    pub fn iny(&mut self) {
        self.cpu.y += 1;
        self.flag_n(self.cpu.y);
        self.flag_z(self.cpu.y);
    }

    pub fn dey(&mut self) {
        self.cpu.y -= 1;
        self.flag_n(self.cpu.y);
        self.flag_z(self.cpu.y);
    }

    pub fn cmp(&mut self, addr: u16) {
        let value = self.cpu.a - self.fetch_memory8(addr);
        let value16 = self.cpu.a - self.fetch_memory8(addr);

        self.flag_n(value);
        self.flag_z(value);
        self.flag_c(Instruction::CMP, value16 as u16);
    }

    pub fn cpx(&mut self, addr: u16) {
        let value = self.cpu.x - self.fetch_memory8(addr);
        let value16 = self.cpu.x - self.fetch_memory8(addr);

        self.flag_n(value);
        self.flag_z(value);
        self.flag_c(Instruction::CPX, value16 as u16);
    }

    pub fn cpy(&mut self, addr: u16) {
        let value = self.cpu.y - self.fetch_memory8(addr);
        let value16 = self.cpu.y - self.fetch_memory8(addr);

        self.flag_n(value);
        self.flag_z(value);
        self.flag_c(Instruction::CPY, value16 as u16);
    }

    pub fn inc(&mut self, addr: u16) {
        self.set_memory8(addr, self.fetch_memory8(addr) + 1)
    }

    pub fn dec(&mut self, addr: u16) {
        self.set_memory8(addr, self.fetch_memory8(addr) - 1)
    }

    pub fn sec(&mut self) {
        self.cpu.p |= 0x01
    }

    pub fn cli(&mut self) {
        self.flag_i(false)
    }

    pub fn clc(&mut self, addr: u16) {
        self.flag_c(Instruction::CLC, addr)
    }

    pub fn cld(&mut self) {
        self.flag_d(false);
    }

    pub fn sed(&mut self) {
        self.flag_d(true);
    }

    pub fn clv(&mut self) {
        self.cpu.p &= 0xbf
    }

    pub fn tax(&mut self) {
        self.cpu.x = self.cpu.a;
        self.flag_n(self.cpu.x);
        self.flag_z(self.cpu.x)
    }

    pub fn tay(&mut self) {
        self.cpu.y = self.cpu.a;
        self.flag_n(self.cpu.y);
        self.flag_z(self.cpu.y)
    }

    pub fn txa(&mut self) {
        self.cpu.a = self.cpu.x;
        self.flag_n(self.cpu.a);
        self.flag_z(self.cpu.a)
    }

    pub fn tya(&mut self) {
        self.cpu.a = self.cpu.y;
        self.flag_n(self.cpu.a);
        self.flag_z(self.cpu.a)
    }

    pub fn tsx(&mut self) {
        self.cpu.x = self.cpu.s as u8;
        self.flag_n(self.cpu.x);
        self.flag_z(self.cpu.x)
    }

    pub fn pha(&mut self) {
        self.set_memory8(0x100 + self.cpu.s, self.cpu.a);
        self.cpu.s -= 1
    }

    pub fn pla(&mut self) {
        let value = self.fetch_memory8(0x0100 + self.cpu.s + 1);
        self.cpu.a = value;
        self.cpu.s += 1;
        self.flag_n(value);
        self.flag_z(value);
    }

    pub fn php(&mut self) {
        self.set_memory8(0x100 + self.cpu.s, self.cpu.p);
        self.cpu.s -= 1
    }

    pub fn plp(&mut self) {
        let value = self.fetch_memory8(0x0100 + self.cpu.s + 1);
        self.cpu.p = value;
        self.cpu.s += 1;
    }

    pub fn nop(&mut self) {}

    fn set_vram(&mut self, value: u8) {
        if self.ppu.ptr > 0x4000 {}
        self.ppu.ram[self.ppu.ptr as usize] = value;
        if self.ppu.ptr == 0x3f00
            || self.ppu.ptr == 0x3f04
            || self.ppu.ptr == 0x3f08
            || self.ppu.ptr == 0x3f0c
        {
            self.ppu.ram[self.ppu.ptr as usize + 0x10] = value
        } else if self.ppu.ptr == 0x3f10
            || self.ppu.ptr == 0x3f14
            || self.ppu.ptr == 0x3f18
            || self.ppu.ptr == 0x3f1c
        {
            self.ppu.ram[self.ppu.ptr as usize - 0x10] = value
        } else if !self.ppu.mirror
            && ((0x2000 <= self.ppu.ptr && self.ppu.ptr < 0x2400)
                || (0x2800 <= self.ppu.ptr && self.ppu.ptr < 0x2c00))
        {
            self.ppu.ram[self.ppu.ptr as usize + 0x0400] = value
        } else if self.ppu.mirror && (0x2000 <= self.ppu.ptr && self.ppu.ptr < 0x2800) {
            self.ppu.ram[self.ppu.ptr as usize + 0x0800] = value
        }
        self.ppu.ptr += self.get_vram_delta()
    }

    fn get_vram_delta(&mut self) -> u16 {
        let value = self.ram[0x2000];
        if (value & 0x04) > 0 {
            return 32;
        }
        1
    }
}
