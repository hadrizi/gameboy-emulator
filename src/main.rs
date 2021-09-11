mod cpu;
mod core;

fn main() {
    let r = core::register::Register::new(0xB668);
    println!("{:X?}\n", r);

    let f = core::flags::Flags::new();
    println!("{:?}\n", f);

    let mut cpu = cpu::CPU::new();
    println!("{:#X?}\n", cpu);

    cpu.memory.load(vec![0xFF, 0xF1, 0x10, 0x0A, 0x13]);

    loop {
        cpu.clock();
    }
}
