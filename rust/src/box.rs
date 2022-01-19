#![allow(dead_code)]

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

pub fn main() {
    use List::{Cons, Nil};

    let list = Cons(0, Box::new(Nil));
    println!("b: {:?}", list);
}

#[cfg(test)]
mod tests {

    #[test]
    fn main() {
        super::main();

        assert_eq!(0, 0);
    }
}
