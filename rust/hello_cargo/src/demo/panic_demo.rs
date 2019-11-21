use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn run() {
    // panic!("crash and burn");

    if let Ok(mut f) = File::open("/home/ysw/workspace/trying/rust/hello_cargo/src/hello.txt") {
        let mut s = String::new();
        if let Ok(_) = f.read_to_string(&mut s) {
            println!("{}", s);
        }
    }

    let f = read1("/home/ysw/workspace/trying/rust/hello_cargo/src/hello2.txt");
    if let Ok(s) = f {
        println!("{}", s);
    }

    let f = read2("/home/ysw/workspace/trying/rust/hello_cargo/src/hello2.txt");
    if let Ok(s) = f {
        println!("{}", s);
    }

    let mut f = read3("/home/ysw/workspace/trying/rust/hello_cargo/src/hello2.txt");
    let mut s = String::new();
    if let Ok(len) = f.read_to_string(&mut s) {
        println!("{}", len);
        println!("{}", s);
    }
}

fn read1(path: &str) -> Result<String, io::Error> {
    let f = File::open(path);
    let mut of = match f {
        Ok(file) => file,
        // Err(err) => match err.kind() {
        //     ErrorKind::NotFound => match fs::write(path, "abc") {
        //         Ok(_) => return read1(path),
        //         Err(e) => return Err(e),
        //     },
        //     _ => return Err(err),
        // },
        Err(err) => return Err(err),
    };
    let mut s = String::new();
    return match of.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    };
}

fn read2(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

fn read3(path: &str) -> File {
    // File::open(path).unwrap()
    let mut msg = String::from("Failed to open ");
    msg.push_str(path);
    File::open(path).expect(msg.as_str())
}
