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

fn log_time(filename: &String) -> io::Result<(String)>{
    let entry = formatted_time_entry();
    {
        let bytes = entry.as_bytes();

        record_entry_in_log(filename, &bytes)?;
    }
    Ok(entry)
}

fn do_log_time(logfile_path: &String, auth_token: &Option<String>) -> String {
    match log_time(logfile_path) {
        Ok(entry) => format!("Created file, entry logged: {}", entry),
        Err(e) => format!("Failed to create file, error: {}", e)
    }
}

fn main() {
    let matches = App::new("simple-log-web-app").version("v0.0.1")
        .arg(Arg::with_name("LOG FILE")
             .short('l')
             .long("logfile")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("AUTH TOKEN")
             .short('t')
             .long("token")
             .takes_value(true))
        .get_matches();

    let logfile_path = matches.value_of("LOG FILE").unwrap().to_string();
    let auth_token = match matches.value_of("AUTH TOKEN") {
        Some(str) => Some(str.to_string()),
        None => None
    };

    let mut server = Nickel::new();
    server.utilize(router! {
        get "**" => |_req, _res| {
            do_log_time(&logfile_path, &auth_token);
        }
    });

    server.listen("localhost:6767");
}
