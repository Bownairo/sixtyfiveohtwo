#[derive(Default)]
pub(crate) struct Registers {
  pub pc: ProgramCounter,
  pub sp: StackPointer,
  pub acc: Accumulator,
  pub x: IndexX,
  pub y: IndexY,
  pub flags: Flags,
}

impl std::fmt::Display for Registers {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "PC:    {}", &self.pc as &dyn Register)?;
    writeln!(f, "SP:    {}", &self.sp as &dyn Register)?;
    writeln!(f, "ACC:   {}", &self.acc as &dyn Register)?;
    writeln!(f, "X:     {}", &self.x as &dyn Register)?;
    writeln!(f, "Y:     {}", &self.y as &dyn Register)?;
    writeln!(f, "Flags: {}", &self.flags as &dyn Register)?;

    Ok(())
  }
}

pub(crate) trait Register {
  fn raw(&self) -> u16;
}

impl std::fmt::Display for dyn Register {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{0:016b} ({0})", self.raw())
  }
}

pub(crate) trait IndexRegister {
  fn read(&self) -> i8;

  fn write(&mut self, value: i8);
}

#[derive(Default)]
pub(crate) struct ProgramCounter {
  pub value: u16,
}
impl Register for ProgramCounter {
  fn raw(&self) -> u16 {
    self.value
  }
}

pub(crate) struct StackPointer {
  pub value: u8,
}
impl Default for StackPointer {
  fn default() -> Self {
    StackPointer { value: 0xFF }
  }
}

impl Register for StackPointer {
  fn raw(&self) -> u16 {
    self.value as u16
  }
}

#[derive(Default)]
pub(crate) struct Accumulator {
  pub value: i8,
}
impl Register for Accumulator {
  fn raw(&self) -> u16 {
    self.value as u16
  }
}

#[derive(Default)]
pub(crate) struct IndexX {
  pub value: i8,
}
impl Register for IndexX {
  fn raw(&self) -> u16 {
    self.value as u16
  }
}
impl IndexRegister for IndexX {
  fn read(&self) -> i8 {
    self.value
  }

  fn write(&mut self, value: i8) {
    self.value = value;
  }
}

#[derive(Default)]
pub(crate) struct IndexY {
  pub value: i8,
}
impl Register for IndexY {
  fn raw(&self) -> u16 {
    self.value as u16
  }
}
impl IndexRegister for IndexY {
  fn read(&self) -> i8 {
    self.value
  }

  fn write(&mut self, value: i8) {
    self.value = value;
  }
}

#[derive(Default)]
pub(crate) struct Flags {
  pub carry: bool,
  pub zero: bool,
  pub interrupt_disable: bool,
  pub decimal_mode: bool,
  pub break_command: bool,
  pub overflow: bool,
  pub negative: bool,
}
impl Register for Flags {
  fn raw(&self) -> u16 {
    let mut output = 0;
    output &= u16::from(self.carry);
    output <<= 1;
    output &= u16::from(self.zero);
    output <<= 1;
    output &= u16::from(self.interrupt_disable);
    output <<= 1;
    output &= u16::from(self.decimal_mode);
    output <<= 1;
    output &= u16::from(self.break_command);
    output <<= 1;
    output &= u16::from(self.overflow);
    output <<= 1;
    output &= u16::from(self.negative);

    output
  }
}
