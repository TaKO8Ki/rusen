use crate::addressing::AddressingMode;
use crate::instruction::Instruction;

const PRG_ROM_PAGE_SIZE: u16 = 0x4000;
const CHR_ROM_PAGE_SIZE: u16 = 0x2000;

pub struct Cpu {
    pub register: Register,
    pub ram: [u8; 0x10000],
}

#[derive(PartialEq)]
pub struct Register {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub s: u16,
    // status register
    // 0: C, 1: Z, 2: I, 3: D, 4: B, 5: R, 6: V, 7: N
    pub p: u8,
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
            s: 0x01fd,
            p: 0x34,
            pc: 0,
        }
    }
}

impl std::fmt::Debug for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Register {{ a: {:x}, x: {:x}, y: {:x}, s: {:x}, p: {:x}, pc: {:x} }}",
            self.a, self.x, self.y, self.s, self.p, self.pc
        )?;
        Ok(())
    }
}

impl Cpu {
    pub fn initialize(&mut self) {
        let lower = self.fetch_memory8(0xfffc);
        let upper = self.fetch_memory8(0xfffd);
        self.register.pc = (upper as u16) << 8 | lower as u16;
    }

    pub fn load(&mut self, rom: Vec<u8>) {
        let prg_addr = 0x0010;
        let prg_page = rom[4];

        let chr_addr = prg_addr + prg_page as u16 * PRG_ROM_PAGE_SIZE;
        let chr_page = rom[5];

        let prg_bytes = rom
            .get(prg_addr as usize..(prg_addr + prg_page as u16 * PRG_ROM_PAGE_SIZE) as usize)
            .unwrap();
        let _chr_bytes =
            rom.get(chr_addr as usize..(chr_addr + chr_page as u16 * CHR_ROM_PAGE_SIZE) as usize);

        for (index, byte) in prg_bytes.iter().enumerate() {
            self.ram[0x8000 + index] = *byte;
            if prg_page == 1 {
                self.ram[(0x8000 + index + 0x4000) as usize] = *byte
            }
        }
    }

    pub fn fetch_code8(&self, index: u8) -> u8 {
        self.ram[(self.register.pc + index as u16) as usize]
    }

    pub fn instructions(&self, opcode: u8) -> (Instruction, AddressingMode) {
        println!("[opcode] {:x}", opcode);
        match opcode {
            0xa9 => (Instruction::LDA, AddressingMode::Immediate),
            0xa5 => (Instruction::LDA, AddressingMode::ZeroPage),
            0xb5 => (Instruction::LDA, AddressingMode::ZeroPageX),
            0xad => (Instruction::LDA, AddressingMode::Absolute),
            0xbd => (Instruction::LDA, AddressingMode::AbsoluteX),
            0xb9 => (Instruction::LDA, AddressingMode::AbsoluteY),
            0xa1 => (Instruction::LDA, AddressingMode::IndirectX),
            0xb1 => (Instruction::LDA, AddressingMode::IndirectY),

            0xa2 => (Instruction::LDX, AddressingMode::Immediate),
            0xa6 => (Instruction::LDX, AddressingMode::ZeroPage),
            0xb6 => (Instruction::LDX, AddressingMode::ZeroPageY),
            0xae => (Instruction::LDX, AddressingMode::Absolute),
            0xbe => (Instruction::LDX, AddressingMode::AbsoluteY),

            0xa0 => (Instruction::LDY, AddressingMode::Immediate),
            0xa4 => (Instruction::LDY, AddressingMode::ZeroPage),
            0xb4 => (Instruction::LDY, AddressingMode::ZeroPageX),
            0xac => (Instruction::LDY, AddressingMode::Absolute),
            0xbc => (Instruction::LDY, AddressingMode::AbsoluteX),

            0x85 => (Instruction::STA, AddressingMode::ZeroPage),
            0x95 => (Instruction::STA, AddressingMode::ZeroPageX),
            0x8d => (Instruction::STA, AddressingMode::Absolute),
            0x9d => (Instruction::STA, AddressingMode::AbsoluteX),
            0x99 => (Instruction::STA, AddressingMode::AbsoluteY),
            0x81 => (Instruction::STA, AddressingMode::IndirectX),
            0x91 => (Instruction::STA, AddressingMode::IndirectY),

            0x86 => (Instruction::STX, AddressingMode::ZeroPage),
            0x96 => (Instruction::STX, AddressingMode::ZeroPageY),
            0x8e => (Instruction::STX, AddressingMode::Absolute),

            0x84 => (Instruction::STY, AddressingMode::ZeroPage),
            0x94 => (Instruction::STY, AddressingMode::ZeroPageX),
            0x8c => (Instruction::STY, AddressingMode::Absolute),

            0xaa => (Instruction::TAX, AddressingMode::Implied),

            0xa8 => (Instruction::TAY, AddressingMode::Implied),

            0xba => (Instruction::TSX, AddressingMode::Implied),

            0x8a => (Instruction::TXA, AddressingMode::Implied),

            0x9a => (Instruction::TXS, AddressingMode::Implied),

            0x98 => (Instruction::TYA, AddressingMode::Implied),

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

            _ => panic!("Invalid opcode: {:08x}", opcode),
        }
    }

    pub fn reset(&mut self) {
        println!("[interrupt] RESET");
        self.flag_i(true);
    }

    pub fn step(&mut self) {
        let opcode = self.fetch_code8(0);
        let (instruction, addressing) = self.instructions(opcode);

        println!("[instruction] {:?}", instruction);
        println!("[addressing mode] {:?}", addressing);
        println!("[before] {:?}", self.register);

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
        };

        match instruction {
            Instruction::ADC => self.adc(addr as u8),
            Instruction::SBC => self.sbc(addr as u8),
            Instruction::AND => self.and(addr as u8),
            Instruction::ORA => self.ora(addr as u8),
            Instruction::EOR => self.eor(addr as u8),
            Instruction::ASL => self.asl(addr as u8),
            Instruction::LSR => self.lsr(addr as u8),
            Instruction::ROL => self.rol(addr as u8),
            Instruction::ROR => self.ror(addr as u8),
            Instruction::BCC => self.bcc(addr),
            Instruction::BCS => self.bcs(addr),
            Instruction::BEQ => self.beq(addr),
            Instruction::BNE => self.bne(addr),
            Instruction::BVC => self.bvc(addr),
            Instruction::BVS => self.bvs(addr),
            Instruction::BPL => self.bpl(addr),
            Instruction::BMI => self.bmi(addr),
            Instruction::BIT => self.bit(addr as u8),
            Instruction::JMP => self.jmp(addr),
            Instruction::JSR => self.jsr(addr as u8),
            Instruction::RTS => self.rts(),
            Instruction::BRK => self.brk(),
            // Instruction::RTI => self.rti(addr),
            // Instruction::CMP => self.cmp(addr),
            // Instruction::CPX => self.cpx(addr),
            // Instruction::CPY => self.cpy(addr),
            // Instruction::INC => self.inc(addr),
            // Instruction::DEC => self.dec(addr),
            Instruction::INX => self.inx(),
            // Instruction::DEX => self.dex(addr),
            // Instruction::INY => self.iny(addr),
            Instruction::DEY => self.dey(),
            // Instruction::SEC => self.sec(addr),
            // Instruction::CLI => self.cli(addr),
            // Instruction::CLC => self.clc(addr),
            Instruction::SEI => self.sei(),
            // Instruction::CLD => self.cld(addr),
            // Instruction::SED => self.sed(addr),
            // Instruction::CLV => self.clv(addr),
            Instruction::LDA => self.lda(addr),
            Instruction::LDX => self.ldx(addr),
            Instruction::LDY => self.ldy(addr),
            Instruction::STA => self.sta(addr),
            // Instruction::STX => self.stx(addr),
            // Instruction::STY => self.sty(addr),
            // Instruction::TAX => self.tax(addr),
            // Instruction::TAY => self.tay(addr),
            // Instruction::TXA => self.txa(addr),
            // Instruction::TYA => self.tya(addr),
            // Instruction::TSX => self.tsx(addr),
            Instruction::TXS => self.txs(),
            // Instruction::PHA => self.pha(addr),
            // Instruction::PLA => self.pla(addr),
            // Instruction::PHP => self.php(addr),
            // Instruction::PLP => self.plp(addr),
            // Instruction::NOP => self.nop(addr),
            _ => (),
        }
        println!("[after] {:?}", self.register);
    }
}
