use rust_temporary_borrow_e0716_debug_lab::selected_length;

#[test]
fn returns_the_length_of_the_first_segment() {
    assert_eq!(selected_length(), 4);
}
