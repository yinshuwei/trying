#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn sq(width: u32) -> Rectangle {
        Rectangle {
            width: width,
            height: width,
        }
    }

    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width: width,
            height: height,
        }
    }
}

pub fn run() {
    let rect1 = Rectangle::sq(20);
    let rect2 = Rectangle::create(15, 16);
    println!("rect1 is {}, {}", rect1.area(), rect1.hold(&rect2));
}
