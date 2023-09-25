mod front_of_house;

// 2.use
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 1.路径
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    // 2.sue
    hosting::add_to_waitlist();
}