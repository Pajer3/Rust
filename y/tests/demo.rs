#[cfg(test)]
mod tests {
    use y::front_of_house::hosting::{add_to_waitlist, seat_at_table};
    use y::ran;
    use y::front_of_house::serving::{take_order, serve_order, take_payment};
    use y::back_of_house::cooking::{cook_order, fix_incorrect_order};
    use y::back_of_house::management::handle_complaint;

    #[test]
    fn try_front_paths() {
        add_to_waitlist();
        seat_at_table();
        take_order();
        serve_order();
        take_payment();
    }


    #[test]
    fn try_back_paths() {
        cook_order();
        fix_incorrect_order();
        handle_complaint();
    }

    #[test]
    fn try_3() {
        ran();
    }
}