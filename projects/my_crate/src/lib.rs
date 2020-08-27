//! # My Crate
//!
//! `my_crate is a collection of blah..

pub struct SomeStruct {}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
///
/// # Panics
/// Some funcs do panic, blah..
/// ```
/// #[test]
/// #[should_panic]
/// panic!("w00t");
/// println!("w00t");
/// ```
///
/// # Errors
/// Here are the various errors returned, blah..
///
/// # Safety
/// Some funcs are unsafe, blah..
pub fn add_one(x: i32) -> i32 {
    x + 1
}
