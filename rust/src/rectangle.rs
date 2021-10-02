#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        let Rectangle {
            width: w,
            height: h,
        } = self;

        w * h
    }

    pub fn can_hold(&self, r: &Rectangle) -> bool {
        self.width >= r.width && self.height >= r.height
    }
}

mod rectangle {
    use super::Rectangle;

    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    pub fn square(side: u32) -> Rectangle {
        Rectangle {
            width: side,
            height: side,
        }
    }
}

pub use rectangle::new;

pub fn main() {
    let width = 10;
    let height = 20;
    let r1 = rectangle::new(width, height);
    let r2 = rectangle::new(10, 5);
    let r3 = rectangle::square(25);

    println!("rectangle area is {}", r1.area());
    println!("rectangle is {:?}", r1);
    println!("can r1 hold h2? {}", r1.can_hold(&r2));
    println!("can r1 hold h3? {}", r1.can_hold(&r3));
    println!("can r2 hold h3? {}", r2.can_hold(&r3));
    println!("can r3 hold h2? {}", r3.can_hold(&r2));
}
