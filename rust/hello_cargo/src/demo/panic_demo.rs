use std::fs::File;
use std::io::prelude::*;

pub fn run() {
    // panic!("crash and burn");

    if let Ok(mut f) = File::open("/home/ysw/workspace/trying/rust/hello_cargo/src/hello.txt") {
        let mut s = String::new();
        if let Ok(len) = f.read_to_string(&mut s) {
            println!("{}", len);
            println!("{}", s);
        }
    }
}
