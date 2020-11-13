use rusen::cpu::Cpu;
use std::fs::{self, File};
use std::io::Read;
use std::io::{self, prelude::*};

fn main() {
    let mut f = File::open("sample1.nes").expect("no file found");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("buffer overflow");
    let mut cpu = Cpu::default();
    cpu.load(buffer);
    cpu.initialize();
    cpu.reset();

    for i in 0..50 {
        println!("================ {} ================", i);
        cpu.step();
    }
}

pub fn run_cpu(cpu: &mut Cpu, end: u8) -> Result<(), Box<dyn std::error::Error>> {
    let mut f = File::open("sample1.nes").expect("no file found");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("buffer overflow");
    println!("{:?}", buffer);
    cpu.load(buffer);
    cpu.initialize();

    for i in 0..end {
        println!("================ {} ================", i);
        cpu.step();
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::run_cpu;
    use rusen::cpu::{Cpu, Register};

    #[test]
    fn test_run_cpu() {
        let mut cpu = Cpu::default();
        run_cpu(&mut cpu, 175).unwrap();
        assert_eq!(
            cpu.register,
            Register {
                a: 0x1e,
                x: 0x0d,
                y: 0x00,
                s: 0x01ff,
                p: 0x34,
                pc: 0x804e,
            }
        )
    }
}
