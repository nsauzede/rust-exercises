fn main() {
    println!("Hello, world!");
    let someval = Some(3u8);
    match someval {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(i) = someval {
        println!("num={}", i);
    }
}
