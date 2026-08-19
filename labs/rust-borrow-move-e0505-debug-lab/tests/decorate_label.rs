use rust_borrow_move_e0505_debug_lab::decorate_label;

#[test]
fn combines_the_prefix_and_full_label() {
    assert_eq!(decorate_label(), "daily: daily-report");
}
