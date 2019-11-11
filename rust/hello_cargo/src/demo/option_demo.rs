pub fn run() {
    let a: u32 = 31;
    // let b: Option<u32> = None;
    let mut b = Some(a);
    match b {
        None => {
            println!("None");
        }
        Some(i) => {
            println!("{}", i);
        }
    }
    b = None;
    if let Some(i) = b {
        println!("{}", i);
    } else {
        println!("None");
    }
}
