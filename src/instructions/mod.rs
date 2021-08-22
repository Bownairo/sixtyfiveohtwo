#![allow(clippy::upper_case_acronyms)]
// TODO: Handling of flags

use crate::memory::Memory;
use crate::registers::Registers;

pub(crate) mod addressing;
pub(crate) mod interp;
use addressing::*;

pub(crate) trait Instruction: Renderable {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers);
}

pub(crate) trait Renderable {
  fn render(&self) -> Vec<u8>;
}

#[macro_export]
macro_rules! evaluate {
  ( $mem:expr, $reg:expr, $( $inst:expr ),* ) => {
    {
      $(
        $inst.evaluate($mem, $reg);
      )*
    }
  };
}

pub(crate) trait ADCAddressMode: AddressMode {}
impl ADCAddressMode for Immediate {}
impl ADCAddressMode for ZeroPage {}
impl ADCAddressMode for ZeroPageX {}
impl ADCAddressMode for Absolute {}
impl ADCAddressMode for AbsoluteX {}
impl ADCAddressMode for AbsoluteY {}
impl ADCAddressMode for IndexedIndirect {}
impl ADCAddressMode for IndirectIndexed {}

pub(crate) struct ADC<T: ADCAddressMode>(pub T);
impl<T: ADCAddressMode> Instruction for ADC<T>
where
  ADC<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    let carry = if registers.flags.carry { 1 } else { 0 };
    registers.acc.value += value + carry;

    registers.pc.value += 1 + T::LENGTH;
  }
}

impl Renderable for ADC<Immediate> {
  fn render(&self) -> Vec<u8> {
    [&[0x69], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ADC<ZeroPage> {
  fn render(&self) -> Vec<u8> {
    [&[0x65], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ADC<ZeroPageX> {
  fn render(&self) -> Vec<u8> {
    [&[0x75], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ADC<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0x6D], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ADC<AbsoluteX> {
  fn render(&self) -> Vec<u8> {
    [&[0x7D], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ADC<AbsoluteY> {
  fn render(&self) -> Vec<u8> {
    [&[0x79], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ADC<IndexedIndirect> {
  fn render(&self) -> Vec<u8> {
    [&[0x61], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ADC<IndirectIndexed> {
  fn render(&self) -> Vec<u8> {
    [&[0x71], self.0.render().as_slice()].concat()
  }
}

pub(crate) trait ANDAddressMode: AddressMode {}
impl ANDAddressMode for Immediate {}
impl ANDAddressMode for ZeroPage {}
impl ANDAddressMode for ZeroPageX {}
impl ANDAddressMode for Absolute {}
impl ANDAddressMode for AbsoluteX {}
impl ANDAddressMode for AbsoluteY {}
impl ANDAddressMode for IndexedIndirect {}
impl ANDAddressMode for IndirectIndexed {}

pub(crate) struct AND<T: ANDAddressMode>(pub T);
impl<T: ANDAddressMode> Instruction for AND<T>
where
  AND<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    registers.acc.value &= value;

    registers.pc.value += 1 + T::LENGTH;
  }
}

impl Renderable for AND<Immediate> {
  fn render(&self) -> Vec<u8> {
    [&[0x29], self.0.render().as_slice()].concat()
  }
}

impl Renderable for AND<ZeroPage> {
  fn render(&self) -> Vec<u8> {
    [&[0x25], self.0.render().as_slice()].concat()
  }
}

impl Renderable for AND<ZeroPageX> {
  fn render(&self) -> Vec<u8> {
    [&[0x35], self.0.render().as_slice()].concat()
  }
}

impl Renderable for AND<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0x2D], self.0.render().as_slice()].concat()
  }
}

impl Renderable for AND<AbsoluteX> {
  fn render(&self) -> Vec<u8> {
    [&[0x3D], self.0.render().as_slice()].concat()
  }
}

impl Renderable for AND<AbsoluteY> {
  fn render(&self) -> Vec<u8> {
    [&[0x39], self.0.render().as_slice()].concat()
  }
}

impl Renderable for AND<IndexedIndirect> {
  fn render(&self) -> Vec<u8> {
    [&[0x21], self.0.render().as_slice()].concat()
  }
}

impl Renderable for AND<IndirectIndexed> {
  fn render(&self) -> Vec<u8> {
    [&[0x31], self.0.render().as_slice()].concat()
  }
}

pub(crate) trait ASLAddressMode: AddressMode {}
impl ASLAddressMode for Accumulator {}
impl ASLAddressMode for ZeroPage {}
impl ASLAddressMode for ZeroPageX {}
impl ASLAddressMode for Absolute {}
impl ASLAddressMode for AbsoluteX {}

pub(crate) struct ASL<T: ASLAddressMode>(pub T);
impl<T: ASLAddressMode> Instruction for ASL<T>
where
  ASL<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);
    let shifted = value << 1;
    self.0.write(memory, registers, shifted);

    registers.pc.value += 1 + T::LENGTH;
  }
}

impl Renderable for ASL<Accumulator> {
  fn render(&self) -> Vec<u8> {
    [&[0x0A], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ASL<ZeroPage> {
  fn render(&self) -> Vec<u8> {
    [&[0x06], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ASL<ZeroPageX> {
  fn render(&self) -> Vec<u8> {
    [&[0x16], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ASL<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0x0E], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ASL<AbsoluteX> {
  fn render(&self) -> Vec<u8> {
    [&[0x1E], self.0.render().as_slice()].concat()
  }
}

pub(crate) trait BITAddressMode: AddressMode {}
impl BITAddressMode for ZeroPage {}
impl BITAddressMode for Absolute {}

pub(crate) struct BIT<T: BITAddressMode>(pub T);
impl<T: BITAddressMode> Instruction for BIT<T>
where
  BIT<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    registers.flags.zero = (value & registers.acc.value) == 0;
    registers.flags.negative = (value & (1 << 7)) != 0;
    registers.flags.overflow = (value & (1 << 6)) != 0;

    registers.pc.value += 1 + T::LENGTH;
  }
}

impl Renderable for BIT<ZeroPage> {
  fn render(&self) -> Vec<u8> {
    [&[0x24], self.0.render().as_slice()].concat()
  }
}

impl Renderable for BIT<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0x2C], self.0.render().as_slice()].concat()
  }
}

pub(crate) struct BPL(pub i8);
impl Instruction for BPL {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.pc.value += 2;
    if !registers.flags.negative {
      let target = registers.pc.value as i16 + self.0 as i16;
      registers.pc.value = target as u16;
    }
  }
}

impl Renderable for BPL {
  fn render(&self) -> Vec<u8> {
    vec![0x10]
  }
}

pub(crate) struct BMI(pub i8);
impl Instruction for BMI {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.pc.value += 2;
    if registers.flags.negative {
      let target = registers.pc.value as i16 + self.0 as i16;
      registers.pc.value = target as u16;
    }
  }
}

impl Renderable for BMI {
  fn render(&self) -> Vec<u8> {
    vec![0x30]
  }
}

pub(crate) struct BVC(pub i8);
impl Instruction for BVC {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.pc.value += 2;
    if !registers.flags.overflow {
      let target = registers.pc.value as i16 + self.0 as i16;
      registers.pc.value = target as u16;
    }
  }
}

impl Renderable for BVC {
  fn render(&self) -> Vec<u8> {
    vec![0x50]
  }
}

pub(crate) struct BVS(pub i8);
impl Instruction for BVS {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.pc.value += 2;
    if registers.flags.overflow {
      let target = registers.pc.value as i16 + self.0 as i16;
      registers.pc.value = target as u16;
    }
  }
}

impl Renderable for BVS {
  fn render(&self) -> Vec<u8> {
    vec![0x70]
  }
}

pub(crate) struct BCC(pub i8);
impl Instruction for BCC {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.pc.value += 2;
    if registers.flags.carry {
      let target = registers.pc.value as i16 + self.0 as i16;
      registers.pc.value = target as u16;
    }
  }
}

impl Renderable for BCC {
  fn render(&self) -> Vec<u8> {
    vec![0x90]
  }
}

pub(crate) struct BCS(pub i8);
impl Instruction for BCS {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.pc.value += 2;
    if !registers.flags.carry {
      let target = registers.pc.value as i16 + self.0 as i16;
      registers.pc.value = target as u16;
    }
  }
}

impl Renderable for BCS {
  fn render(&self) -> Vec<u8> {
    vec![0xB0]
  }
}

pub(crate) struct BNE(pub i8);
impl Instruction for BNE {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.pc.value += 2;
    if !registers.flags.zero {
      let target = registers.pc.value as i16 + self.0 as i16;
      registers.pc.value = target as u16;
    }
  }
}

impl Renderable for BNE {
  fn render(&self) -> Vec<u8> {
    vec![0xD0]
  }
}

pub(crate) struct BEQ(pub i8);
impl Instruction for BEQ {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.pc.value += 2;
    if registers.flags.zero {
      let target = registers.pc.value as i16 + self.0 as i16;
      registers.pc.value = target as u16;
    }
  }
}

impl Renderable for BEQ {
  fn render(&self) -> Vec<u8> {
    vec![0xF0]
  }
}

pub(crate) struct BRK;
impl Instruction for BRK {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    // TODO: Interupt
    registers.pc.value += 1;
  }
}

impl Renderable for BRK {
  fn render(&self) -> Vec<u8> {
    vec![0x00]
  }
}

pub(crate) trait CMPAddressMode: AddressMode {}
impl CMPAddressMode for Immediate {}
impl CMPAddressMode for ZeroPage {}
impl CMPAddressMode for ZeroPageX {}
impl CMPAddressMode for Absolute {}
impl CMPAddressMode for AbsoluteX {}
impl CMPAddressMode for AbsoluteY {}
impl CMPAddressMode for IndexedIndirect {}
impl CMPAddressMode for IndirectIndexed {}

pub(crate) struct CMP<T: CMPAddressMode>(pub T);
impl<T: CMPAddressMode> Instruction for CMP<T>
where
  CMP<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    let test = registers.acc.value - value;

    registers.flags.carry = test >= 0;
    registers.flags.zero = test == 0;
    registers.flags.negative = test < 0;

    registers.pc.value += 1 + T::LENGTH;
  }
}

impl Renderable for CMP<Immediate> {
  fn render(&self) -> Vec<u8> {
    [&[0xC9], self.0.render().as_slice()].concat()
  }
}

impl Renderable for CMP<ZeroPage> {
  fn render(&self) -> Vec<u8> {
    [&[0xC5], self.0.render().as_slice()].concat()
  }
}

impl Renderable for CMP<ZeroPageX> {
  fn render(&self) -> Vec<u8> {
    [&[0xD5], self.0.render().as_slice()].concat()
  }
}

impl Renderable for CMP<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0xCD], self.0.render().as_slice()].concat()
  }
}

impl Renderable for CMP<AbsoluteX> {
  fn render(&self) -> Vec<u8> {
    [&[0xDD], self.0.render().as_slice()].concat()
  }
}

impl Renderable for CMP<AbsoluteY> {
  fn render(&self) -> Vec<u8> {
    [&[0xD9], self.0.render().as_slice()].concat()
  }
}

impl Renderable for CMP<IndexedIndirect> {
  fn render(&self) -> Vec<u8> {
    [&[0xC1], self.0.render().as_slice()].concat()
  }
}

impl Renderable for CMP<IndirectIndexed> {
  fn render(&self) -> Vec<u8> {
    [&[0xD1], self.0.render().as_slice()].concat()
  }
}

pub(crate) trait CPXAddressMode: AddressMode {}
impl CPXAddressMode for Immediate {}
impl CPXAddressMode for ZeroPage {}
impl CPXAddressMode for Absolute {}

pub(crate) struct CPX<T: CPXAddressMode>(pub T);
impl<T: CPXAddressMode> Instruction for CPX<T>
where
  CPX<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    let test = registers.x.value - value;

    registers.flags.carry = test >= 0;
    registers.flags.zero = test == 0;
    registers.flags.negative = test < 0;

    registers.pc.value += 1 + T::LENGTH;
  }
}

impl Renderable for CPX<Immediate> {
  fn render(&self) -> Vec<u8> {
    [&[0xE0], self.0.render().as_slice()].concat()
  }
}

impl Renderable for CPX<ZeroPage> {
  fn render(&self) -> Vec<u8> {
    [&[0xE4], self.0.render().as_slice()].concat()
  }
}

impl Renderable for CPX<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0xEC], self.0.render().as_slice()].concat()
  }
}

pub(crate) trait CPYAddressMode: AddressMode {}
impl CPYAddressMode for Immediate {}
impl CPYAddressMode for ZeroPage {}
impl CPYAddressMode for Absolute {}

pub(crate) struct CPY<T: CPYAddressMode>(pub T);
impl<T: CPYAddressMode> Instruction for CPY<T>
where
  CPY<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    let test = registers.y.value - value;

    registers.flags.carry = test >= 0;
    registers.flags.zero = test == 0;
    registers.flags.negative = test < 0;

    registers.pc.value += 1 + T::LENGTH;
  }
}

impl Renderable for CPY<Immediate> {
  fn render(&self) -> Vec<u8> {
    [&[0xC0], self.0.render().as_slice()].concat()
  }
}

impl Renderable for CPY<ZeroPage> {
  fn render(&self) -> Vec<u8> {
    [&[0xC4], self.0.render().as_slice()].concat()
  }
}

impl Renderable for CPY<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0xCC], self.0.render().as_slice()].concat()
  }
}

pub(crate) trait DECAddressMode: AddressMode {}
impl DECAddressMode for ZeroPage {}
impl DECAddressMode for ZeroPageX {}
impl DECAddressMode for Absolute {}
impl DECAddressMode for AbsoluteX {}

pub(crate) struct DEC<T: DECAddressMode>(pub T);
impl<T: DECAddressMode> Instruction for DEC<T>
where
  DEC<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);
    self.0.write(memory, registers, value - 1);

    registers.pc.value += 1 + T::LENGTH;
  }
}

impl Renderable for DEC<ZeroPage> {
  fn render(&self) -> Vec<u8> {
    [&[0xC6], self.0.render().as_slice()].concat()
  }
}

impl Renderable for DEC<ZeroPageX> {
  fn render(&self) -> Vec<u8> {
    [&[0xD6], self.0.render().as_slice()].concat()
  }
}

impl Renderable for DEC<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0xCE], self.0.render().as_slice()].concat()
  }
}

impl Renderable for DEC<AbsoluteX> {
  fn render(&self) -> Vec<u8> {
    [&[0xDE], self.0.render().as_slice()].concat()
  }
}

pub(crate) trait EORAddressMode: AddressMode {}
impl EORAddressMode for Immediate {}
impl EORAddressMode for ZeroPage {}
impl EORAddressMode for ZeroPageX {}
impl EORAddressMode for Absolute {}
impl EORAddressMode for AbsoluteX {}
impl EORAddressMode for AbsoluteY {}
impl EORAddressMode for IndexedIndirect {}
impl EORAddressMode for IndirectIndexed {}

pub(crate) struct EOR<T: EORAddressMode>(pub T);
impl<T: EORAddressMode> Instruction for EOR<T>
where
  EOR<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    let result = value ^ registers.acc.value;

    self.0.write(memory, registers, result);

    registers.pc.value += 1 + T::LENGTH;
  }
}

impl Renderable for EOR<Immediate> {
  fn render(&self) -> Vec<u8> {
    [&[0x49], self.0.render().as_slice()].concat()
  }
}

impl Renderable for EOR<ZeroPage> {
  fn render(&self) -> Vec<u8> {
    [&[0x45], self.0.render().as_slice()].concat()
  }
}

impl Renderable for EOR<ZeroPageX> {
  fn render(&self) -> Vec<u8> {
    [&[0x55], self.0.render().as_slice()].concat()
  }
}

impl Renderable for EOR<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0x4D], self.0.render().as_slice()].concat()
  }
}

impl Renderable for EOR<AbsoluteX> {
  fn render(&self) -> Vec<u8> {
    [&[0x5D], self.0.render().as_slice()].concat()
  }
}

impl Renderable for EOR<AbsoluteY> {
  fn render(&self) -> Vec<u8> {
    [&[0x59], self.0.render().as_slice()].concat()
  }
}

impl Renderable for EOR<IndexedIndirect> {
  fn render(&self) -> Vec<u8> {
    [&[0x41], self.0.render().as_slice()].concat()
  }
}

impl Renderable for EOR<IndirectIndexed> {
  fn render(&self) -> Vec<u8> {
    [&[0x51], self.0.render().as_slice()].concat()
  }
}

pub(crate) struct CLC;
impl Instruction for CLC {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.flags.carry = false;

    registers.pc.value += 1;
  }
}

impl Renderable for CLC {
  fn render(&self) -> Vec<u8> {
    vec![0x18]
  }
}

pub(crate) struct SEC;
impl Instruction for SEC {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.flags.carry = true;

    registers.pc.value += 1;
  }
}

impl Renderable for SEC {
  fn render(&self) -> Vec<u8> {
    vec![0x38]
  }
}

pub(crate) struct CLI;
impl Instruction for CLI {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.flags.interrupt_disable = false;

    registers.pc.value += 1;
  }
}

impl Renderable for CLI {
  fn render(&self) -> Vec<u8> {
    vec![0x58]
  }
}

pub(crate) struct SEI;
impl Instruction for SEI {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.flags.interrupt_disable = true;

    registers.pc.value += 1;
  }
}

impl Renderable for SEI {
  fn render(&self) -> Vec<u8> {
    vec![0x78]
  }
}

pub(crate) struct CLV;
impl Instruction for CLV {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.flags.overflow = false;

    registers.pc.value += 1;
  }
}

impl Renderable for CLV {
  fn render(&self) -> Vec<u8> {
    vec![0xB8]
  }
}

pub(crate) struct CLD;
impl Instruction for CLD {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.flags.decimal_mode = false;

    registers.pc.value += 1;
  }
}

impl Renderable for CLD {
  fn render(&self) -> Vec<u8> {
    vec![0xD8]
  }
}

pub(crate) struct SED;
impl Instruction for SED {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.flags.decimal_mode = true;

    registers.pc.value += 1;
  }
}

impl Renderable for SED {
  fn render(&self) -> Vec<u8> {
    vec![0xF8]
  }
}

pub(crate) trait INCAddressMode: AddressMode {}
impl INCAddressMode for ZeroPage {}
impl INCAddressMode for ZeroPageX {}
impl INCAddressMode for Absolute {}
impl INCAddressMode for AbsoluteX {}

pub(crate) struct INC<T: INCAddressMode>(pub T);
impl<T: INCAddressMode> Instruction for INC<T>
where
  INC<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);
    self.0.write(memory, registers, value + 1);

    registers.pc.value += 1;
  }
}

impl Renderable for INC<ZeroPage> {
  fn render(&self) -> Vec<u8> {
    [&[0xE6], self.0.render().as_slice()].concat()
  }
}

impl Renderable for INC<ZeroPageX> {
  fn render(&self) -> Vec<u8> {
    [&[0xF6], self.0.render().as_slice()].concat()
  }
}

impl Renderable for INC<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0xEE], self.0.render().as_slice()].concat()
  }
}

impl Renderable for INC<AbsoluteX> {
  fn render(&self) -> Vec<u8> {
    [&[0xFE], self.0.render().as_slice()].concat()
  }
}

pub(crate) trait JMPAddressMode: JumpMode {}
impl JMPAddressMode for Absolute {}
impl JMPAddressMode for Indirect {}

pub(crate) struct JMP<T: JMPAddressMode>(pub T);
impl<T: JMPAddressMode> Instruction for JMP<T>
where
  JMP<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    registers.pc.value = self.0.dest(memory);
  }
}

impl Renderable for JMP<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0x4C], self.0.render().as_slice()].concat()
  }
}

impl Renderable for JMP<Indirect> {
  fn render(&self) -> Vec<u8> {
    [&[0x6C], self.0.render().as_slice()].concat()
  }
}

pub(crate) struct JSR(u16);
impl Instruction for JSR {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let addr = (registers.pc.value + 2).to_le_bytes();
    memory
      .absolute_write(u16::from_le_bytes([0x01, registers.sp.value]), addr[0]);
    memory.absolute_write(
      u16::from_le_bytes([0x01, registers.sp.value - 1]),
      addr[1],
    );
    registers.sp.value -= 2;

    registers.pc.value = self.0;
  }
}

impl Renderable for JSR {
  fn render(&self) -> Vec<u8> {
    [&[0x20], &self.0.to_le_bytes()[..]].concat()
  }
}

pub(crate) trait LDAAddressMode: AddressMode {}
impl LDAAddressMode for Immediate {}
impl LDAAddressMode for ZeroPage {}
impl LDAAddressMode for ZeroPageX {}
impl LDAAddressMode for Absolute {}
impl LDAAddressMode for AbsoluteX {}
impl LDAAddressMode for AbsoluteY {}
impl LDAAddressMode for IndexedIndirect {}
impl LDAAddressMode for IndirectIndexed {}

pub(crate) struct LDA<T: LDAAddressMode>(pub T);
impl<T: LDAAddressMode> Instruction for LDA<T>
where
  LDA<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    registers.acc.value = value;

    registers.pc.value += 1 + T::LENGTH;
  }
}

impl Renderable for LDA<Immediate> {
  fn render(&self) -> Vec<u8> {
    [&[0xA9], self.0.render().as_slice()].concat()
  }
}

impl Renderable for LDA<ZeroPage> {
  fn render(&self) -> Vec<u8> {
    [&[0xA5], self.0.render().as_slice()].concat()
  }
}

impl Renderable for LDA<ZeroPageX> {
  fn render(&self) -> Vec<u8> {
    [&[0xB5], self.0.render().as_slice()].concat()
  }
}

impl Renderable for LDA<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0xAD], self.0.render().as_slice()].concat()
  }
}

impl Renderable for LDA<AbsoluteX> {
  fn render(&self) -> Vec<u8> {
    [&[0xBD], self.0.render().as_slice()].concat()
  }
}

impl Renderable for LDA<AbsoluteY> {
  fn render(&self) -> Vec<u8> {
    [&[0xB9], self.0.render().as_slice()].concat()
  }
}

impl Renderable for LDA<IndexedIndirect> {
  fn render(&self) -> Vec<u8> {
    [&[0xA1], self.0.render().as_slice()].concat()
  }
}

impl Renderable for LDA<IndirectIndexed> {
  fn render(&self) -> Vec<u8> {
    [&[0xB1], self.0.render().as_slice()].concat()
  }
}

pub(crate) trait LDXAddressMode: AddressMode {}
impl LDXAddressMode for Immediate {}
impl LDXAddressMode for ZeroPage {}
impl LDXAddressMode for ZeroPageY {}
impl LDXAddressMode for Absolute {}
impl LDXAddressMode for AbsoluteY {}

pub(crate) struct LDX<T: LDXAddressMode>(pub T);
impl<T: LDXAddressMode> Instruction for LDX<T>
where
  LDX<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    registers.x.value = value;

    registers.pc.value += 1 + T::LENGTH;
  }
}

impl Renderable for LDX<Immediate> {
  fn render(&self) -> Vec<u8> {
    [&[0xA2], self.0.render().as_slice()].concat()
  }
}

impl Renderable for LDX<ZeroPage> {
  fn render(&self) -> Vec<u8> {
    [&[0xA6], self.0.render().as_slice()].concat()
  }
}

impl Renderable for LDX<ZeroPageY> {
  fn render(&self) -> Vec<u8> {
    [&[0xB6], self.0.render().as_slice()].concat()
  }
}

impl Renderable for LDX<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0xAE], self.0.render().as_slice()].concat()
  }
}

impl Renderable for LDX<AbsoluteY> {
  fn render(&self) -> Vec<u8> {
    [&[0xBE], self.0.render().as_slice()].concat()
  }
}

pub(crate) trait LDYAddressMode: AddressMode {}
impl LDYAddressMode for Immediate {}
impl LDYAddressMode for ZeroPage {}
impl LDYAddressMode for ZeroPageX {}
impl LDYAddressMode for Absolute {}
impl LDYAddressMode for AbsoluteX {}

pub(crate) struct LDY<T: LDYAddressMode>(pub T);
impl<T: LDYAddressMode> Instruction for LDY<T>
where
  LDY<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    registers.y.value = value;

    registers.pc.value += 1 + T::LENGTH;
  }
}

impl Renderable for LDY<Immediate> {
  fn render(&self) -> Vec<u8> {
    [&[0xA0], self.0.render().as_slice()].concat()
  }
}

impl Renderable for LDY<ZeroPage> {
  fn render(&self) -> Vec<u8> {
    [&[0xA4], self.0.render().as_slice()].concat()
  }
}

impl Renderable for LDY<ZeroPageX> {
  fn render(&self) -> Vec<u8> {
    [&[0xB4], self.0.render().as_slice()].concat()
  }
}

impl Renderable for LDY<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0xAC], self.0.render().as_slice()].concat()
  }
}

impl Renderable for LDY<AbsoluteX> {
  fn render(&self) -> Vec<u8> {
    [&[0xBC], self.0.render().as_slice()].concat()
  }
}

pub(crate) trait LSRAddressMode: AddressMode {}
impl LSRAddressMode for Accumulator {}
impl LSRAddressMode for ZeroPage {}
impl LSRAddressMode for ZeroPageX {}
impl LSRAddressMode for Absolute {}
impl LSRAddressMode for AbsoluteX {}

pub(crate) struct LSR<T: LSRAddressMode>(pub T);
impl<T: LSRAddressMode> Instruction for LSR<T>
where
  LSR<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);
    let shifted = value >> 1;
    self.0.write(memory, registers, shifted);

    registers.pc.value += 1 + T::LENGTH;
  }
}

impl Renderable for LSR<Accumulator> {
  fn render(&self) -> Vec<u8> {
    [&[0x4A], self.0.render().as_slice()].concat()
  }
}

impl Renderable for LSR<ZeroPage> {
  fn render(&self) -> Vec<u8> {
    [&[0x46], self.0.render().as_slice()].concat()
  }
}

impl Renderable for LSR<ZeroPageX> {
  fn render(&self) -> Vec<u8> {
    [&[0x56], self.0.render().as_slice()].concat()
  }
}

impl Renderable for LSR<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0x4E], self.0.render().as_slice()].concat()
  }
}

impl Renderable for LSR<AbsoluteX> {
  fn render(&self) -> Vec<u8> {
    [&[0x5E], self.0.render().as_slice()].concat()
  }
}

pub(crate) struct NOP;
impl Instruction for NOP {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.pc.value += 1;
  }
}

impl Renderable for NOP {
  fn render(&self) -> Vec<u8> {
    vec![0xEA]
  }
}

pub(crate) trait ORAAddressMode: AddressMode {}
impl ORAAddressMode for Immediate {}
impl ORAAddressMode for ZeroPage {}
impl ORAAddressMode for ZeroPageX {}
impl ORAAddressMode for Absolute {}
impl ORAAddressMode for AbsoluteX {}
impl ORAAddressMode for AbsoluteY {}
impl ORAAddressMode for IndexedIndirect {}
impl ORAAddressMode for IndirectIndexed {}

pub(crate) struct ORA<T: ORAAddressMode>(pub T);
impl<T: ORAAddressMode> Instruction for ORA<T>
where
  ORA<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    let ored = value | registers.acc.value;

    self.0.write(memory, registers, ored);

    registers.pc.value += 1 + T::LENGTH;
  }
}

impl Renderable for ORA<Immediate> {
  fn render(&self) -> Vec<u8> {
    [&[0x09], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ORA<ZeroPage> {
  fn render(&self) -> Vec<u8> {
    [&[0x05], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ORA<ZeroPageX> {
  fn render(&self) -> Vec<u8> {
    [&[0x15], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ORA<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0x0D], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ORA<AbsoluteX> {
  fn render(&self) -> Vec<u8> {
    [&[0x1D], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ORA<AbsoluteY> {
  fn render(&self) -> Vec<u8> {
    [&[0x19], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ORA<IndexedIndirect> {
  fn render(&self) -> Vec<u8> {
    [&[0x01], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ORA<IndirectIndexed> {
  fn render(&self) -> Vec<u8> {
    [&[0x11], self.0.render().as_slice()].concat()
  }
}

pub(crate) struct TAX;
impl Instruction for TAX {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.x.value = registers.acc.value;

    registers.pc.value += 1;
  }
}

impl Renderable for TAX {
  fn render(&self) -> Vec<u8> {
    vec![0xAA]
  }
}

pub(crate) struct TXA;
impl Instruction for TXA {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.acc.value = registers.x.value;

    registers.pc.value += 1;
  }
}

impl Renderable for TXA {
  fn render(&self) -> Vec<u8> {
    vec![0x8A]
  }
}

pub(crate) struct DEX;
impl Instruction for DEX {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.x.value -= 1;

    registers.pc.value += 1;
  }
}

impl Renderable for DEX {
  fn render(&self) -> Vec<u8> {
    vec![0xCA]
  }
}

pub(crate) struct INX;
impl Instruction for INX {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.x.value += 1;

    registers.pc.value += 1;
  }
}

impl Renderable for INX {
  fn render(&self) -> Vec<u8> {
    vec![0xE8]
  }
}

pub(crate) struct TAY;
impl Instruction for TAY {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.y.value = registers.acc.value;

    registers.pc.value += 1;
  }
}

impl Renderable for TAY {
  fn render(&self) -> Vec<u8> {
    vec![0xA8]
  }
}

pub(crate) struct TYA;
impl Instruction for TYA {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.acc.value = registers.y.value;

    registers.pc.value += 1;
  }
}

impl Renderable for TYA {
  fn render(&self) -> Vec<u8> {
    vec![0x98]
  }
}

pub(crate) struct DEY;
impl Instruction for DEY {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.y.value -= 1;

    registers.pc.value += 1;
  }
}

impl Renderable for DEY {
  fn render(&self) -> Vec<u8> {
    vec![0x88]
  }
}

pub(crate) struct INY;
impl Instruction for INY {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.y.value += 1;

    registers.pc.value += 1;
  }
}

impl Renderable for INY {
  fn render(&self) -> Vec<u8> {
    vec![0xC8]
  }
}

pub(crate) trait ROLAddressMode: AddressMode {}
impl ROLAddressMode for Accumulator {}
impl ROLAddressMode for ZeroPage {}
impl ROLAddressMode for ZeroPageX {}
impl ROLAddressMode for Absolute {}
impl ROLAddressMode for AbsoluteX {}

pub(crate) struct ROL<T: ROLAddressMode>(pub T);
impl<T: ROLAddressMode> Instruction for ROL<T>
where
  ROL<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    let rotated = value.rotate_left(1);

    self.0.write(memory, registers, rotated);

    registers.pc.value += 1 + T::LENGTH;
  }
}

impl Renderable for ROL<Accumulator> {
  fn render(&self) -> Vec<u8> {
    [&[0x2A], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ROL<ZeroPage> {
  fn render(&self) -> Vec<u8> {
    [&[0x26], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ROL<ZeroPageX> {
  fn render(&self) -> Vec<u8> {
    [&[0x36], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ROL<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0x2E], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ROL<AbsoluteX> {
  fn render(&self) -> Vec<u8> {
    [&[0x3E], self.0.render().as_slice()].concat()
  }
}

pub(crate) trait RORAddressMode: AddressMode {}
impl RORAddressMode for Accumulator {}
impl RORAddressMode for ZeroPage {}
impl RORAddressMode for ZeroPageX {}
impl RORAddressMode for Absolute {}
impl RORAddressMode for AbsoluteX {}

pub(crate) struct ROR<T: RORAddressMode>(pub T);
impl<T: RORAddressMode> Instruction for ROR<T>
where
  ROR<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    let rotated = value.rotate_right(1);

    self.0.write(memory, registers, rotated);

    registers.pc.value += 1 + T::LENGTH;
  }
}

impl Renderable for ROR<Accumulator> {
  fn render(&self) -> Vec<u8> {
    [&[0x6A], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ROR<ZeroPage> {
  fn render(&self) -> Vec<u8> {
    [&[0x66], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ROR<ZeroPageX> {
  fn render(&self) -> Vec<u8> {
    [&[0x76], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ROR<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0x6E], self.0.render().as_slice()].concat()
  }
}

impl Renderable for ROR<AbsoluteX> {
  fn render(&self) -> Vec<u8> {
    [&[0x7E], self.0.render().as_slice()].concat()
  }
}

pub(crate) struct RTI;
impl Instruction for RTI {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    registers.flags.write(
      memory.absolute(u16::from_le_bytes([0x01, registers.sp.value + 1])),
    );

    let first =
      memory.absolute(u16::from_le_bytes([0x01, registers.sp.value + 2]));
    let second =
      memory.absolute(u16::from_le_bytes([0x01, registers.sp.value + 3]));
    registers.pc.value = u16::from_le_bytes([first, second]);
    registers.sp.value += 3;
  }
}

impl Renderable for RTI {
  fn render(&self) -> Vec<u8> {
    vec![0x40]
  }
}

pub(crate) struct RTS;
impl Instruction for RTS {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let first =
      memory.absolute(u16::from_le_bytes([0x01, registers.sp.value + 1]));
    let second =
      memory.absolute(u16::from_le_bytes([0x01, registers.sp.value + 2]));
    registers.pc.value = u16::from_le_bytes([first, second]);
    registers.sp.value += 2;

    registers.pc.value += 1;
  }
}

impl Renderable for RTS {
  fn render(&self) -> Vec<u8> {
    vec![0x60]
  }
}

pub(crate) trait SBCAddressMode: AddressMode {}
impl SBCAddressMode for Immediate {}
impl SBCAddressMode for ZeroPage {}
impl SBCAddressMode for ZeroPageX {}
impl SBCAddressMode for Absolute {}
impl SBCAddressMode for AbsoluteX {}
impl SBCAddressMode for AbsoluteY {}
impl SBCAddressMode for IndexedIndirect {}
impl SBCAddressMode for IndirectIndexed {}

pub(crate) struct SBC<T: SBCAddressMode>(pub T);
impl<T: SBCAddressMode> Instruction for SBC<T>
where
  SBC<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    registers.acc.value -= value;

    registers.pc.value += 1 + T::LENGTH;
  }
}

impl Renderable for SBC<Immediate> {
  fn render(&self) -> Vec<u8> {
    [&[0xE9], self.0.render().as_slice()].concat()
  }
}

impl Renderable for SBC<ZeroPage> {
  fn render(&self) -> Vec<u8> {
    [&[0xE5], self.0.render().as_slice()].concat()
  }
}

impl Renderable for SBC<ZeroPageX> {
  fn render(&self) -> Vec<u8> {
    [&[0xF5], self.0.render().as_slice()].concat()
  }
}

impl Renderable for SBC<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0xED], self.0.render().as_slice()].concat()
  }
}

impl Renderable for SBC<AbsoluteX> {
  fn render(&self) -> Vec<u8> {
    [&[0xFD], self.0.render().as_slice()].concat()
  }
}

impl Renderable for SBC<AbsoluteY> {
  fn render(&self) -> Vec<u8> {
    [&[0xF9], self.0.render().as_slice()].concat()
  }
}

impl Renderable for SBC<IndexedIndirect> {
  fn render(&self) -> Vec<u8> {
    [&[0xE1], self.0.render().as_slice()].concat()
  }
}

impl Renderable for SBC<IndirectIndexed> {
  fn render(&self) -> Vec<u8> {
    [&[0xF1], self.0.render().as_slice()].concat()
  }
}

pub(crate) trait STAAddressMode: AddressMode {}
impl STAAddressMode for ZeroPage {}
impl STAAddressMode for ZeroPageX {}
impl STAAddressMode for Absolute {}
impl STAAddressMode for AbsoluteX {}
impl STAAddressMode for AbsoluteY {}
impl STAAddressMode for IndexedIndirect {}
impl STAAddressMode for IndirectIndexed {}

pub(crate) struct STA<T: STAAddressMode>(pub T);
impl<T: STAAddressMode> Instruction for STA<T>
where
  STA<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    self.0.write(memory, registers, registers.acc.value);

    registers.pc.value += 1 + T::LENGTH;
  }
}

impl Renderable for STA<ZeroPage> {
  fn render(&self) -> Vec<u8> {
    [&[0x85], self.0.render().as_slice()].concat()
  }
}

impl Renderable for STA<ZeroPageX> {
  fn render(&self) -> Vec<u8> {
    [&[0x95], self.0.render().as_slice()].concat()
  }
}

impl Renderable for STA<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0x8D], self.0.render().as_slice()].concat()
  }
}

impl Renderable for STA<AbsoluteX> {
  fn render(&self) -> Vec<u8> {
    [&[0x9D], self.0.render().as_slice()].concat()
  }
}

impl Renderable for STA<AbsoluteY> {
  fn render(&self) -> Vec<u8> {
    [&[0x99], self.0.render().as_slice()].concat()
  }
}

impl Renderable for STA<IndexedIndirect> {
  fn render(&self) -> Vec<u8> {
    [&[0x81], self.0.render().as_slice()].concat()
  }
}

impl Renderable for STA<IndirectIndexed> {
  fn render(&self) -> Vec<u8> {
    [&[0x91], self.0.render().as_slice()].concat()
  }
}

pub(crate) struct TXS;
impl Instruction for TXS {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    memory.absolute_write(
      u16::from_le_bytes([0x01, registers.sp.value]),
      registers.x.value as u8,
    );
    registers.sp.value -= 1;

    registers.pc.value += 1;
  }
}

impl Renderable for TXS {
  fn render(&self) -> Vec<u8> {
    vec![0x9A]
  }
}

pub(crate) struct TSX;
impl Instruction for TSX {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    registers.x.value =
      memory.absolute(u16::from_le_bytes([0x01, registers.sp.value + 1])) as i8;
    registers.sp.value += 1;

    registers.pc.value += 1;
  }
}

impl Renderable for TSX {
  fn render(&self) -> Vec<u8> {
    vec![0xBA]
  }
}

pub(crate) struct PHA;
impl Instruction for PHA {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    memory.absolute_write(
      u16::from_le_bytes([0x01, registers.sp.value]),
      registers.acc.value as u8,
    );
    registers.sp.value -= 1;

    registers.pc.value += 1;
  }
}

impl Renderable for PHA {
  fn render(&self) -> Vec<u8> {
    vec![0x48]
  }
}

pub(crate) struct PLA;
impl Instruction for PLA {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    registers.acc.value =
      memory.absolute(u16::from_le_bytes([0x01, registers.sp.value + 1])) as i8;
    registers.sp.value += 1;

    registers.pc.value += 1;
  }
}

impl Renderable for PLA {
  fn render(&self) -> Vec<u8> {
    vec![0x68]
  }
}

pub(crate) struct PHP;
impl Instruction for PHP {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    use crate::registers::Register;

    memory.absolute_write(
      u16::from_le_bytes([0x01, registers.sp.value]),
      registers.flags.raw() as u8,
    );
    registers.sp.value -= 1;

    registers.pc.value += 1;
  }
}

impl Renderable for PHP {
  fn render(&self) -> Vec<u8> {
    vec![0x08]
  }
}

pub(crate) struct PLP;
impl Instruction for PLP {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    // registers.flags.value =
    //   memory.absolute(u16::from_le_bytes([0x01, registers.sp.value + 1])) as i8;
    registers.sp.value += 1;

    registers.pc.value += 1;
  }
}

impl Renderable for PLP {
  fn render(&self) -> Vec<u8> {
    vec![0x28]
  }
}

pub(crate) trait STXAddressMode: AddressMode {}
impl STXAddressMode for ZeroPage {}
impl STXAddressMode for ZeroPageY {}
impl STXAddressMode for Absolute {}

pub(crate) struct STX<T: STXAddressMode>(pub T);
impl<T: STXAddressMode> Instruction for STX<T>
where
  STX<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    self.0.write(memory, registers, registers.x.value);

    registers.pc.value += 1 + T::LENGTH;
  }
}

impl Renderable for STX<ZeroPage> {
  fn render(&self) -> Vec<u8> {
    [&[0x86], self.0.render().as_slice()].concat()
  }
}

impl Renderable for STX<ZeroPageY> {
  fn render(&self) -> Vec<u8> {
    [&[0x96], self.0.render().as_slice()].concat()
  }
}

impl Renderable for STX<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0x8E], self.0.render().as_slice()].concat()
  }
}

pub(crate) trait STYAddressMode: AddressMode {}
impl STYAddressMode for ZeroPage {}
impl STYAddressMode for ZeroPageX {}
impl STYAddressMode for Absolute {}

pub(crate) struct STY<T: STYAddressMode>(pub T);
impl<T: STYAddressMode> Instruction for STY<T>
where
  STY<T>: Renderable,
{
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    self.0.write(memory, registers, registers.y.value);

    registers.pc.value += 1 + T::LENGTH;
  }
}

impl Renderable for STY<ZeroPage> {
  fn render(&self) -> Vec<u8> {
    [&[0x84], self.0.render().as_slice()].concat()
  }
}

impl Renderable for STY<ZeroPageX> {
  fn render(&self) -> Vec<u8> {
    [&[0x94], self.0.render().as_slice()].concat()
  }
}

impl Renderable for STY<Absolute> {
  fn render(&self) -> Vec<u8> {
    [&[0x8C], self.0.render().as_slice()].concat()
  }
}
