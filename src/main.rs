mod modules;
use modules::front;
fn main() {
    println!("Hello, world!");
    eat_at_restaurant();
}

pub fn eat_at_restaurant() {
    // Absolute path
    modules::front::hosting::add_to_waitlist();

    // Relative path
    modules::front::hosting::add_to_waitlist();

    // Absolute path with renaming
    modules::front::hosting::seat_at_table();

    // Relative path with renaming
    modules::front::serving::take_order();
}
