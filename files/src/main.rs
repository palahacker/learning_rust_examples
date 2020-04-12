use std::fs::File;
use std::io::prelude::*;

fn main() {
    write_file();
}

fn write_file() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello, world!\n")?;
    file.write_all(b"Hello, world!\n")?;
    file.write_all(b"Hello, world!\n")?;
    file.write_all(b"Hello, world!\n")?;
    Ok(())
}