use cpu::*;
use cpumem::*;
pub mod cpumem;
pub mod cpu;

fn main() {
    let mut m_cpu = Cpu::init();
    for i in 0x00..0xFF {
        let mode: String;
        match m_cpu.exec_op(i) {
            1=>mode = String::from("imm"),
            2=>mode = String::from("zpg"),
            3=>mode = String::from("zpx"),
            4=>mode = String::from("zpy"),
            5=>mode = String::from("rel"),
            6=>mode = String::from("abs"),
            7=>mode = String::from("abx"),
            8=>mode = String::from("aby"),
            9=>mode = String::from("ind"),
            10=>mode = String::from("inx"),
            11=>mode = String::from("iny"),
            _ =>mode = String::from("xxx"),
        }
        print!("{}  ", mode);
        if (i & 0x0F) == 0x0F {
            println!();
            println!();
        }
    }
}
