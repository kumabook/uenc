extern crate url;

use url::percent_encoding::{lossy_utf8_percent_decode};
use std::env;
use std::io::{self};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        let stdin = io::stdin();
        loop {
            let mut buffer = String::new();
            match stdin.read_line(&mut buffer) {
                Ok(len) => {
                    if len > 0 {
                        print!("{}", decode(&buffer))
                    } else {
                        return
                    }
                }
                Err(_) => return,
            }
        }
    }
    println!("{}", decode(&args[1]));
}

fn decode(str: &String) -> String {
    return lossy_utf8_percent_decode(str.as_bytes());
}
