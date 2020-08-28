//! # My tools
//!
//! `mytools` is a collection of useful Rust tools.

/// Get type name of provided referenced Rust variable.
///
/// It can be useful to print debug information about a convoluted variable type.
/// Eg: the following code should print "&alloc::boxed::Box<(alloc::vec::Vec<f32>, alloc::string::String)>"
/// ```
/// let convoluted = &Box::new((vec![123f32],3.14.to_string()));
/// let type_name = mytools::type_of(&convoluted);
/// println!("{}", type_name);
/// assert_eq!("&alloc::boxed::Box<(alloc::vec::Vec<f32>, alloc::string::String)>", type_name);
/// ```
///
/// # Examples
/// Examples below ensure that the type name got is as expected.
/// ```
/// let forty_two = 42; // i32
/// let pi = 3.14;      // f64
/// assert_eq!("i32", mytools::type_of(&forty_two));
/// assert_eq!("f64", mytools::type_of(&pi));
/// ```
/// ```
/// let boxed_notb = Box::new(666);
/// assert_eq!("alloc::boxed::Box<i32>", mytools::type_of(&boxed_notb));
/// ```
/// ```
/// let bigint = 1i64 << 33;
/// assert_eq!("i64", mytools::type_of(&bigint));
/// ```
/// ```
/// let var = "string litteral";
/// assert_eq!("&str", mytools::type_of(&var));
/// ```
pub fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
