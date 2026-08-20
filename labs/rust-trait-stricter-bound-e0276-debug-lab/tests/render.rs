use rust_trait_stricter_bound_e0276_debug_lab::{Brackets, Render, Visible};

#[test]
fn renders_a_display_only_value_between_brackets() {
    assert_eq!(Brackets.render(Visible("task")), "[task]");
}
