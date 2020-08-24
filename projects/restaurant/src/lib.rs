mod front_of_house;
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
