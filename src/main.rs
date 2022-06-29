extern crate chrono;
use chrono::*;
use std::fs::File;
use std::io::Write;
use std::io;

fn log_time(filename: &'static str) -> io::Result<()>{
    let local: DateTime<Local> = Local::now();
    let formatted = local.format("%a, %d %b %Y %I:%M:%S %p\n").to_string();
    let bytes = formatted.as_bytes();
    let mut file = File::create(filename).unwrap();
    file.write_all(bytes).unwrap();
    Ok(())
}

fn main() {
    let local: DateTime<Local> = Local::now();
    println!("{}", local);
    match log_time("log.txt") {
        Ok(..) => println!("Created file"),
        Err(..) => println!("Failed to create file")
    }
}
