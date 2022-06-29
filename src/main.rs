#[macro_use] extern crate nickel;
use nickel::Nickel;

extern crate chrono;
use chrono::{DateTime,Local};

extern crate clap;
use clap::{App,Arg};

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

fn do_log_time() -> String {
    match log_time("log.txt") {
        Ok(..) => format!("Created file"),
        Err(e) => format!("Failed to create file, error: {}", e)
    }
}

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            do_log_time();
        }
    });

    server.listen("localhost:6767");
}
