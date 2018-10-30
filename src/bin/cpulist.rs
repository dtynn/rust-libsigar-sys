extern crate libsigar_sys;

use libsigar_sys::cpu;

fn main() {
    let cpulist = cpu::list().unwrap();

    println!("{:?}", cpulist)
}
