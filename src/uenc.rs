extern crate url;

use url::percent_encoding::{percent_encode, FORM_URLENCODED_ENCODE_SET};
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let encoded = percent_encode(args[1].as_bytes(),
                                 FORM_URLENCODED_ENCODE_SET);
    println!("{}", encoded);
}
