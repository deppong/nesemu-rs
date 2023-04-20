/*
    Memory Map for the CPU
    $0000 - $07FF    -> 2KB RAM
    $2000 - $2007    -> PPU registers
    $4000 - $4017    -> IO registers
    $4020 - $FFFF    -> Cart memory
*/

pub struct memory {
    pub ram: [u8; 2048],
    pub rom: [u8; 49120],
}

impl memory {
    pub fn write(&self, addr: u16, val: u8) {
        if addr >= 0x0000 && addr <= 0x07FF {
            self.ram[addr] = val;
        }

    }

    pub fn read(&self, addr: u16) -> u8 {
        if addr >= 0x0000 && addr <= 0x07FF {
            self.ram[addr]
        }
        if addr >= 0x4020 && addr <= 0xFFFF {
            self.rom[addr]
        }
    }
}
