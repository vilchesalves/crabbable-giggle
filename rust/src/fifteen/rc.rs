#![allow(dead_code)]
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[test]
fn rc() {
    use List::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Nil)));
    println!("a initial rc count = {}", Rc::strong_count(&a));

    let _b = Cons(3, Rc::clone(&a));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));

    {
        let _c = Cons(4, Rc::clone(&a));
        println!("a rc count after c creation = {}", Rc::strong_count(&a));
    }

    println!(
        "a rc count after c goes out of scope = {}",
        Rc::strong_count(&a)
    );
}

#[test]
fn mutable() {
    use std::cell::RefCell;

    #[derive(Debug)]
    enum Lista {
        Cons(Rc<RefCell<i32>>, Rc<Lista>),
        Nil,
    }

    use Lista::{Cons, Nil};

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
