enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn print(&self) {
        if let List::Cons(i, c) = self {
            println!("{}", i);
            c.print();
        }
    }
}

struct MyList<'a> {
    list: &'a List,
}

impl MyList<'_> {
    fn new<'a>(list: &'a List) -> MyList<'a> {
        MyList {
            list: list,
        }
    }
}

impl Iterator for MyList<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if let List::Cons(i, c) = self.list {
            self.list = c;
            return Some(*i);
        }
        return None;
    }
}

pub fn run() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    list.print();

    println!();

    let my = MyList::new(&list);
    for i in my.into_iter() {
        println!("{}", i);
    }
}
