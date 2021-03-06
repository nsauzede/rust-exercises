use hello_macro::HelloMacro;
use hello_macro_derive::route;
use hello_macro_derive::sql;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[route(GET, "/")]
fn hello() {
    println!("hello");
}

fn main() {
    Pancakes::hello_macro();
    hello();
    println!("sql={:#?}", sql!(SELECT * FROM emp WHERE id=1));
}
