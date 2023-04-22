/*
    A MOS6502 Implementation 
    The NES actually has a Ricoh 2A03 Microprocessor, but it is a slightly different
    version of the 6502. This implementathtion could be used in the various other computers
    that operated with the 6502
*/

use super::cpumem::*;

/*
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
*/

const CARRY_FLAG: u8        = 0b00000001;
const ZERO_FLAG: u8         = 0b00000010;
const INTERRUPT_FLAG: u8    = 0b00000100;
const DECIMAL_FLAG: u8      = 0b00001000;
const OVERFLOW_FLAG: u8     = 0b01000000;
const NEGATIVE_FLAG: u8     = 0b10000000;

pub struct Cpu {
    // registers
    pub a: u8,      // Accumulator
    pub x: u8,      
    pub y: u8,
    pub sp: u8,      // Stack pointer
    pub pc: u16,    // program counter
    pub p: u8,  // Status Register

    // sys memory
    pub mem: CpuMemory,
}

impl Cpu {
   pub fn init() -> Self {
        Cpu {
            a: 0, x: 0, y: 0, sp: 0, pc: 0, p: 0, mem: CpuMemory::init(),
        }
    }

    // invaluable resource:
    // https://www.masswerk.at/6502/6502_instruction_set.html
    pub fn exec_op(&mut self, op: u8) -> u16{
        // let addr: u16 = 0x0000;
        // address modes
        let addr: u16 = match op {
            0x10 | 0x30 | 0x50 | 0x70 | 0x90 | 0xB0 | 0xD0 | 0xF0   => self.rel_addr(),
            0x01 | 0x21 | 0x41 | 0x61 | 0x81 | 0xA1 | 0xC1 | 0xE1   => self.inx_addr(),
            0x11 | 0x31 | 0x51 | 0x71 | 0x91 | 0xB1 | 0xD1 | 0xF1   => self.iny_addr(),
            0x6C                                                    => self.ind_addr(),

            0x05 | 0x25 | 0x45 | 0x65 | 0x85 | 0xA5 | 0xC5 | 0xE5 |
            0x06 | 0x26 | 0x46 | 0x66 | 0x86 | 0xA6 | 0xC6 | 0xE6 |
            0x24 | 0x84 | 0x94 | 0xA4 | 0xB4 | 0xC4 | 0xE4          => self.zpg_addr(),

            0x15 | 0x35 | 0x55 | 0x75 | 0x95 | 0xB5 | 0xD5 | 0xF5 | 
            0x16 | 0x36 | 0x56 | 0x76 | 0xD6 | 0xF6                 => self.zpx_addr(),

            0x96 | 0xB6                                             => self.zpy_addr(),

            0x19 | 0x39 | 0x59 | 0x79 | 0x99 | 0xB9 | 0xD9 | 0xF9 | 
            0xBE                                                    => self.aby_addr(),

            0x1D | 0x3D | 0x5D | 0x7D | 0x9D | 0xBD | 0xDD | 0xFD | 
            0x1E | 0x3E | 0x5E | 0x7E | 0xDE | 0xFE | 0xBC          => self.abx_addr(),

            0x0D | 0x2D | 0x4D | 0x6D | 0x8D | 0xAD | 0xCD | 0xED |
            0x0E | 0x2E | 0x4E | 0x6E | 0x8E | 0xAE | 0xCE | 0xEE |
            0x20                                                    => self.abs_addr(),

            0xA0 | 0xC0 | 0xE0 | 0xA2 | 0x09 | 0x29 | 0x49 | 0x69 | 
            0xA9 | 0xC9 | 0xE9                                      => self.imm_addr(),
 
            _ => 0x0000, // rest of the opcodes have implied operands (none or the accumulator)
        };


        match op {
            0x00 => self.brk(),
            0x01 | 0x05 | 0x09 | 0x0D | 0x11 | 0x15 | 0x19 | 0x1D => self.ora(addr),
            _ => () 
        }
        addr
    }
 

    // addressing modes
    pub fn imm_addr(&self) -> u16{ 1 }
    pub fn zpg_addr(&self) -> u16{ 2 }
    pub fn zpx_addr(&self) -> u16{ 3 }
    pub fn zpy_addr(&self) -> u16{ 4 }
    pub fn rel_addr(&self) -> u16{ 5 }
    pub fn abs_addr(&self) -> u16{ 6 }
    pub fn abx_addr(&self) -> u16{ 7 }
    pub fn aby_addr(&self) -> u16{ 8 }
    pub fn ind_addr(&self) -> u16{ 9 }
    pub fn inx_addr(&self) -> u16{ 10 }
    pub fn iny_addr(&self) -> u16{ 11 }
    
    
    // Opcode Implementations -----
    // transfer instructions
    pub fn lda(&self){}
    pub fn ldx(&self){}
    pub fn ldy(&self){}
    pub fn sta(&self){}
    pub fn stx(&self){}
    pub fn sty(&self){}
    pub fn tax(&self){}
    pub fn tay(&self){}
    pub fn tsx(&self){}
    pub fn txa(&self){}
    pub fn txs(&self){}
    pub fn tya(&self){}

    // stack instructions
    pub fn pha(&self){}
    pub fn php(&self){}
    pub fn pla(&self){}
    pub fn plp(&self){}

    // decrement and increment
    pub fn dec(&self){}
    pub fn dex(&self){}
    pub fn dey(&self){}
    pub fn inc(&self){}
    pub fn inx(&self){}
    pub fn iny(&self){}

    // arithmetic operations
    pub fn adc(&self){}
    pub fn sbc(&self){}

    // logical operators
    pub fn and(&self){}
    pub fn eor(&self){}
    pub fn ora(&self, addr: u16){}

    // shift and rotate
    pub fn asl(&self){}
    pub fn lsr(&self){}
    pub fn rol(&self){}
    pub fn ror(&self){}

    // flags
    // these are all 2 ticks! 
    pub fn clc(&mut self) { self.p &= CARRY_FLAG; }
    pub fn cld(&mut self) { self.p &= DECIMAL_FLAG; }
    pub fn cli(&mut self) { self.p &= INTERRUPT_FLAG; }
    pub fn clv(&mut self) { self.p &= OVERFLOW_FLAG; }
    pub fn sec(&mut self) { self.p |= CARRY_FLAG; }
    pub fn sed(&mut self) { self.p |= DECIMAL_FLAG; }
    pub fn sei(&mut self) { self.p |= INTERRUPT_FLAG; }

    // comparison
    pub fn cmp(&self){}
    pub fn cpx(&self){}
    pub fn cpy(&self){}

    // branching
    pub fn bcc(&self){}
    pub fn bcs(&self){}
    pub fn beq(&self){}
    pub fn bmi(&self){}
    pub fn bne(&self){}
    pub fn bpl(&self){}
    pub fn bvc(&self){}
    pub fn bvs(&self){}

    // jumps and subroutines
    pub fn jmp(&self){}
    pub fn jsr(&self){}
    pub fn rts(&self){}

    // interupts
    pub fn brk(&self){}
    pub fn rti(&self){}

    // misc
    pub fn bit(&self){}
    pub fn nop(&self){}
    // -------------------------
}
