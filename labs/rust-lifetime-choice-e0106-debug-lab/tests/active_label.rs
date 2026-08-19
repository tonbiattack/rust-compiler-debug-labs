use rust_lifetime_choice_e0106_debug_lab::active_label;

#[test]
fn returns_the_primary_label_when_it_is_not_empty() {
    assert_eq!(active_label("daily", "default"), "daily");
}

#[test]
fn returns_the_fallback_label_when_primary_is_empty() {
    assert_eq!(active_label("", "default"), "default");
}
