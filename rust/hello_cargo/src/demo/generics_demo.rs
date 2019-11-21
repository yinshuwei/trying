struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<u32> {
    // fn xx(&self) -> &u32 {
    //     &self.x
    // }
}

pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];
    let lg = largest(&number_list);
    println!("The largest number is {}", lg);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let lg = largest(&number_list);
    println!("The largest number is {}", lg);

    let number_list = vec!['A', 'C', 'a', 'z', 'D', 'U'];
    let lg = largest(&number_list);
    println!("The largest number is {}", lg);

    let p = Point {
        x: 5 as i32,
        y: 5 as i32,
    };
    println!("p.x = {}, p.y = {}", p.x(), p.y());
}

// no copy
fn largest<T: PartialOrd>(vs: &[T]) -> &T {
    let mut lg = &vs[0];
    for v in vs {
        if lg < v {
            lg = v;
        }
    }
    lg
}
