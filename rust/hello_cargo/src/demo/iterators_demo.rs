struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|s| {
            println!("shoe_size: {}", shoe_size);
            s.size == shoe_size
        })
        .collect()
}

fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    println!();
    let in_my_size = shoes_in_my_size(shoes, 10);
    for s in in_my_size {
        println!("shoes 10: {} {}", s.style, s.size);
    }
}

pub fn run() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!();
    println!("map: {:?} => {:?}", v1, v2);

    let v3 = vec![1, 2, 3, 4, 5, 6];
    let v4 = vec![11, 22, 33, 44, 55, 66];
    let v5: Vec<_> = v3
        .iter()
        .zip(&v4)
        .map(|(a, b)| {
            println!("{} {}", a, b);
            a + b * 100
        })
        .collect();
    println!();
    println!("map: {:?},{:?} => {:?}", v3, v4, v5);

    filters_by_size();
}
