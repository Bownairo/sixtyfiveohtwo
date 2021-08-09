use std::convert::TryFrom;

use crate::registers::{IndexRegister, IndexX, IndexY};

// TODO: Stack operations

pub(crate) struct Memory {
  pub inner: [u8; 64000],
}

impl Memory {
  pub(crate) fn new() -> Self {
    Memory { inner: [0; 64000] }
  }

  pub(crate) fn zero_page(&self, addr: u8) -> u8 {
    let checked = usize::from(addr);
    self.inner[checked]
  }

  pub(crate) fn zero_page_write(&mut self, addr: u8, value: u8) {
    let checked = usize::from(addr);
    self.inner[checked] = value;
  }

  pub(crate) fn zero_page_register<T: IndexRegister>(
    &self,
    addr: u8,
    register: &T,
  ) -> u8 {
    let mut checked =
      usize::try_from(i8::try_from(addr).unwrap() + register.read()).unwrap();
    if checked > 0xFF {
      checked -= 0xFF;
    }
    self.inner[checked]
  }

  pub(crate) fn zero_page_register_write<T: IndexRegister>(
    &mut self,
    addr: u8,
    register: &T,
    value: u8,
  ) {
    let mut checked =
      usize::try_from(i8::try_from(addr).unwrap() + register.read()).unwrap();
    if checked > 0xFF {
      checked -= 0xFF;
    }
    self.inner[checked] = value;
  }

  pub(crate) fn absolute(&self, addr: u16) -> u8 {
    let checked = usize::from(addr);
    self.inner[checked]
  }

  pub(crate) fn absolute_write(&mut self, addr: u16, value: u8) {
    let checked = usize::from(addr);
    self.inner[checked] = value;
  }

  pub(crate) fn absolute_register<T: IndexRegister>(
    &self,
    addr: u16,
    register: &T,
  ) -> u8 {
    let checked =
      usize::try_from(addr + u16::try_from(register.read()).unwrap()).unwrap();
    self.inner[checked]
  }

  pub(crate) fn absolute_register_write<T: IndexRegister>(
    &mut self,
    addr: u16,
    register: &T,
    value: u8,
  ) {
    let checked =
      usize::try_from(addr + u16::try_from(register.read()).unwrap()).unwrap();
    self.inner[checked] = value;
  }

  #[allow(dead_code)]
  // NOTE: u16 because read jump location from memory
  pub(crate) fn indirect(&self, addr: u16) -> u16 {
    let checked_first = usize::from(addr);
    let first = self.inner[checked_first];

    let mut checked_second = usize::from(addr + 1);
    if (checked_first % 1000) != (checked_second % 1000) {
      checked_second -= 1000;
    }
    let second = self.inner[checked_second];

    u16::from_le_bytes([first, second])
  }

  pub(crate) fn indexed_indirect(&self, addr: u8, register: &IndexX) -> u8 {
    let mut checked =
      usize::try_from(i8::try_from(addr).unwrap() + register.read()).unwrap();
    if checked > 0xFF {
      checked -= 0xFF;
    }
    let first = self.inner[checked];

    let mut checked =
      usize::try_from(i8::try_from(addr).unwrap() + register.read() + 1)
        .unwrap();
    if checked > 0xFF {
      checked -= 0xFF;
    }
    let second = self.inner[checked];

    let checked = usize::from(u16::from_le_bytes([first, second]));
    self.inner[checked]
  }

  pub(crate) fn indexed_indirect_write(
    &mut self,
    addr: u8,
    register: &IndexX,
    value: u8,
  ) {
    let mut checked =
      usize::try_from(i8::try_from(addr).unwrap() + register.read()).unwrap();
    if checked > 0xFF {
      checked -= 0xFF;
    }
    let first = self.inner[checked];

    let mut checked =
      usize::try_from(i8::try_from(addr).unwrap() + register.read() + 1)
        .unwrap();
    if checked > 0xFF {
      checked -= 0xFF;
    }
    let second = self.inner[checked];

    let checked = usize::from(u16::from_le_bytes([first, second]));
    self.inner[checked] = value;
  }

  pub(crate) fn indirect_indexed(&self, addr: u8, register: &IndexY) -> u8 {
    let checked = usize::from(addr);
    let first = self.inner[checked];

    let checked = usize::from(addr + 1);
    let second = self.inner[checked];

    let checked = usize::from(
      u16::from_le_bytes([first, second])
        + u16::try_from(register.read()).unwrap(),
    );
    self.inner[checked]
  }

  pub(crate) fn indirect_indexed_write(
    &mut self,
    addr: u8,
    register: &IndexY,
    value: u8,
  ) {
    let checked = usize::from(addr);
    let first = self.inner[checked];

    let checked = usize::from(addr + 1);
    let second = self.inner[checked];

    let checked = usize::from(
      u16::from_le_bytes([first, second])
        + u16::try_from(register.read()).unwrap(),
    );
    self.inner[checked] = value;
  }
}
