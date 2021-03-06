use crate::List::{Cons, Nil};
use mytools;
use std::ops::Deref;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let b = Box::new(0b_1010_0101);
    println!("*b = {:?} ({})", *b, mytools::type_of(&*b));

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("l = {:?} ({})", list, mytools::type_of(&list));

    let mb = MyBox::new(*b);
    println!("*mb = {:?} ({})", *mb, mytools::type_of(&*mb));

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
