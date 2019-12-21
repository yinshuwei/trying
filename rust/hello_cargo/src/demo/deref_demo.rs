use std::ops::Deref;
use std::ops::DerefMut;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        println!("no mut");
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        println!("mut");
        &mut self.0
    }
}

fn print(v: &i32) {
    println!("{}", v);
}

fn print_mut(v: &mut i32) {
    println!("{}", v);
}

pub fn run() {
    let x = 5;
    let y = MyBox::new(x);
    print(&y);

    let mut x = 5;
    let mut y = MyBox::new(&mut x);
    print_mut(&mut y);
}
