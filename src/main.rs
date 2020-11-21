use rusen::nes::Nes;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let file_path = &args[1];
    let file_stem = Path::new(file_path).file_stem().unwrap().to_str().unwrap();

    let mut f = File::open(file_path)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    let mut nes = Nes::default();
    nes.load(buffer);
    nes.initialize();
    nes.reset();

    for _ in 0..180 {
        nes.step();
    }

    nes.run(file_stem)?;
    Ok(())
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
