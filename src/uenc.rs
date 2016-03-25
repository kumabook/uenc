extern crate url;

use url::percent_encoding::{percent_encode, FORM_URLENCODED_ENCODE_SET};
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
                        println!("{}", encode(&buffer[..len-1]))
                    } else {
                        return
                    }
                },
                Err(_) => return,
            }
        }
    }
    println!("{}", encode(&args[1]));
}

fn encode(str: &str) -> String {
    return percent_encode(str.as_bytes(),
                          FORM_URLENCODED_ENCODE_SET);
}
