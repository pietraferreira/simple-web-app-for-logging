extern crate chrono;
use chrono::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::io;

fn formatted_time_entry() -> String {
    let local: DateTime<Local> = Local::now();
    let formatted = local.format("%a, %d %b %Y %I:%M:%S %p\n").to_string();
    formatted
}

fn record_entry_in_log(filename: &str, bytes: &[u8]) -> io::Result<()> {
    let mut file = (OpenOptions::new().
                   append(true). // append if file exists, create otherwise
                   write(true).
                   create(true).
                   open(filename))?;
    file.write_all(bytes).unwrap();
    Ok(())
}

fn log_time(filename: &'static str) -> io::Result<()>{
    let entry = formatted_time_entry();
    let bytes = entry.as_bytes();

    record_entry_in_log(filename, &bytes)?;
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
