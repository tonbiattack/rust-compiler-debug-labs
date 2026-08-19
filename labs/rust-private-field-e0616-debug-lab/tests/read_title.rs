use rust_private_field_e0616_debug_lab::read_title;
#[test]
fn returns_the_title_through_the_public_api() {
    assert_eq!(read_title(), "daily-report");
}
