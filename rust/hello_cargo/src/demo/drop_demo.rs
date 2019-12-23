struct CustomeSmartPointer {
    data: String,
}

impl Drop for CustomeSmartPointer {
    fn drop(&mut self) {
        println!("drop:{}", self.data);
    }
}

pub fn run() {
    let b = CustomeSmartPointer { data: String::from("b") };
    println!("run:{}", b.data);
    let c = CustomeSmartPointer { data: String::from("c") };
    drop(c);
    println!("run");
}
