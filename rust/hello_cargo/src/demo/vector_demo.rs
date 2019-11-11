pub fn run() {
    let v = vec![1, 2];

    let second: &i32 = &v[1];
    println!("The second element is {}", second);
    let o = v.get(2);
    match o {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];

    v.push(6);

    if let Some(first) = v.get(1) {
        println!("The first element is: {}", first);
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }
}
