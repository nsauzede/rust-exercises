use std::time::{Duration, SystemTime};
fn main() {
    let a = [1, 2, 3, 4, 5];
    let now = SystemTime::now();
    let mut index = 6;
    if now.elapsed().unwrap() > Duration::from_secs(0) {
        index = 10;
//    } else {
//        index = 6;
    }

    let element = a[index];

    println!("The value of element is: {}", element);
}
