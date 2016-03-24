extern crate url;

use url::percent_encoding::{lossy_utf8_percent_decode};
use std::env;
use std::io::{self, Read};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        let mut buffer = String::new();
        match io::stdin().read_to_string(&mut buffer) {
            Ok(_)  => println!("{}", decode(&buffer)),
            Err(_) => println!(""),
        }
        return;
    }
    println!("{}", decode(&args[1]));
}

fn decode(str: &String) -> String {
    return lossy_utf8_percent_decode(str.as_bytes());
}
