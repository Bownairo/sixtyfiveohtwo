mod instructions;
mod memory;
mod registers;

use instructions::addressing::*;
use instructions::{Instruction, ADC, ASL, INY, LDA, LDX, LDY, STX, TAX};

fn main() {
  let mut memory = memory::Memory::new();
  let mut registers = registers::Registers::default();

  evaluate!(
    &mut memory,
    &mut registers,
    LDA(Immediate(0)),  // LDA #$0
    ADC(Immediate(5)),  // ADC #$5
    ASL(Accumulator),   // ASL A
    TAX,                // TAX
    STX(Absolute(100)), // STX $100
    LDX(Immediate(1)),  // LDX #$1
    // LDY(AbsoluteY(99)), // LDY $99,Y (This results in a compilation error,
    //                     //            as this is an invalid instruction.)
    LDY(AbsoluteX(99)), // LDY $99,X
    INY                 // INY
  );

  println!("{}", registers);
}
