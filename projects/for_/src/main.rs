fn main() {
    for i in 1..=10 {
        println!("{}", i);
    }
    let x = 7;

    match x {
        1..=5 | 7..=9 => println!("one through five or"),
        _ => println!("something else"),
    }
}
