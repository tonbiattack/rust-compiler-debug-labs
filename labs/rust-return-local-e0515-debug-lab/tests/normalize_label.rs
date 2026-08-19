use rust_return_local_e0515_debug_lab::normalize_label;

#[test]
fn returns_an_owned_normalized_label() {
    assert_eq!(normalize_label("  DAILY-Report  "), "daily-report");
}

#[test]
fn keeps_internal_spaces() {
    assert_eq!(normalize_label("  Daily Report  "), "daily report");
}
