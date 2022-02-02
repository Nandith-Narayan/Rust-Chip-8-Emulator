//use std::time::Instant;

mod cpu;

fn main() {
    let mut cpu = cpu::init();
    cpu.load_rom("C:/test/test-rom.ch8".to_string());
    for _ in 1..5_000{
        cpu.fetch_instruction();
    }
}
