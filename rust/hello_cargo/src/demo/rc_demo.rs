use std::rc::Rc;
use List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn run() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count:{}", Rc::strong_count(&a));

    {
        let b = Cons(3, Rc::clone(&a));
        if let Cons(i, _) = b {
            println!("{}", i)
        }
        println!("count:{}", Rc::strong_count(&a));
    }
    println!("count:{}", Rc::strong_count(&a));

    let c = Cons(4, a.clone());
    println!("count:{}", Rc::strong_count(&a));

    drop(c);
    println!("count:{}", Rc::strong_count(&a));
}
