use crate::variable::INSTRUCTIONS;

const prg_rom_page_size: u16 = 16 * 1024;
const chr_rom_page_size: u16 = 8 * 1024;

pub struct Cpu {
    pub register: Register,
    pub ram: [u16; 0x10000],
}

pub struct Register {
    pub a: u16,
    pub x: u16,
    pub y: u16,
    pub s: u16,
    // status register
    // 0: C, 1: Z, 2: I, 3: D, 4: B, 5: R, 6: V, 7: N
    pub p: u16,
    pub sp: u16,
    pub pc: u16,
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            register: Register::default(),
            ram: [0x999; 0x10000],
        }
    }
}

impl Default for Register {
    fn default() -> Self {
        Register {
            a: 0x999,
            x: 0x999,
            y: 0x999,
            s: 0x999,
            p: 0x999,
            sp: 0x999,
            pc: 0x999,
        }
    }
}

impl Cpu {
    pub fn new(&mut self) {
        self.register.s = 0xfd;
        self.register.p = 0x34;
        let lower = self.fetch_memory(0xfffc);
        let upper = self.fetch_memory(0xfffd);
    }

    fn load(&mut self, rom: Vec<u16>) {
        let prg_addr = 0x0010;
        let prg_page = rom[4];

        let chr_addr = prg_addr + prg_page * prg_rom_page_size;
        let chr_page = rom[5];

        let prg_bytes = rom
            .get(prg_addr as usize..(prg_addr + prg_page * prg_rom_page_size) as usize)
            .unwrap();
        let chr_bytes =
            rom.get(chr_addr as usize..(chr_addr + chr_page * chr_rom_page_size) as usize);

        for byte in prg_bytes {
            self.ram[(0x8000 + byte) as usize] = prg_bytes[*byte as usize];
            if prg_page == 1 {
                self.ram[(0x8000 + byte + 0x4000) as usize] = prg_bytes[*byte as usize]
            }
        }
    }

    fn read(addr: String) {}

    pub fn fetch_code(&self, index: u16) -> u16 {
        self.ram[(self.register.pc + index) as usize]
    }

    fn reset() {}

    fn exec(&mut self) {
        let pre_pc = self.register.pc;
        let opecode = self.fetch_code(0) as usize;
        let (instruction, addressing) = (INSTRUCTIONS[opecode][0], INSTRUCTIONS[opecode][1]);

        let addr = match addressing {
            "impl" => self.implied(),
            "A" => self.accumulator(),
            "#" => self.immediate(),
            "zpg" => self.zero_page(),
            "zpg,X" => self.zero_page_x(),
            "zpg,Y" => self.zero_page_y(),
            "abs" => self.absolute(),
            "abs,X" => self.absolute_x(),
            "abs,Y" => self.absolute_y(),
            "rel" => self.relative(),
            "X,ind" => self.indexed_indirect(),
            "Ind,Y" => self.indirect_indexed(),
            "Ind" => self.absolute_indirect(),
            _ => None,
        };

        match instruction {
            "ADC" => self.adc(addr.unwrap()),
            "SBC" => self.sbc(addr.unwrap()),
            "AND" => self.and(addr.unwrap()),
            "ORA" => self.ora(addr.unwrap()),
            _ => ()
            // "EOR" => self.EOR(addr),
            // "ASL" => self.ASL(addr),
            // "LSR" => self.LSR(addr),
            // "ROL" => self.ROL(addr),
            // "ROR" => self.ROR(addr),
            // "BCC" => self.BCC(addr),
            // "BCS" => self.BCS(addr),
            // "BEQ" => self.BEQ(addr),
            // "BNE" => self.BNE(addr),
            // "BVC" => self.BVC(addr),
            // "BVS" => self.BVS(addr),
            // "BPL" => self.BPL(addr),
            // "BMI" => self.BMI(addr),
            // "BIT" => self.BIT(addr),
            // "JMP" => self.JMP(addr),
            // "JSR" => self.JSR(addr),
            // "RTS" => self.RTS(addr),
            // "BRK" => self.BRK(addr),
            // "RTI" => self.RTI(addr),
            // "CMP" => self.CMP(addr),
            // "CPX" => self.CPX(addr),
            // "CPY" => self.CPY(addr),
            // "INC" => self.INC(addr),
            // "DEC" => self.DEC(addr),
            // "INX" => self.INX(addr),
            // "DEX" => self.DEX(addr),
            // "INY" => self.INY(addr),
            // "DEY" => self.DEY(addr),
            // "CLC" => self.CLC(addr),
            // "SEC" => self.SEC(addr),
            // "CLI" => self.CLI(addr),
            // "SEI" => self.SEI(addr),
            // "CLD" => self.CLD(addr),
            // "SED" => self.SED(addr),
            // "CLV" => self.CLV(addr),
            // "LDA" => self.LDA(addr),
            // "LDX" => self.LDX(addr),
            // "LDY" => self.LDY(addr),
            // "STA" => self.STA(addr),
            // "STX" => self.STX(addr),
            // "STY" => self.STY(addr),
            // "TAX" => self.TAX(addr),
            // "TAY" => self.TAY(addr),
            // "TXA" => self.TXA(addr),
            // "TYA" => self.TYA(addr),
            // "TSX" => self.TSX(addr),
            // "TXS" => self.TXS(addr),
            // "PHA" => self.PHA(addr),
            // "PLA" => self.PLA(addr),
            // "PHP" => self.PHP(addr),
            // "PLP" => self.PLP(addr),
            // "NOP" => self.NOP(addr),
        }
    }
}
