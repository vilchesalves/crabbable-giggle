#![allow(dead_code)]

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
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

    #[test]
    fn dereference() {
        struct MyBox<T>(T);

        impl<T> MyBox<T> {
            fn new(value: T) -> MyBox<T> {
                MyBox(value)
            }
        }

        impl<T> std::ops::Deref for MyBox<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        fn hello(name: &str) {
            println!("Hello, {name}!");
        }

        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);

        let string_box = MyBox::new(String::from("I'm a string in a box."));

        hello(&string_box);
    }
}
