use rust_trait_import_e0599_debug_lab::format_status;

#[test]
fn writes_the_expected_status_to_memory() {
    assert_eq!(format_status(), "ready");
}
