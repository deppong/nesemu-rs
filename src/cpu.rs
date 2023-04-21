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
    pub fn exec_op(&mut self) {
        let op: u8 = 0x00;

        match op {
            0x00 => self.brk(),
            0x01 => self.ora(),
            0x05 => self.ora(),
            0x06 => self.asl(),
            0x08 => self.php(),
            0x09 => self.ora(),
            0x0D => self.ora(),
            0x0E => self.asl(),
            0x10 => self.bpl(),
            0x11 => self.ora(),
            0x15 => self.ora(),
            0x16 => self.asl(),
            0x18 => self.clc(),
            0x19 => self.ora(),
            0x1D => self.ora(),
            0x1E => self.asl(),
            0x20 => self.jsr(),
            0x21 => self.and(),
            0x24 => self.bit(),
            0x25 => self.and(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            0x26 => self.rol(),
            _ => ()
        }
    }


    // addressing modes
    pub fn imm_addr(&self) /*-> u16*/{}
    pub fn zp_addr(&self)  /*-> u16*/{}
    pub fn zpx_addr(&self) /*-> u16*/{}
    pub fn zpy_addr(&self) /*-> u16*/{}
    pub fn rel_addr(&self) /*-> u16*/{}
    pub fn abs_addr(&self) /*-> u16*/{}
    pub fn abx_addr(&self) /*-> u16*/{}
    pub fn aby_addr(&self) /*-> u16*/{}
    pub fn ind_addr(&self) /*-> u16*/{}
    pub fn inx_addr(&self) /*-> u16*/{}
    pub fn iny_addr(&self) /*-> u16*/{}
    
    
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
    pub fn ora(&self){}

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
