extern crate url;
extern crate getopts;

use url::percent_encoding::{percent_encode, FORM_URLENCODED_ENCODE_SET};
use std::env;
use std::io::{self};
use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [string] [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("n", "newline", "keep newline");
    opts.optflag("h", "help"   , "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok (m) => { m }
        Err(_) => { print_usage(&program, opts); return; }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if !matches.free.is_empty() {
        println!("{}", encode(&matches.free[0]));
        return;
    };

    let keep_newline = matches.opt_present("n");
    let stdin        = io::stdin();

    loop {
        let mut buffer = String::new();
        match stdin.read_line(&mut buffer) {
            Ok(len) => {
                if len > 0 {
                    let last = buffer.chars().skip(len-1).next().unwrap();
                    if last == '\n' {
                        if keep_newline {
                            println!("{}", encode(&buffer[..len-1]))
                        } else {
                            print!("{}", encode(&buffer[..len]))
                        }
                    } else {
                        print!("{}", encode(&buffer))
                    }
                } else {
                    return
                }
            },
            Err(_) => return,
        }
    }
}

fn encode(str: &str) -> String {
    return percent_encode(str.as_bytes(),
                          FORM_URLENCODED_ENCODE_SET);
}
