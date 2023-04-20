use cpu::*;
pub mod cpu;

fn main() {
    let m_cpu = cpu::Init();
    println!("{}", m_cpu.x);
}
