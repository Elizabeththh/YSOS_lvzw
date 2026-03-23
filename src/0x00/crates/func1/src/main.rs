mod utils;
use std::io;

use utils::{count_down, file_size, read_and_print};

fn main() {
    count_down(5);
    if let Err(_) = read_and_print("/etc/hosts") {
        println!("File not found!");
    }

    loop {
        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            println!("{}", e.to_string());
        }

        let path = input.trim();

        if path == "q" {
            break;
        }
        match file_size(path) {
            Err(e) => println!("{}", e),
            Ok(sz) => println!("The size of the file {} is {}", path, sz),
        }
    }
}
