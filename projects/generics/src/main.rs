pub fn largest0<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    &largest
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest0(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
    let str_list = vec![String::from("yo"), String::from("zazou"), "zozo".to_string(), "toto".to_string()];
    let result = largest(&str_list);
    println!("The largest str is {}", result);
}
