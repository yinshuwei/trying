use std::collections::HashMap;

pub fn run() {
    let mut map = HashMap::new();
    map.insert("A", "1");
    map.insert("B", "2");
    if let Some(v) = map.get(&"B") {
        println!("insert get B: {}", v);
    }

    let keys = vec!["A", "B"];
    let values = vec!["1", "2"];
    let map: HashMap<_, _> = keys.iter().zip(values.iter()).collect();
    if let Some(v) = &map.get(&"B") {
        println!("zip get B: {}", v);
    }

    for (key, value) in &map {
        println!("for in {}: {}", key, value);
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
