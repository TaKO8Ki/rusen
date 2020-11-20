use rusen::nes::Nes;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("sample1.nes").expect("no file found");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("buffer overflow");
    let mut nes = Nes::default();
    nes.load(buffer);
    nes.initialize();
    nes.reset();

    nes.render().unwrap();

    for i in 0..175 {
        println!("================ {} ================", i);
        nes.step();
    }
}

pub fn run_cpu(nes: &mut Nes, end: u8) -> Result<(), Box<dyn std::error::Error>> {
    let mut f = File::open("sample1.nes").expect("no file found");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("buffer overflow");
    println!("{:?}", buffer);
    nes.load(buffer);
    nes.initialize();

    for i in 0..end {
        println!("================ {} ================", i);
        nes.step();
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::run_cpu;
    use rusen::cpu::Cpu;
    use rusen::nes::Nes;

    #[test]
    fn test_run_cpu() {
        let mut nes = Nes::default();
        run_cpu(&mut nes, 175).unwrap();
        assert_eq!(
            nes.cpu,
            Cpu {
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
