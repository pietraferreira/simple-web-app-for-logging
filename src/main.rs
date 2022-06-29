use std::fs::File;

fn main() {
    match File::create("foo.txt") {
        Ok(..) => println!("Created file"),
        Err(..) => println!("Failed to create file")
    }
}
