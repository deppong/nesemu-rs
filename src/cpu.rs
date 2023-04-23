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

const STACK_HI_ADDR: u16 = 0x0100;

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
            a: 0, x: 0, y: 0, sp: 0x00, pc: 0, p: 0, mem: CpuMemory::init(),
        }
    }

    // invaluable resource:
    // https://www.masswerk.at/6502/6502_instruction_set.html
    pub fn exec_op(&mut self, op: u8) -> u16{

        let addr = self.get_addr_mode(op);

        match op {
            0x00 => self.brk(),
            0x01 | 0x05 | 0x09 | 0x0D | 0x11 | 0x15 | 0x19 | 0x1D => self.ora(addr as u8),
            _ => () 
        }
        addr
    }

    fn get_addr_mode(&mut self, op: u8) -> u16 {
        let addr = match op {
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
       addr 
    }

    // addressing modes
    pub fn imm_addr(&mut self) -> u16{ self.pc += 1; return self.mem.read(self.pc) as u16; }
    pub fn zpg_addr(&mut self) -> u16{ self.pc += 1; return self.mem.read(self.pc) as u16; }
    pub fn zpx_addr(&mut self) -> u16{ self.pc += 1; return self.mem.read(self.pc + self.x as u16) as u16; }
    pub fn zpy_addr(&mut self) -> u16{ self.pc += 1; return self.mem.read(self.pc + self.y as u16) as u16;  }
    pub fn rel_addr(&mut self) -> u16{ self.pc += 2; return self.mem.read(self.pc-1) as u16 + self.pc; }
    pub fn abs_addr(&mut self) -> u16 { 
        self.pc+=2;
        // pull the data at the little-endian address 
        let absolute_address: u16 = (self.mem.read(self.pc) << 4) as u16 & self.mem.read(self.pc-1) as u16;
        return self.mem.read(absolute_address) as u16;
    }
    pub fn abx_addr(&mut self) -> u16 { 
        self.pc+=2;
        let absolute_address: u16 = (self.mem.read(self.pc) << 4) as u16 & self.mem.read(self.pc-1) as u16;
        return self.mem.read(absolute_address + self.x as u16) as u16;
    }
    pub fn aby_addr(&mut self) -> u16 {
        self.pc+=2;
        let absolute_address: u16 = (self.mem.read(self.pc) << 4) as u16 & self.mem.read(self.pc-1) as u16;
        return self.mem.read(absolute_address + self.y as u16) as u16;
    }
    pub fn ind_addr(&mut self) -> u16 { 
        9
    }
    pub fn inx_addr(&mut self) -> u16 { 10 }
    pub fn iny_addr(&mut self) -> u16 { 11 }
    
    
    // Opcode Implementations -----
    // transfer instructions
    pub fn lda(&mut self, data: u8) { self.a = data; /*update status flag N, and Z */ }
    pub fn ldx(&mut self, data: u8) { self.x = data; /*update status flag N, and Z */ }
    pub fn ldy(&mut self, data: u8) { self.y = data; /*update status flag N, and Z */ }
    pub fn sta(&mut self, data: u16) { self.mem.write(data, self.a); }
    pub fn stx(&mut self, data: u16) { self.mem.write(data, self.x); }
    pub fn sty(&mut self, data: u16) { self.mem.write(data, self.y); }
    pub fn tax(&mut self) { self.x = self.a; }      // update NZ
    pub fn tay(&mut self) { self.y = self.a; }      // ""
    pub fn tsx(&mut self) { self.sp = self.a; }     // ""
    pub fn txa(&mut self) { self.a = self.x; }      // ""
    pub fn txs(&mut self) { self.sp = self.x; }     // nah
    pub fn tya(&mut self) { self.y = self.x; }      // ""

    // stack instructions
    pub fn pha(&mut self){ self.mem.write(STACK_HI_ADDR & self.sp as u16, self.a); self.sp+=1;}
    pub fn php(&mut self){ self.mem.write(STACK_HI_ADDR & self.sp as u16, self.p); self.sp+=1;} 
    pub fn pla(&mut self){ self.a = self.mem.read(STACK_HI_ADDR & self.sp as u16); self.sp-=1;}
    pub fn plp(&mut self){ self.a = self.mem.read(STACK_HI_ADDR & self.sp as u16); self.sp-=1;}


    // decrement and increment
    pub fn dec(&mut self){ self.a -= 1; }
    pub fn dex(&mut self){ self.x -= 1; }
    pub fn dey(&mut self){ self.y -= 1; }
    pub fn inc(&mut self){ self.a += 1; }
    pub fn inx(&mut self){ self.x += 1; }
    pub fn iny(&mut self){ self.y += 1; }

    // arithmetic operations
    pub fn adc(&mut self){} // NZCV
    pub fn sbc(&mut self){} // NZCV

    // logical operators
    pub fn and(&mut self, addr: u8) { self.a &= addr; } // NZ
    pub fn eor(&mut self, addr: u8) { self.a ^= addr; } // NZ
    pub fn ora(&mut self, addr: u8) { self.a |= addr; } // NZ

    // shift and rotate
    pub fn asl(&mut self){}
    pub fn lsr(&mut self){}
    pub fn rol(&mut self){}
    pub fn ror(&mut self){}

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
    pub fn cmp(&mut self){}
    pub fn cpx(&mut self){}
    pub fn cpy(&mut self){}

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
