/*
    A MOS6502 Implementation 
    The NES actually has a Ricoh 2A03 Microprocessor, but it is a slightly different
    version of the 6502. This implementathtion could be used in the various other computers
    that operated with the 6502
*/

enum StatusFlag {
    const carry     = 0b00000001;
    const zero      = 0b00000010;
    const interrupt = 0b00000100;
    const decimal   = 0b00001000; // present but unsupported on nes
    // bits 4 and 5 are   0b00010000
    // unused on 6502     0b00100000
    const overflow  = 0b01000000;
    const negative  = 0b10000000;
}

pub struct cpu {
    // registers
    pub a: u8,      // Accumulator
    pub x: u8,      
    pub y: u8,
    pub sp: u8      // Stack pointer
    pub pc: u16,    // program counter
    pub p: u8,  // Status Register
}

impl cpu {
    fn Init() -> Self {
        cpu {
        a: 0, x: 0, y: 0, sp: 0, pc: 0, p: 0
        }
    }
}
