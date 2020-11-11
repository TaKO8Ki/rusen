use crate::addressing::AddressingMode;
use crate::instruction::Instruction;

const prg_rom_page_size: u16 = 16 * 1024;
const chr_rom_page_size: u16 = 8 * 1024;

pub struct Cpu {
    pub register: Register,
    pub ram: [u16; 0x10000],
}

#[derive(Debug)]
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
            ram: [0; 0x10000],
        }
    }
}

impl Default for Register {
    fn default() -> Self {
        Register {
            a: 0,
            x: 0,
            y: 0,
            s: 0xfd,
            p: 0x34,
            sp: 0,
            pc: 0,
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

    pub fn fetch_code8(&self, index: u16) -> u16 {
        self.ram[(self.register.pc + index) as usize]
    }

    fn reset() {}

    pub fn instructions(&self, opecode: u16) -> (Instruction, AddressingMode) {
        match opecode {
            0x69 => (Instruction::ADC, AddressingMode::Immediate),
            0x65 => (Instruction::ADC, AddressingMode::ZeroPage),
            0x75 => (Instruction::ADC, AddressingMode::ZeroPageX),
            0x6d => (Instruction::ADC, AddressingMode::Absolute),
            0x7d => (Instruction::ADC, AddressingMode::AbsoluteX),
            0x79 => (Instruction::ADC, AddressingMode::AbsoluteY),
            0x61 => (Instruction::ADC, AddressingMode::IndirectX),
            0x71 => (Instruction::ADC, AddressingMode::IndirectY),

            0x29 => (Instruction::AND, AddressingMode::Immediate),
            0x25 => (Instruction::AND, AddressingMode::ZeroPage),
            0x35 => (Instruction::AND, AddressingMode::ZeroPageX),
            0x2d => (Instruction::AND, AddressingMode::Absolute),
            0x3d => (Instruction::AND, AddressingMode::AbsoluteX),
            0x39 => (Instruction::AND, AddressingMode::AbsoluteY),
            0x21 => (Instruction::AND, AddressingMode::IndirectX),
            0x31 => (Instruction::AND, AddressingMode::IndirectY),

            0x0a => (Instruction::ASL, AddressingMode::Accumulator),
            0x06 => (Instruction::ASL, AddressingMode::ZeroPage),
            0x16 => (Instruction::ASL, AddressingMode::ZeroPageX),
            0x0e => (Instruction::ASL, AddressingMode::Absolute),
            0x1e => (Instruction::ASL, AddressingMode::AbsoluteX),

            0x24 => (Instruction::BIT, AddressingMode::ZeroPageX),
            0x2c => (Instruction::BIT, AddressingMode::Absolute),

            0xc9 => (Instruction::CMP, AddressingMode::Immediate),
            0xc5 => (Instruction::CMP, AddressingMode::ZeroPage),
            0xd5 => (Instruction::CMP, AddressingMode::ZeroPageX),
            0xcd => (Instruction::CMP, AddressingMode::Absolute),
            0xdd => (Instruction::CMP, AddressingMode::AbsoluteX),
            0xd9 => (Instruction::CMP, AddressingMode::AbsoluteY),
            0xc1 => (Instruction::CMP, AddressingMode::IndirectX),
            0xd1 => (Instruction::CMP, AddressingMode::IndirectY),

            0xe0 => (Instruction::CPX, AddressingMode::Immediate),
            0xe4 => (Instruction::CPX, AddressingMode::ZeroPage),
            0xec => (Instruction::CPX, AddressingMode::Absolute),

            0xc0 => (Instruction::CPY, AddressingMode::Immediate),
            0xc4 => (Instruction::CPY, AddressingMode::ZeroPage),
            0xcc => (Instruction::CPY, AddressingMode::Absolute),

            0xc6 => (Instruction::DEC, AddressingMode::ZeroPage),
            0xd6 => (Instruction::DEC, AddressingMode::ZeroPageX),
            0xce => (Instruction::DEC, AddressingMode::Absolute),
            0xde => (Instruction::DEC, AddressingMode::AbsoluteX),

            0xca => (Instruction::DEX, AddressingMode::Implied),

            0x88 => (Instruction::DEY, AddressingMode::Implied),

            0x49 => (Instruction::EOR, AddressingMode::Immediate),
            0x45 => (Instruction::EOR, AddressingMode::ZeroPage),
            0x55 => (Instruction::EOR, AddressingMode::ZeroPageX),
            0x4d => (Instruction::EOR, AddressingMode::Absolute),
            0x5d => (Instruction::EOR, AddressingMode::AbsoluteX),
            0x59 => (Instruction::EOR, AddressingMode::AbsoluteY),
            0x41 => (Instruction::EOR, AddressingMode::IndirectX),
            0x51 => (Instruction::EOR, AddressingMode::IndirectY),

            0xe6 => (Instruction::INC, AddressingMode::ZeroPage),
            0xf6 => (Instruction::INC, AddressingMode::ZeroPageX),
            0xee => (Instruction::INC, AddressingMode::Absolute),
            0xfe => (Instruction::INC, AddressingMode::AbsoluteX),

            0xe8 => (Instruction::INX, AddressingMode::Implied),

            0xc8 => (Instruction::INY, AddressingMode::Implied),

            0x4a => (Instruction::LSR, AddressingMode::Accumulator),
            0x46 => (Instruction::LSR, AddressingMode::ZeroPage),
            0x56 => (Instruction::LSR, AddressingMode::ZeroPageX),
            0x4e => (Instruction::LSR, AddressingMode::Absolute),
            0x5e => (Instruction::LSR, AddressingMode::AbsoluteX),

            0x09 => (Instruction::ORA, AddressingMode::Immediate),
            0x05 => (Instruction::ORA, AddressingMode::ZeroPage),
            0x15 => (Instruction::ORA, AddressingMode::ZeroPageX),
            0x0d => (Instruction::ORA, AddressingMode::Absolute),
            0x1d => (Instruction::ORA, AddressingMode::AbsoluteX),
            0x19 => (Instruction::ORA, AddressingMode::AbsoluteY),
            0x01 => (Instruction::ORA, AddressingMode::IndirectX),
            0x11 => (Instruction::ORA, AddressingMode::IndirectY),

            0x2a => (Instruction::ROL, AddressingMode::Accumulator),
            0x26 => (Instruction::ROL, AddressingMode::ZeroPage),
            0x36 => (Instruction::ROL, AddressingMode::ZeroPageX),
            0x2e => (Instruction::ROL, AddressingMode::Absolute),
            0x3e => (Instruction::ROL, AddressingMode::AbsoluteX),

            0x6a => (Instruction::ROR, AddressingMode::Accumulator),
            0x66 => (Instruction::ROR, AddressingMode::ZeroPage),
            0x76 => (Instruction::ROR, AddressingMode::ZeroPageX),
            0x6e => (Instruction::ROR, AddressingMode::Absolute),
            0x7e => (Instruction::ROR, AddressingMode::AbsoluteX),

            0xe9 => (Instruction::SBC, AddressingMode::Immediate),
            0xe5 => (Instruction::SBC, AddressingMode::ZeroPage),
            0xf5 => (Instruction::SBC, AddressingMode::ZeroPageX),
            0xed => (Instruction::SBC, AddressingMode::Absolute),
            0xfd => (Instruction::SBC, AddressingMode::AbsoluteX),
            0xf9 => (Instruction::SBC, AddressingMode::AbsoluteY),
            0xe1 => (Instruction::SBC, AddressingMode::IndirectX),
            0xf1 => (Instruction::SBC, AddressingMode::IndirectY),

            0x48 => (Instruction::PHA, AddressingMode::Implied),
            0x08 => (Instruction::PHP, AddressingMode::Implied),
            0x68 => (Instruction::PLA, AddressingMode::Implied),
            0x28 => (Instruction::PLP, AddressingMode::Implied),

            0x4c => (Instruction::JMP, AddressingMode::Absolute),
            0x6c => (Instruction::JMP, AddressingMode::Indirect),

            0x20 => (Instruction::JSR, AddressingMode::Absolute),
            0x60 => (Instruction::RTS, AddressingMode::Implied),
            0x40 => (Instruction::RTI, AddressingMode::Implied),

            0x90 => (Instruction::BCC, AddressingMode::Relative),
            0xb0 => (Instruction::BCS, AddressingMode::Relative),
            0xf0 => (Instruction::BEQ, AddressingMode::Relative),
            0x30 => (Instruction::BMI, AddressingMode::Relative),
            0xd0 => (Instruction::BNE, AddressingMode::Relative),
            0x10 => (Instruction::BPL, AddressingMode::Relative),
            0x50 => (Instruction::BVC, AddressingMode::Relative),
            0x70 => (Instruction::BVS, AddressingMode::Relative),

            0x18 => (Instruction::CLC, AddressingMode::Implied),
            0xd8 => (Instruction::CLD, AddressingMode::Implied),
            0x58 => (Instruction::CLI, AddressingMode::Implied),
            0xb8 => (Instruction::CLV, AddressingMode::Implied),
            0x38 => (Instruction::SEC, AddressingMode::Implied),
            0xf8 => (Instruction::SED, AddressingMode::Implied),
            0x78 => (Instruction::SEI, AddressingMode::Implied),

            0x00 => (Instruction::BRK, AddressingMode::Implied),
            0xea => (Instruction::NOP, AddressingMode::Implied),

            _ => panic!("Invalid opecode: {:08x}", opecode),
        }
    }

    pub fn exec(&mut self) {
        let pre_pc = self.register.pc;
        let opecode = self.fetch_code8(0);
        let (instruction, addressing) = self.instructions(opecode);

        let addr = match addressing {
            AddressingMode::Implied => self.implied(),
            AddressingMode::Accumulator => self.accumulator(),
            AddressingMode::Immediate => self.immediate(),
            AddressingMode::ZeroPage => self.zero_page(),
            AddressingMode::ZeroPageX => self.zero_page_x(),
            AddressingMode::ZeroPageY => self.zero_page_y(),
            AddressingMode::Absolute => self.absolute(),
            AddressingMode::AbsoluteX => self.absolute_x(),
            AddressingMode::AbsoluteY => self.absolute_y(),
            AddressingMode::Relative => self.relative(),
            AddressingMode::IndirectX => self.indexed_indirect(),
            AddressingMode::IndirectY => self.indirect_indexed(),
            AddressingMode::Indirect => self.absolute_indirect(),
            _ => None,
        };

        match instruction {
            Instruction::ADC => self.adc(addr.unwrap()),
            Instruction::SBC => self.sbc(addr.unwrap()),
            Instruction::AND => self.and(addr.unwrap()),
            Instruction::ORA => self.ora(addr.unwrap()),
            Instruction::EOR => (),
            Instruction::ASL => (),
            Instruction::LSR => (),
            Instruction::ROL => (),
            Instruction::ROR => (),
            Instruction::BCC => (),
            Instruction::BCS => (),
            Instruction::BEQ => (),
            Instruction::BNE => (),
            Instruction::BVC => (),
            Instruction::BVS => (),
            Instruction::BPL => (),
            Instruction::BMI => (),
            Instruction::BIT => (),
            Instruction::JMP => (),
            Instruction::JSR => (),
            Instruction::RTS => (),
            Instruction::BRK => self.brk(),
            Instruction::RTI => (),
            Instruction::CMP => (),
            Instruction::CPX => (),
            Instruction::CPY => (),
            Instruction::INC => (),
            Instruction::DEC => (),
            Instruction::INX => (),
            Instruction::DEX => (),
            Instruction::INY => (),
            Instruction::DEY => (),
            Instruction::SEC => (),
            Instruction::CLI => (),
            Instruction::CLC => (),
            Instruction::SEI => (),
            Instruction::CLD => (),
            Instruction::SED => (),
            Instruction::CLV => (),
            Instruction::LDA => (),
            Instruction::LDX => (),
            Instruction::LDY => (),
            Instruction::STA => (),
            Instruction::STX => (),
            Instruction::STY => (),
            Instruction::TAX => (),
            Instruction::TAY => (),
            Instruction::TXA => (),
            Instruction::TYA => (),
            Instruction::TSX => (),
            Instruction::TXS => (),
            Instruction::PHA => (),
            Instruction::PLA => (),
            Instruction::PHP => (),
            Instruction::PLP => (),
            Instruction::NOP => (),
        }
        println!("{:?}", self.register);
    }
}
