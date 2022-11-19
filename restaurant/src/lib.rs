mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 绝对路径
    hosting::add_to_waitlist();

    // 相对路径
    hosting::add_to_waitlist();
}
