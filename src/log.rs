use std::fs::File;
use std::io::Write;

pub fn info(message: &str) -> std::io::Result<()> {
    let mut f = File::create("rust-log.txt")?;
    f.write_all(message.as_bytes())?;
    println!("{message}");
    Ok(())
}
