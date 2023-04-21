use cpu::*;
use cpumem::*;
pub mod cpumem;
pub mod cpu;

fn main() {
    let m_cpu = Cpu::init();
    println!("{}", m_cpu.x);
}
