pub fn run() {
    let mut hello = String::from("AAA");

    hello.push_str("_AA_");
    hello.push('A');
    hello.push('_');
    hello.push('中');
    // let word = String::new() + &hello + &String::from("_好好好");
    let word = String::from(&hello) + &String::from("_好好好");
    // let word = format!("{}{}", hello, "_好好好");
    println!("{}", hello);
    println!("{}", word);
}
