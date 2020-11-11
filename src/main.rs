use rusen::cpu::Cpu;
use std::fs::{self, File};
use std::io::Read;
use std::io::{self, prelude::*};

fn main() {
    let mut f = File::open("sample1.nes").expect("no file found");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("buffer overflow");
    println!("{:?}", buffer);
    let mut cpu = Cpu::default();

    for i in 0..150 {
        println!("================ {} ================", i);
        cpu.exec();
    }
}
