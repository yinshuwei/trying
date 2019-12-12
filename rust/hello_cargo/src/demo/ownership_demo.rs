pub fn run(){
    let s1 = String::from("hello");
    let s2 = s1;
    
    // println!("{}, world!", s1);
    println!("{}, world!", s2);


    // 引用
    let s1 = String::from("hello");
    let s2 = &s1;
    
    println!("{}, world!", s1);
    println!("{}, world!", s2);

    // 借用,获取引用作为函数参数称为 借用
    let s1 = String::from("hello");

    // let len = calculate_length_move(s1);
    // println!("len:{}", len);

    let len = calculate_length(&s1);
    println!("len:{}", len);
}

// fn calculate_length_move(s: String) -> usize { // s 所有权移动
//     s.len()
// }

fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
    s.len()
}