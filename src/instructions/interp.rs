use super::addressing::*;
use super::*;
use crate::memory::Memory;

pub(crate) fn interp(memory: &Memory, pc: u16) -> Box<dyn Instruction> {
  let op = memory.absolute(pc);

  match op {
    0x69 => Box::new(ADC(Immediate(memory.absolute(pc + 1) as i8))),
    0x65 => Box::new(ADC(ZeroPage(memory.absolute(pc + 1)))),
    0x75 => Box::new(ADC(ZeroPageX(memory.absolute(pc + 1)))),
    0x6D => Box::new(ADC(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0x7D => Box::new(ADC(AbsoluteX(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0x79 => Box::new(ADC(AbsoluteY(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0x61 => Box::new(ADC(IndexedIndirect(memory.absolute(pc + 1)))),
    0x71 => Box::new(ADC(IndirectIndexed(memory.absolute(pc + 1)))),

    0x29 => Box::new(AND(Immediate(memory.absolute(pc + 1) as i8))),
    0x25 => Box::new(AND(ZeroPage(memory.absolute(pc + 1)))),
    0x35 => Box::new(AND(ZeroPageX(memory.absolute(pc + 1)))),
    0x2D => Box::new(AND(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0x3D => Box::new(AND(AbsoluteX(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0x39 => Box::new(AND(AbsoluteY(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0x21 => Box::new(AND(IndexedIndirect(memory.absolute(pc + 1)))),
    0x31 => Box::new(AND(IndirectIndexed(memory.absolute(pc + 1)))),

    0x0A => Box::new(ASL(Accumulator)),
    0x06 => Box::new(ASL(ZeroPage(memory.absolute(pc + 1)))),
    0x16 => Box::new(ASL(ZeroPageX(memory.absolute(pc + 1)))),
    0x0E => Box::new(ASL(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0x1E => Box::new(ASL(AbsoluteX(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),

    0x24 => Box::new(BIT(ZeroPage(memory.absolute(pc + 1)))),
    0x2C => Box::new(BIT(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),

    0x10 => Box::new(BPL(memory.absolute(pc + 1) as i8)),
    0x30 => Box::new(BMI(memory.absolute(pc + 1) as i8)),
    0x50 => Box::new(BVC(memory.absolute(pc + 1) as i8)),
    0x70 => Box::new(BVS(memory.absolute(pc + 1) as i8)),
    0x90 => Box::new(BCC(memory.absolute(pc + 1) as i8)),
    0xB0 => Box::new(BCS(memory.absolute(pc + 1) as i8)),
    0xD0 => Box::new(BNE(memory.absolute(pc + 1) as i8)),
    0xF0 => Box::new(BEQ(memory.absolute(pc + 1) as i8)),

    0x00 => Box::new(BRK),

    0xC9 => Box::new(CMP(Immediate(memory.absolute(pc + 1) as i8))),
    0xC5 => Box::new(CMP(ZeroPage(memory.absolute(pc + 1)))),
    0xD5 => Box::new(CMP(ZeroPageX(memory.absolute(pc + 1)))),
    0xCD => Box::new(CMP(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0xDD => Box::new(CMP(AbsoluteX(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0xD9 => Box::new(CMP(AbsoluteY(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0xC1 => Box::new(CMP(IndexedIndirect(memory.absolute(pc + 1)))),
    0xD1 => Box::new(CMP(IndirectIndexed(memory.absolute(pc + 1)))),

    0xE0 => Box::new(CPX(Immediate(memory.absolute(pc + 1) as i8))),
    0xE4 => Box::new(CPX(ZeroPage(memory.absolute(pc + 1)))),
    0xEC => Box::new(CPX(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),

    0xC0 => Box::new(CPY(Immediate(memory.absolute(pc + 1) as i8))),
    0xC4 => Box::new(CPY(ZeroPage(memory.absolute(pc + 1)))),
    0xCC => Box::new(CPY(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),

    0xC6 => Box::new(DEC(ZeroPage(memory.absolute(pc + 1)))),
    0xD6 => Box::new(DEC(ZeroPageX(memory.absolute(pc + 1)))),
    0xCE => Box::new(DEC(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0xDE => Box::new(DEC(AbsoluteX(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),

    0x49 => Box::new(EOR(Immediate(memory.absolute(pc + 1) as i8))),
    0x45 => Box::new(EOR(ZeroPage(memory.absolute(pc + 1)))),
    0x55 => Box::new(EOR(ZeroPageX(memory.absolute(pc + 1)))),
    0x4D => Box::new(EOR(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0x5D => Box::new(EOR(AbsoluteX(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0x59 => Box::new(EOR(AbsoluteY(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0x41 => Box::new(EOR(IndexedIndirect(memory.absolute(pc + 1)))),
    0x51 => Box::new(EOR(IndirectIndexed(memory.absolute(pc + 1)))),

    0x18 => Box::new(CLC),
    0x38 => Box::new(SEC),
    0x58 => Box::new(CLI),
    0x78 => Box::new(SEI),
    0xB8 => Box::new(CLV),
    0xD8 => Box::new(CLD),
    0xF8 => Box::new(SED),

    0xE6 => Box::new(INC(ZeroPage(memory.absolute(pc + 1)))),
    0xF6 => Box::new(INC(ZeroPageX(memory.absolute(pc + 1)))),
    0xEE => Box::new(INC(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0xFE => Box::new(INC(AbsoluteX(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),

    0x4C => Box::new(JMP(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0x6C => Box::new(JMP(Indirect(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),

    0x20 => Box::new(JSR(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ]))),

    0xA9 => Box::new(LDA(Immediate(memory.absolute(pc + 1) as i8))),
    0xA5 => Box::new(LDA(ZeroPage(memory.absolute(pc + 1)))),
    0xB5 => Box::new(LDA(ZeroPageX(memory.absolute(pc + 1)))),
    0xAD => Box::new(LDA(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0xBD => Box::new(LDA(AbsoluteX(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0xB9 => Box::new(LDA(AbsoluteY(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0xA1 => Box::new(LDA(IndexedIndirect(memory.absolute(pc + 1)))),
    0xB1 => Box::new(LDA(IndirectIndexed(memory.absolute(pc + 1)))),

    0xA2 => Box::new(LDX(Immediate(memory.absolute(pc + 1) as i8))),
    0xA6 => Box::new(LDX(ZeroPage(memory.absolute(pc + 1)))),
    0xB6 => Box::new(LDX(ZeroPageY(memory.absolute(pc + 1)))),
    0xAE => Box::new(LDX(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0xBE => Box::new(LDX(AbsoluteY(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),

    0xA0 => Box::new(LDY(Immediate(memory.absolute(pc + 1) as i8))),
    0xA4 => Box::new(LDY(ZeroPage(memory.absolute(pc + 1)))),
    0xB4 => Box::new(LDY(ZeroPageX(memory.absolute(pc + 1)))),
    0xAC => Box::new(LDY(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0xBC => Box::new(LDY(AbsoluteX(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),

    0x4A => Box::new(LSR(Accumulator)),
    0x46 => Box::new(LSR(ZeroPage(memory.absolute(pc + 1)))),
    0x56 => Box::new(LSR(ZeroPageX(memory.absolute(pc + 1)))),
    0x4E => Box::new(LSR(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0x5E => Box::new(LSR(AbsoluteX(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),

    0xEA => Box::new(NOP),

    0x09 => Box::new(ORA(Immediate(memory.absolute(pc + 1) as i8))),
    0x05 => Box::new(ORA(ZeroPage(memory.absolute(pc + 1)))),
    0x15 => Box::new(ORA(ZeroPageX(memory.absolute(pc + 1)))),
    0x0D => Box::new(ORA(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0x1D => Box::new(ORA(AbsoluteX(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0x19 => Box::new(ORA(AbsoluteY(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0x01 => Box::new(ORA(IndexedIndirect(memory.absolute(pc + 1)))),
    0x11 => Box::new(ORA(IndirectIndexed(memory.absolute(pc + 1)))),

    0xAA => Box::new(TAX),
    0x8A => Box::new(TXA),
    0xCA => Box::new(DEX),
    0xE8 => Box::new(INX),
    0xA8 => Box::new(TAY),
    0x98 => Box::new(TYA),
    0x88 => Box::new(DEY),
    0xC8 => Box::new(INY),

    0x2A => Box::new(ROL(Accumulator)),
    0x26 => Box::new(ROL(ZeroPage(memory.absolute(pc + 1)))),
    0x36 => Box::new(ROL(ZeroPageX(memory.absolute(pc + 1)))),
    0x2E => Box::new(ROL(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0x3E => Box::new(ROL(AbsoluteX(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),

    0x6A => Box::new(ROR(Accumulator)),
    0x66 => Box::new(ROR(ZeroPage(memory.absolute(pc + 1)))),
    0x76 => Box::new(ROR(ZeroPageX(memory.absolute(pc + 1)))),
    0x6E => Box::new(ROR(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0x7E => Box::new(ROR(AbsoluteX(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),

    0x40 => Box::new(RTI),

    0x60 => Box::new(RTS),

    0xE9 => Box::new(SBC(Immediate(memory.absolute(pc + 1) as i8))),
    0xE5 => Box::new(SBC(ZeroPage(memory.absolute(pc + 1)))),
    0xF5 => Box::new(SBC(ZeroPageX(memory.absolute(pc + 1)))),
    0xED => Box::new(SBC(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0xFD => Box::new(SBC(AbsoluteX(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0xF9 => Box::new(SBC(AbsoluteY(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0xE1 => Box::new(SBC(IndexedIndirect(memory.absolute(pc + 1)))),
    0xF1 => Box::new(SBC(IndirectIndexed(memory.absolute(pc + 1)))),

    0x85 => Box::new(STA(ZeroPage(memory.absolute(pc + 1)))),
    0x95 => Box::new(STA(ZeroPageX(memory.absolute(pc + 1)))),
    0x8D => Box::new(STA(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0x9D => Box::new(STA(AbsoluteX(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0x99 => Box::new(STA(AbsoluteY(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    0x81 => Box::new(STA(IndexedIndirect(memory.absolute(pc + 1)))),

    0x9A => Box::new(TXS),
    0xBA => Box::new(TSX),
    0x48 => Box::new(PHA),
    0x68 => Box::new(PLA),
    0x08 => Box::new(PHP),
    0x28 => Box::new(PLP),

    0x86 => Box::new(STX(ZeroPage(memory.absolute(pc + 1)))),
    0x96 => Box::new(STX(ZeroPageY(memory.absolute(pc + 1)))),
    0x8E => Box::new(STX(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),

    0x84 => Box::new(STY(ZeroPage(memory.absolute(pc + 1)))),
    0x94 => Box::new(STY(ZeroPageX(memory.absolute(pc + 1)))),
    0x8C => Box::new(STY(Absolute(u16::from_le_bytes([
      memory.absolute(pc + 1),
      memory.absolute(pc + 2),
    ])))),
    _ => panic!("Invalid instruction"),
  }
}
