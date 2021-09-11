mod cpu;
mod core;

fn main() {
    let r = core::register::Register::new(0xB668);
    println!("{:X?}\n", r);

    let mut f = core::flags::Flags::new();
    println!("start\t {:?}", f);
    f.set_z();
    f.set_n();
    f.set_c();
    f.set_h();
    println!("set\t {:?}", f);
    f.set_z();
    f.set_n();
    f.set_c();
    f.set_h();
    println!("reset\t {:?}", f);
}
