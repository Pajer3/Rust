use y;

#[test]
fn try_out_paths() {
    y::front_of_house::hosting::add_to_waitlist();
    y::front_of_house::hosting::seat_at_table();
    y::front_of_house::serving::take_order();
    y::front_of_house::serving::serve_order();
    y::front_of_house::serving::take_payment();
}