extern crate url;

use url::percent_encoding::{percent_encode, FORM_URLENCODED_ENCODE_SET};
use std::env;
use std::io::{self, Read};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        let mut buffer = String::new();
        match io::stdin().read_to_string(&mut buffer) {
            Ok(_)  => println!("{}", encode(&buffer)),
            Err(_) => println!(""),
        }
        return;
    }
    println!("{}", encode(&args[1]));
}

fn encode(str: &String) -> String {
    return percent_encode(str.as_bytes(),
                          FORM_URLENCODED_ENCODE_SET);
}
