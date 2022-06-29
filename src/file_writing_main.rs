use std::fs::File;
use std::io::Write;
use std::io;

fn log_something(filename: &'static str, string: &'static [u8; 10]) -> io::Result<()>{
    let mut file = File::create(filename).unwrap();
    file.write_all(string).unwrap();
    Ok(())
}

fn main() {
    match log_something("log.txt", b"I'm ALIVE!") {
        Ok(..) => println!("Created file"),
        Err(..) => println!("Failed to create file")
    }
}
