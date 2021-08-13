use super::Renderable;
use crate::memory::Memory;
use crate::registers::Registers;

pub(crate) trait AddressMode: Renderable {
  const LENGTH: u16;
  fn read(&self, memory: &Memory, registers: &Registers) -> i8;
  fn write(&self, memory: &mut Memory, registers: &mut Registers, value: i8);
}

pub(crate) trait JumpMode {
  fn dest(&self, memory: &Memory) -> u16;
}

pub(crate) struct Accumulator;
impl AddressMode for Accumulator {
  const LENGTH: u16 = 0;

  fn read(&self, _memory: &Memory, registers: &Registers) -> i8 {
    registers.acc.value
  }
  fn write(&self, _memory: &mut Memory, registers: &mut Registers, value: i8) {
    registers.acc.value = value;
  }
}
impl Renderable for Accumulator {
  fn render(&self) -> Vec<u8> {
    vec![]
  }
}

pub(crate) struct Immediate(pub i8);
impl AddressMode for Immediate {
  const LENGTH: u16 = 1;

  fn read(&self, _memory: &Memory, _registers: &Registers) -> i8 {
    self.0
  }
  fn write(
    &self,
    _memory: &mut Memory,
    _registers: &mut Registers,
    _value: i8,
  ) {
    panic!("Cannot write to immediate value.");
  }
}
impl Renderable for Immediate {
  fn render(&self) -> Vec<u8> {
    vec![self.0 as u8]
  }
}

pub(crate) struct ZeroPage(pub u8);
impl AddressMode for ZeroPage {
  const LENGTH: u16 = 1;

  fn read(&self, memory: &Memory, _registers: &Registers) -> i8 {
    memory.zero_page(self.0) as i8
  }
  fn write(&self, memory: &mut Memory, _registers: &mut Registers, value: i8) {
    memory.zero_page_write(self.0, value as u8)
  }
}
impl Renderable for ZeroPage {
  fn render(&self) -> Vec<u8> {
    vec![self.0]
  }
}

pub(crate) struct ZeroPageX(pub u8);
impl AddressMode for ZeroPageX {
  const LENGTH: u16 = 1;

  fn read(&self, memory: &Memory, registers: &Registers) -> i8 {
    memory.zero_page_register(self.0, &registers.x) as i8
  }
  fn write(&self, memory: &mut Memory, registers: &mut Registers, value: i8) {
    memory.zero_page_register_write(self.0, &registers.x, value as u8)
  }
}
impl Renderable for ZeroPageX {
  fn render(&self) -> Vec<u8> {
    vec![self.0]
  }
}

pub(crate) struct ZeroPageY(pub u8);
impl AddressMode for ZeroPageY {
  const LENGTH: u16 = 1;

  fn read(&self, memory: &Memory, registers: &Registers) -> i8 {
    memory.zero_page_register(self.0, &registers.y) as i8
  }
  fn write(&self, memory: &mut Memory, registers: &mut Registers, value: i8) {
    memory.zero_page_register_write(self.0, &registers.y, value as u8)
  }
}
impl Renderable for ZeroPageY {
  fn render(&self) -> Vec<u8> {
    vec![self.0]
  }
}

pub(crate) struct Absolute(pub u16);
impl AddressMode for Absolute {
  const LENGTH: u16 = 2;

  fn read(&self, memory: &Memory, _registers: &Registers) -> i8 {
    memory.absolute(self.0) as i8
  }
  fn write(&self, memory: &mut Memory, _registers: &mut Registers, value: i8) {
    memory.absolute_write(self.0, value as u8)
  }
}
impl Renderable for Absolute {
  fn render(&self) -> Vec<u8> {
    self.0.to_le_bytes().to_vec()
  }
}
impl JumpMode for Absolute {
  fn dest(&self, _memory: &Memory) -> u16 {
    self.0
  }
}

pub(crate) struct AbsoluteX(pub u16);
impl AddressMode for AbsoluteX {
  const LENGTH: u16 = 2;

  fn read(&self, memory: &Memory, registers: &Registers) -> i8 {
    memory.absolute_register(self.0, &registers.x) as i8
  }
  fn write(&self, memory: &mut Memory, registers: &mut Registers, value: i8) {
    memory.absolute_register_write(self.0, &registers.x, value as u8)
  }
}
impl Renderable for AbsoluteX {
  fn render(&self) -> Vec<u8> {
    self.0.to_le_bytes().to_vec()
  }
}

pub(crate) struct AbsoluteY(pub u16);
impl AddressMode for AbsoluteY {
  const LENGTH: u16 = 2;

  fn read(&self, memory: &Memory, registers: &Registers) -> i8 {
    memory.absolute_register(self.0, &registers.y) as i8
  }
  fn write(&self, memory: &mut Memory, registers: &mut Registers, value: i8) {
    memory.absolute_register_write(self.0, &registers.y, value as u8)
  }
}
impl Renderable for AbsoluteY {
  fn render(&self) -> Vec<u8> {
    self.0.to_le_bytes().to_vec()
  }
}

pub(crate) struct Indirect(pub u16);
impl JumpMode for Indirect {
  fn dest(&self, memory: &Memory) -> u16 {
    memory.indirect(self.0)
  }
}
impl Renderable for Indirect {
  fn render(&self) -> Vec<u8> {
    self.0.to_le_bytes().to_vec()
  }
}

pub(crate) struct IndexedIndirect(pub u8);
impl AddressMode for IndexedIndirect {
  const LENGTH: u16 = 1;

  fn read(&self, memory: &Memory, registers: &Registers) -> i8 {
    memory.indexed_indirect(self.0, &registers.x) as i8
  }
  fn write(&self, memory: &mut Memory, registers: &mut Registers, value: i8) {
    memory.indexed_indirect_write(self.0, &registers.x, value as u8)
  }
}
impl Renderable for IndexedIndirect {
  fn render(&self) -> Vec<u8> {
    vec![self.0]
  }
}

pub(crate) struct IndirectIndexed(pub u8);
impl AddressMode for IndirectIndexed {
  const LENGTH: u16 = 1;

  fn read(&self, memory: &Memory, registers: &Registers) -> i8 {
    memory.indirect_indexed(self.0, &registers.y) as i8
  }
  fn write(&self, memory: &mut Memory, registers: &mut Registers, value: i8) {
    memory.indirect_indexed_write(self.0, &registers.y, value as u8)
  }
}
impl Renderable for IndirectIndexed {
  fn render(&self) -> Vec<u8> {
    vec![self.0]
  }
}
