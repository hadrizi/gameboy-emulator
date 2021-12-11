use std::path::Path;

mod cpu;
mod core;
mod debugger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cpu = cpu::CPU::new();
    cpu.memory.load(&Path::new("D:\\utilities\\gb_roms\\games\\Tetris (World) (Rev A).gb"));
    let mut dbg = debugger::Debugger::new(cpu);
    dbg.run()
    // cpu.check_table();
    // while !cpu.stopped {
    //     cpu.clock();
    // }

    // Ok(())
}
