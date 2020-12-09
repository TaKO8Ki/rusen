use crate::addressing::AddressingMode;
use crate::instruction::Instruction;
use crate::nes::Nes;

const PRG_ROM_PAGE_SIZE: u16 = 0x4000;
const CHR_ROM_PAGE_SIZE: u16 = 0x2000;

#[derive(PartialEq)]
pub struct Cpu {
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
            a: 0,
            x: 0,
            y: 0,
            s: 0x01fd,
            p: 0x34,
            pc: 0,
        }
    }
}

impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Cpu {{ a: {:x}, x: {:x}, y: {:x}, s: {:x}, p: {:x}, pc: {:x} }}",
            self.a, self.x, self.y, self.s, self.p, self.pc
        )?;
        Ok(())
    }
}

impl Nes {
    pub fn initialize(&mut self) {
        let lower = self.fetch_memory8(0xfffc);
        let upper = self.fetch_memory8(0xfffd);
        self.cpu.pc = (upper as u16) << 8 | lower as u16;
    }

    pub fn load(&mut self, rom: Vec<u8>) {
        let mirror_flag = rom[6];
        self.ppu.mirror = mirror_flag > 0;

        let prg_addr = 0x0010;
        let prg_page = rom[4];

        let chr_addr = prg_addr + prg_page as u16 * PRG_ROM_PAGE_SIZE;
        let chr_page = rom[5];

        let prg_bytes = rom
            .get(prg_addr as usize..(prg_addr + prg_page as u16 * PRG_ROM_PAGE_SIZE) as usize)
            .unwrap();
        let chr_bytes = rom
            .get(chr_addr as usize..(chr_addr + chr_page as u16 * CHR_ROM_PAGE_SIZE) as usize)
            .unwrap();

        for (index, byte) in prg_bytes.iter().enumerate() {
            self.ram[0x8000 + index] = *byte;
            if prg_page == 1 {
                self.ram[(0x8000 + index + 0x4000) as usize] = *byte
            }
        }

        for (index, byte) in chr_bytes.iter().enumerate() {
            self.ppu.ram[index] = *byte
        }
    }

    pub fn fetch_code8(&self, index: u8) -> u8 {
        self.ram[(self.cpu.pc + index as u16) as usize]
    }

    pub fn instructions(&self, opcode: u8) -> (Instruction, AddressingMode, u8) {
        println!("[opcode] {:x}", opcode);
        match opcode {
            0xa9 => (Instruction::LDA, AddressingMode::Immediate, 2),
            0xa5 => (Instruction::LDA, AddressingMode::ZeroPage, 3),
            0xb5 => (Instruction::LDA, AddressingMode::ZeroPageX, 4),
            0xad => (Instruction::LDA, AddressingMode::Absolute, 4),
            0xbd => (Instruction::LDA, AddressingMode::AbsoluteX, 4),
            0xb9 => (Instruction::LDA, AddressingMode::AbsoluteY, 4),
            0xa1 => (Instruction::LDA, AddressingMode::IndirectX, 6),
            0xb1 => (Instruction::LDA, AddressingMode::IndirectY, 5),

            0xa2 => (Instruction::LDX, AddressingMode::Immediate, 2),
            0xa6 => (Instruction::LDX, AddressingMode::ZeroPage, 3),
            0xb6 => (Instruction::LDX, AddressingMode::ZeroPageY, 4),
            0xae => (Instruction::LDX, AddressingMode::Absolute, 4),
            0xbe => (Instruction::LDX, AddressingMode::AbsoluteY, 4),

            0xa0 => (Instruction::LDY, AddressingMode::Immediate, 2),
            0xa4 => (Instruction::LDY, AddressingMode::ZeroPage, 3),
            0xb4 => (Instruction::LDY, AddressingMode::ZeroPageX, 4),
            0xac => (Instruction::LDY, AddressingMode::Absolute, 4),
            0xbc => (Instruction::LDY, AddressingMode::AbsoluteX, 4),

            0x85 => (Instruction::STA, AddressingMode::ZeroPage, 3),
            0x95 => (Instruction::STA, AddressingMode::ZeroPageX, 4),
            0x8d => (Instruction::STA, AddressingMode::Absolute, 4),
            0x9d => (Instruction::STA, AddressingMode::AbsoluteX, 5),
            0x99 => (Instruction::STA, AddressingMode::AbsoluteY, 5),
            0x81 => (Instruction::STA, AddressingMode::IndirectX, 6),
            0x91 => (Instruction::STA, AddressingMode::IndirectY, 6),

            0x86 => (Instruction::STX, AddressingMode::ZeroPage, 3),
            0x96 => (Instruction::STX, AddressingMode::ZeroPageY, 4),
            0x8e => (Instruction::STX, AddressingMode::Absolute, 4),

            0x84 => (Instruction::STY, AddressingMode::ZeroPage, 3),
            0x94 => (Instruction::STY, AddressingMode::ZeroPageX, 4),
            0x8c => (Instruction::STY, AddressingMode::Absolute, 4),

            0xaa => (Instruction::TAX, AddressingMode::Implied, 2),

            0xa8 => (Instruction::TAY, AddressingMode::Implied, 2),

            0xba => (Instruction::TSX, AddressingMode::Implied, 2),

            0x8a => (Instruction::TXA, AddressingMode::Implied, 2),

            0x9a => (Instruction::TXS, AddressingMode::Implied, 2),

            0x98 => (Instruction::TYA, AddressingMode::Implied, 2),

            0x69 => (Instruction::ADC, AddressingMode::Immediate, 2),
            0x65 => (Instruction::ADC, AddressingMode::ZeroPage, 3),
            0x75 => (Instruction::ADC, AddressingMode::ZeroPageX, 4),
            0x6d => (Instruction::ADC, AddressingMode::Absolute, 4),
            0x7d => (Instruction::ADC, AddressingMode::AbsoluteX, 4),
            0x79 => (Instruction::ADC, AddressingMode::AbsoluteY, 4),
            0x61 => (Instruction::ADC, AddressingMode::IndirectX, 6),
            0x71 => (Instruction::ADC, AddressingMode::IndirectY, 5),

            0x29 => (Instruction::AND, AddressingMode::Immediate, 2),
            0x25 => (Instruction::AND, AddressingMode::ZeroPage, 3),
            0x35 => (Instruction::AND, AddressingMode::ZeroPageX, 3),
            0x2d => (Instruction::AND, AddressingMode::Absolute, 3),
            0x3d => (Instruction::AND, AddressingMode::AbsoluteX, 3),
            0x39 => (Instruction::AND, AddressingMode::AbsoluteY, 3),
            0x21 => (Instruction::AND, AddressingMode::IndirectX, 6),
            0x31 => (Instruction::AND, AddressingMode::IndirectY, 5),

            0x0a => (Instruction::ASL, AddressingMode::Accumulator, 2),
            0x06 => (Instruction::ASL, AddressingMode::ZeroPage, 5),
            0x16 => (Instruction::ASL, AddressingMode::ZeroPageX, 6),
            0x0e => (Instruction::ASL, AddressingMode::Absolute, 6),
            0x1e => (Instruction::ASL, AddressingMode::AbsoluteX, 7),

            0x24 => (Instruction::BIT, AddressingMode::ZeroPageX, 3),
            0x2c => (Instruction::BIT, AddressingMode::Absolute, 4),

            0xc9 => (Instruction::CMP, AddressingMode::Immediate, 2),
            0xc5 => (Instruction::CMP, AddressingMode::ZeroPage, 3),
            0xd5 => (Instruction::CMP, AddressingMode::ZeroPageX, 4),
            0xcd => (Instruction::CMP, AddressingMode::Absolute, 4),
            0xdd => (Instruction::CMP, AddressingMode::AbsoluteX, 4),
            0xd9 => (Instruction::CMP, AddressingMode::AbsoluteY, 4),
            0xc1 => (Instruction::CMP, AddressingMode::IndirectX, 6),
            0xd1 => (Instruction::CMP, AddressingMode::IndirectY, 5),

            0xe0 => (Instruction::CPX, AddressingMode::Immediate, 2),
            0xe4 => (Instruction::CPX, AddressingMode::ZeroPage, 3),
            0xec => (Instruction::CPX, AddressingMode::Absolute, 4),

            0xc0 => (Instruction::CPY, AddressingMode::Immediate, 2),
            0xc4 => (Instruction::CPY, AddressingMode::ZeroPage, 3),
            0xcc => (Instruction::CPY, AddressingMode::Absolute, 4),

            0xc6 => (Instruction::DEC, AddressingMode::ZeroPage, 5),
            0xd6 => (Instruction::DEC, AddressingMode::ZeroPageX, 6),
            0xce => (Instruction::DEC, AddressingMode::Absolute, 6),
            0xde => (Instruction::DEC, AddressingMode::AbsoluteX, 7),

            0xca => (Instruction::DEX, AddressingMode::Implied, 2),

            0x88 => (Instruction::DEY, AddressingMode::Implied, 2),

            0x49 => (Instruction::EOR, AddressingMode::Immediate, 2),
            0x45 => (Instruction::EOR, AddressingMode::ZeroPage, 3),
            0x55 => (Instruction::EOR, AddressingMode::ZeroPageX, 4),
            0x4d => (Instruction::EOR, AddressingMode::Absolute, 4),
            0x5d => (Instruction::EOR, AddressingMode::AbsoluteX, 4),
            0x59 => (Instruction::EOR, AddressingMode::AbsoluteY, 4),
            0x41 => (Instruction::EOR, AddressingMode::IndirectX, 6),
            0x51 => (Instruction::EOR, AddressingMode::IndirectY, 5),

            0xe6 => (Instruction::INC, AddressingMode::ZeroPage, 5),
            0xf6 => (Instruction::INC, AddressingMode::ZeroPageX, 6),
            0xee => (Instruction::INC, AddressingMode::Absolute, 6),
            0xfe => (Instruction::INC, AddressingMode::AbsoluteX, 7),

            0xe8 => (Instruction::INX, AddressingMode::Implied, 2),

            0xc8 => (Instruction::INY, AddressingMode::Implied, 2),

            0x4a => (Instruction::LSR, AddressingMode::Accumulator, 2),
            0x46 => (Instruction::LSR, AddressingMode::ZeroPage, 5),
            0x56 => (Instruction::LSR, AddressingMode::ZeroPageX, 6),
            0x4e => (Instruction::LSR, AddressingMode::Absolute, 6),
            0x5e => (Instruction::LSR, AddressingMode::AbsoluteX, 7),

            0x09 => (Instruction::ORA, AddressingMode::Immediate, 2),
            0x05 => (Instruction::ORA, AddressingMode::ZeroPage, 3),
            0x15 => (Instruction::ORA, AddressingMode::ZeroPageX, 4),
            0x0d => (Instruction::ORA, AddressingMode::Absolute, 4),
            0x1d => (Instruction::ORA, AddressingMode::AbsoluteX, 4),
            0x19 => (Instruction::ORA, AddressingMode::AbsoluteY, 4),
            0x01 => (Instruction::ORA, AddressingMode::IndirectX, 6),
            0x11 => (Instruction::ORA, AddressingMode::IndirectY, 5),

            0x2a => (Instruction::ROL, AddressingMode::Accumulator, 2),
            0x26 => (Instruction::ROL, AddressingMode::ZeroPage, 5),
            0x36 => (Instruction::ROL, AddressingMode::ZeroPageX, 6),
            0x2e => (Instruction::ROL, AddressingMode::Absolute, 6),
            0x3e => (Instruction::ROL, AddressingMode::AbsoluteX, 7),

            0x6a => (Instruction::ROR, AddressingMode::Accumulator, 2),
            0x66 => (Instruction::ROR, AddressingMode::ZeroPage, 5),
            0x76 => (Instruction::ROR, AddressingMode::ZeroPageX, 6),
            0x6e => (Instruction::ROR, AddressingMode::Absolute, 6),
            0x7e => (Instruction::ROR, AddressingMode::AbsoluteX, 7),

            0xe9 => (Instruction::SBC, AddressingMode::Immediate, 2),
            0xe5 => (Instruction::SBC, AddressingMode::ZeroPage, 3),
            0xf5 => (Instruction::SBC, AddressingMode::ZeroPageX, 4),
            0xed => (Instruction::SBC, AddressingMode::Absolute, 4),
            0xfd => (Instruction::SBC, AddressingMode::AbsoluteX, 4),
            0xf9 => (Instruction::SBC, AddressingMode::AbsoluteY, 4),
            0xe1 => (Instruction::SBC, AddressingMode::IndirectX, 6),
            0xf1 => (Instruction::SBC, AddressingMode::IndirectY, 5),

            0x48 => (Instruction::PHA, AddressingMode::Implied, 3),
            0x08 => (Instruction::PHP, AddressingMode::Implied, 3),
            0x68 => (Instruction::PLA, AddressingMode::Implied, 4),
            0x28 => (Instruction::PLP, AddressingMode::Implied, 4),

            0x4c => (Instruction::JMP, AddressingMode::Absolute, 3),
            0x6c => (Instruction::JMP, AddressingMode::Indirect, 5),

            0x20 => (Instruction::JSR, AddressingMode::Absolute, 6),
            0x60 => (Instruction::RTS, AddressingMode::Implied, 6),
            0x40 => (Instruction::RTI, AddressingMode::Implied, 6),

            0x90 => (Instruction::BCC, AddressingMode::Relative, 2),
            0xb0 => (Instruction::BCS, AddressingMode::Relative, 2),
            0xf0 => (Instruction::BEQ, AddressingMode::Relative, 2),
            0x30 => (Instruction::BMI, AddressingMode::Relative, 2),
            0xd0 => (Instruction::BNE, AddressingMode::Relative, 2),
            0x10 => (Instruction::BPL, AddressingMode::Relative, 2),
            0x50 => (Instruction::BVC, AddressingMode::Relative, 2),
            0x70 => (Instruction::BVS, AddressingMode::Relative, 2),

            0x18 => (Instruction::CLC, AddressingMode::Implied, 2),
            0xd8 => (Instruction::CLD, AddressingMode::Implied, 2),
            0x58 => (Instruction::CLI, AddressingMode::Implied, 2),
            0xb8 => (Instruction::CLV, AddressingMode::Implied, 2),
            0x38 => (Instruction::SEC, AddressingMode::Implied, 2),
            0xf8 => (Instruction::SED, AddressingMode::Implied, 2),
            0x78 => (Instruction::SEI, AddressingMode::Implied, 2),

            0x00 => (Instruction::BRK, AddressingMode::Implied, 7),
            0xea => (Instruction::NOP, AddressingMode::Implied, 2),

            _ => panic!("Invalid opcode: {:08x}", opcode),
        }
    }

    pub fn reset(&mut self) {
        println!("[interrupt] RESET");
        self.flag_i(true);
    }

    pub fn step(&mut self) -> u8 {
        let opcode = self.fetch_code8(0);
        let (instruction, addressing, cycle) = self.instructions(opcode);

        println!("[instruction] {:?}", instruction);
        println!("[addressing mode] {:?}", addressing);
        println!("[before] {:?}", self.cpu);

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
            Instruction::ADC => self.adc(addr),
            Instruction::SBC => self.sbc(addr),
            Instruction::AND => self.and(addr),
            Instruction::ORA => self.ora(addr),
            Instruction::EOR => self.eor(addr),
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
            Instruction::BIT => self.bit(addr),
            Instruction::JMP => self.jmp(addr),
            Instruction::JSR => self.jsr(addr),
            Instruction::RTS => self.rts(),
            Instruction::BRK => self.brk(),
            Instruction::RTI => self.rti(),
            Instruction::CMP => self.cmp(addr),
            Instruction::CPX => self.cpx(addr),
            Instruction::CPY => self.cpy(addr),
            Instruction::INC => self.inc(addr),
            Instruction::DEC => self.dec(addr),
            Instruction::INX => self.inx(),
            Instruction::DEX => self.dex(),
            Instruction::INY => self.iny(),
            Instruction::DEY => self.dey(),
            Instruction::SEC => self.sec(),
            Instruction::CLI => self.cli(),
            Instruction::CLC => self.clc(addr),
            Instruction::SEI => self.sei(),
            Instruction::CLD => self.cld(),
            Instruction::SED => self.sed(),
            Instruction::CLV => self.clv(),
            Instruction::LDA => self.lda(addr),
            Instruction::LDX => self.ldx(addr),
            Instruction::LDY => self.ldy(addr),
            Instruction::STA => self.sta(addr),
            Instruction::STX => self.stx(addr),
            Instruction::STY => self.sty(addr),
            Instruction::TAX => self.tax(),
            Instruction::TAY => self.tay(),
            Instruction::TXA => self.txa(),
            Instruction::TYA => self.tya(),
            Instruction::TSX => self.tsx(),
            Instruction::TXS => self.txs(),
            Instruction::PHA => self.pha(),
            Instruction::PLA => self.pla(),
            Instruction::PHP => self.php(),
            Instruction::PLP => self.plp(),
            Instruction::NOP => self.nop(),
        }
        println!("[after] {:?}\n", self.cpu);
        cycle
    }
}
