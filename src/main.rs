use std::path::Path;

mod cpu;
mod core;
mod debugger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cpu = cpu::CPU::new();
    cpu.memory.load(&Path::new("D:\\utilities\\gb_roms\\blargg test roms\\cpu_instrs\\cpu_instrs.gb"));
    let mut dbg = debugger::Debugger::new(cpu);
    dbg.run()

    // while !cpu.stopped {
    //     cpu.clock();
    // }

    // Ok(())
}
