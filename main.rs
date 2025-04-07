use std::fs::File;
use std::io::{self, Write, Read};

fn main() -> io::Result<()> {
    let mut file = File::create("notes.txt")?;
    file.write_all(b"Learning Rust is fun!\n")?;
    println!("✅ Wrote to file.");

    let mut file = File::open("notes.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("📄 File Contents:\n{}", contents);

    Ok(())
}
