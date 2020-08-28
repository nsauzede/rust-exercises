use crate::List::{Cons, Nil};
use mytools;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let co = Cons(1, Box::new(Nil));
    println!("co={:?} ({})", co, mytools::type_of(&co));
    //let b = Cons(3, Box::new(a));
    //let c = Cons(4, Box::new(a));
}
