#![allow(clippy::upper_case_acronyms)]
// TODO: Handling of flags
// TODO: Binary mode

use crate::memory::Memory;
use crate::registers::Registers;

pub(crate) mod addressing;
use addressing::*;

pub(crate) trait Instruction {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers);
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
impl<T: ADCAddressMode> Instruction for ADC<T> {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    let carry = if registers.flags.carry { 1 } else { 0 };
    registers.acc.value += value + carry;
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
impl<T: ANDAddressMode> Instruction for AND<T> {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    registers.acc.value &= value;
  }
}

pub(crate) trait ASLAddressMode: AddressMode {}
impl ASLAddressMode for Accumulator {}
impl ASLAddressMode for ZeroPage {}
impl ASLAddressMode for ZeroPageX {}
impl ASLAddressMode for Absolute {}
impl ASLAddressMode for AbsoluteX {}

pub(crate) struct ASL<T: ASLAddressMode>(pub T);
impl<T: ASLAddressMode> Instruction for ASL<T> {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);
    let shifted = value << 1;
    self.0.write(memory, registers, shifted);
  }
}

pub(crate) trait BITAddressMode: AddressMode {}
impl BITAddressMode for ZeroPage {}
impl BITAddressMode for Absolute {}

pub(crate) struct BIT<T: BITAddressMode>(pub T);
impl<T: BITAddressMode> Instruction for BIT<T> {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    registers.flags.zero = (value & registers.acc.value) == 0;
    registers.flags.negative = (value & (1 << 7)) != 0;
    registers.flags.overflow = (value & (1 << 6)) != 0;
  }
}

pub(crate) struct BPL(pub u8);
impl Instruction for BPL {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    if !registers.flags.negative {
      // TODO
    }
  }
}

pub(crate) struct BMI(pub u8);
impl Instruction for BMI {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    if registers.flags.negative {
      // TODO
    }
  }
}

pub(crate) struct BVC(pub u8);
impl Instruction for BVC {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    if !registers.flags.overflow {
      // TODO
    }
  }
}

pub(crate) struct BVS(pub u8);
impl Instruction for BVS {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    if registers.flags.overflow {
      // TODO
    }
  }
}

pub(crate) struct BCC(pub u8);
impl Instruction for BCC {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    if registers.flags.carry {
      // TODO
    }
  }
}

pub(crate) struct BCS(pub u8);
impl Instruction for BCS {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    if !registers.flags.carry {
      // TODO
    }
  }
}

pub(crate) struct BNE(pub u8);
impl Instruction for BNE {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    if !registers.flags.zero {
      // TODO
    }
  }
}

pub(crate) struct BEQ(pub u8);
impl Instruction for BEQ {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    if registers.flags.zero {
      // TODO
    }
  }
}

pub(crate) struct BRK(pub u8);
impl Instruction for BRK {
  fn evaluate(&self, _memory: &mut Memory, _registers: &mut Registers) {
    // TODO
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
impl<T: CMPAddressMode> Instruction for CMP<T> {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    let test = registers.acc.value - value;

    registers.flags.carry = test >= 0;
    registers.flags.zero = test == 0;
    registers.flags.negative = test < 0;
  }
}

pub(crate) trait CPXAddressMode: AddressMode {}
impl CPXAddressMode for Immediate {}
impl CPXAddressMode for ZeroPage {}
impl CPXAddressMode for Absolute {}

pub(crate) struct CPX<T: CPXAddressMode>(pub T);
impl<T: CPXAddressMode> Instruction for CPX<T> {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    let test = registers.x.value - value;

    registers.flags.carry = test >= 0;
    registers.flags.zero = test == 0;
    registers.flags.negative = test < 0;
  }
}

pub(crate) trait CPYAddressMode: AddressMode {}
impl CPYAddressMode for Immediate {}
impl CPYAddressMode for ZeroPage {}
impl CPYAddressMode for Absolute {}

pub(crate) struct CPY<T: CPYAddressMode>(pub T);
impl<T: CPYAddressMode> Instruction for CPY<T> {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    let test = registers.y.value - value;

    registers.flags.carry = test >= 0;
    registers.flags.zero = test == 0;
    registers.flags.negative = test < 0;
  }
}

pub(crate) trait DECAddressMode: AddressMode {}
impl DECAddressMode for ZeroPage {}
impl DECAddressMode for ZeroPageX {}
impl DECAddressMode for Absolute {}
impl DECAddressMode for AbsoluteX {}

pub(crate) struct DEC<T: DECAddressMode>(pub T);
impl<T: DECAddressMode> Instruction for DEC<T> {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);
    self.0.write(memory, registers, value - 1);
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
impl<T: EORAddressMode> Instruction for EOR<T> {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    let result = value ^ registers.acc.value;

    self.0.write(memory, registers, result);
  }
}

pub(crate) struct CLC;
impl Instruction for CLC {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.flags.carry = false;
  }
}

pub(crate) struct SEC;
impl Instruction for SEC {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.flags.carry = true;
  }
}

pub(crate) struct CLI;
impl Instruction for CLI {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.flags.interrupt_disable = false;
  }
}

pub(crate) struct SEI;
impl Instruction for SEI {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.flags.interrupt_disable = true;
  }
}

pub(crate) struct CLV;
impl Instruction for CLV {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.flags.overflow = false;
  }
}

pub(crate) struct CLD;
impl Instruction for CLD {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.flags.decimal_mode = false;
  }
}

pub(crate) struct SED;
impl Instruction for SED {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.flags.decimal_mode = true;
  }
}

pub(crate) trait INCAddressMode: AddressMode {}
impl INCAddressMode for ZeroPage {}
impl INCAddressMode for ZeroPageX {}
impl INCAddressMode for Absolute {}
impl INCAddressMode for AbsoluteX {}

pub(crate) struct INC<T: INCAddressMode>(pub T);
impl<T: INCAddressMode> Instruction for INC<T> {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);
    self.0.write(memory, registers, value + 1);
  }
}

pub(crate) struct JMP;
impl Instruction for JMP {
  fn evaluate(&self, _memory: &mut Memory, _registers: &mut Registers) {
    // TODO
  }
}

pub(crate) struct JSR;
impl Instruction for JSR {
  fn evaluate(&self, _memory: &mut Memory, _registers: &mut Registers) {
    // TODO
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
impl<T: LDAAddressMode> Instruction for LDA<T> {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    registers.acc.value = value;
  }
}

pub(crate) trait LDXAddressMode: AddressMode {}
impl LDXAddressMode for Immediate {}
impl LDXAddressMode for ZeroPage {}
impl LDXAddressMode for ZeroPageY {}
impl LDXAddressMode for Absolute {}
impl LDXAddressMode for AbsoluteY {}

pub(crate) struct LDX<T: LDXAddressMode>(pub T);
impl<T: LDXAddressMode> Instruction for LDX<T> {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    registers.x.value = value;
  }
}

pub(crate) trait LDYAddressMode: AddressMode {}
impl LDYAddressMode for Immediate {}
impl LDYAddressMode for ZeroPage {}
impl LDYAddressMode for ZeroPageX {}
impl LDYAddressMode for Absolute {}
impl LDYAddressMode for AbsoluteX {}

pub(crate) struct LDY<T: LDYAddressMode>(pub T);
impl<T: LDYAddressMode> Instruction for LDY<T> {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    registers.y.value = value;
  }
}

pub(crate) trait LSRAddressMode: AddressMode {}
impl LSRAddressMode for Accumulator {}
impl LSRAddressMode for ZeroPage {}
impl LSRAddressMode for ZeroPageX {}
impl LSRAddressMode for Absolute {}
impl LSRAddressMode for AbsoluteX {}

pub(crate) struct LSR<T: LSRAddressMode>(pub T);
impl<T: LSRAddressMode> Instruction for LSR<T> {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);
    let shifted = value >> 1;
    self.0.write(memory, registers, shifted);
  }
}

pub(crate) struct NOP;
impl Instruction for NOP {
  fn evaluate(&self, _memory: &mut Memory, _registers: &mut Registers) {}
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
impl<T: ORAAddressMode> Instruction for ORA<T> {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    let ored = value | registers.acc.value;

    self.0.write(memory, registers, ored);
  }
}

pub(crate) struct TAX;
impl Instruction for TAX {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.x.value = registers.acc.value;
  }
}

pub(crate) struct TXA;
impl Instruction for TXA {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.acc.value = registers.x.value;
  }
}

pub(crate) struct DEX;
impl Instruction for DEX {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.x.value -= 1;
  }
}

pub(crate) struct INX;
impl Instruction for INX {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.x.value += 1;
  }
}

pub(crate) struct TAY;
impl Instruction for TAY {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.y.value = registers.acc.value;
  }
}

pub(crate) struct TYA;
impl Instruction for TYA {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.acc.value = registers.y.value;
  }
}

pub(crate) struct DEY;
impl Instruction for DEY {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.y.value -= 1;
  }
}

pub(crate) struct INY;
impl Instruction for INY {
  fn evaluate(&self, _memory: &mut Memory, registers: &mut Registers) {
    registers.y.value += 1;
  }
}

pub(crate) trait ROLAddressMode: AddressMode {}
impl ROLAddressMode for Accumulator {}
impl ROLAddressMode for ZeroPage {}
impl ROLAddressMode for ZeroPageX {}
impl ROLAddressMode for Absolute {}
impl ROLAddressMode for AbsoluteX {}

pub(crate) struct ROL<T: ROLAddressMode>(pub T);
impl<T: ROLAddressMode> Instruction for ROL<T> {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    let rotated = value.rotate_left(1);

    self.0.write(memory, registers, rotated);
  }
}

pub(crate) trait RORAddressMode: AddressMode {}
impl RORAddressMode for Accumulator {}
impl RORAddressMode for ZeroPage {}
impl RORAddressMode for ZeroPageX {}
impl RORAddressMode for Absolute {}
impl RORAddressMode for AbsoluteX {}

pub(crate) struct ROR<T: RORAddressMode>(pub T);
impl<T: RORAddressMode> Instruction for ROR<T> {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    let rotated = value.rotate_right(1);

    self.0.write(memory, registers, rotated);
  }
}

pub(crate) struct RTI;
impl Instruction for RTI {
  fn evaluate(&self, _memory: &mut Memory, _registers: &mut Registers) {
    // TODO
  }
}

pub(crate) struct RTS;
impl Instruction for RTS {
  fn evaluate(&self, _memory: &mut Memory, _registers: &mut Registers) {
    // TODO
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
impl<T: SBCAddressMode> Instruction for SBC<T> {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    let value = self.0.read(memory, registers);

    registers.acc.value -= value;
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
impl<T: STAAddressMode> Instruction for STA<T> {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    self.0.write(memory, registers, registers.acc.value);
  }
}

pub(crate) struct TXS;
impl Instruction for TXS {
  fn evaluate(&self, _memory: &mut Memory, _registers: &mut Registers) {
    // TODO
  }
}

pub(crate) struct TSX;
impl Instruction for TSX {
  fn evaluate(&self, _memory: &mut Memory, _registers: &mut Registers) {
    // TODO
  }
}

pub(crate) struct PHA;
impl Instruction for PHA {
  fn evaluate(&self, _memory: &mut Memory, _registers: &mut Registers) {
    // TODO
  }
}

pub(crate) struct PLA;
impl Instruction for PLA {
  fn evaluate(&self, _memory: &mut Memory, _registers: &mut Registers) {
    // TODO
  }
}

pub(crate) struct PHP;
impl Instruction for PHP {
  fn evaluate(&self, _memory: &mut Memory, _registers: &mut Registers) {
    // TODO
  }
}

pub(crate) struct PLP;
impl Instruction for PLP {
  fn evaluate(&self, _memory: &mut Memory, _registers: &mut Registers) {
    // TODO
  }
}

pub(crate) trait STXAddressMode: AddressMode {}
impl STXAddressMode for ZeroPage {}
impl STXAddressMode for ZeroPageY {}
impl STXAddressMode for Absolute {}

pub(crate) struct STX<T: STXAddressMode>(pub T);
impl<T: STXAddressMode> Instruction for STX<T> {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    self.0.write(memory, registers, registers.x.value);
  }
}

pub(crate) trait STYAddressMode: AddressMode {}
impl STYAddressMode for ZeroPage {}
impl STYAddressMode for ZeroPageX {}
impl STYAddressMode for Absolute {}

pub(crate) struct STY<T: STYAddressMode>(pub T);
impl<T: STYAddressMode> Instruction for STY<T> {
  fn evaluate(&self, memory: &mut Memory, registers: &mut Registers) {
    self.0.write(memory, registers, registers.y.value);
  }
}
