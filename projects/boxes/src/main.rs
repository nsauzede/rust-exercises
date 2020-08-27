fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
fn main() {
    let b = Box::new(0b_1010_0101);
    println!("*b = {:#b} ({})", *b, type_of(&*b));
}
