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

    let b = Cons(3, Rc::clone(&a));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("a rc count after c creation = {}", Rc::strong_count(&a));
    }

    println!(
        "a rc count after c goes out of scope = {}",
        Rc::strong_count(&a)
    );
}
