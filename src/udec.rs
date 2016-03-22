extern crate url;

use url::percent_encoding::{lossy_utf8_percent_decode};
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let decoded = lossy_utf8_percent_decode(args[1].as_bytes());
    println!("{}", decoded);
}
