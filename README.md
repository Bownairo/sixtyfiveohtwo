# sixtyfiveohtwo
**A 6502 Assembly emulator, soon to be part of an NES emulator.**

[![GitHub](https://img.shields.io/github/license/bownairo/sixtyfiveohtwo)](https://github.com/Bownairo/sixtyfiveohtwo/blob/master/LICENSE)

Instructions can be written out in Rust code, checked by the type system.

## Example
```
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
```

## TODO
- [x] Memory Access
- [x] Registers
- [x] Basic Instructions
- [ ] Stack Instructions
- [x] Branch Instructions
- [ ] Flags
- [ ] Binary Mode
