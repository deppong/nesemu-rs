/*
    Memory Map for the CPU
    $0000 - $07FF    -> 2KB RAM
    $2000 - $2007    -> PPU registers
    $4000 - $4017    -> IO registers
    $4020 - $FFFF    -> Cart memory
*/

pub struct CpuMemory {
    pub ram: [u8; 2048],
    pub rom: [u8; 49120],
}

impl CpuMemory {
    pub fn init() -> Self {
        CpuMemory {ram: [0x0000; 2048], rom: [0x000; 49120]}
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        if addr >= 0x0000 && addr <= 0x4017 {
            self.ram[addr as usize] = val;
        }

    }

    pub fn read(&self, addr: u16) -> u8 {
        if addr >= 0x0000 && addr <= 0x4017 {
            return self.ram[addr as usize]
        }
        if addr >= 0x4020 && addr <= 0xFFFF {
            return self.rom[addr as usize]
        }

        panic!("ADDRESS NOT FOUND {}", addr);
    }
}
