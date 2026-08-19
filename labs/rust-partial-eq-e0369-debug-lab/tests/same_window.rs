use rust_partial_eq_e0369_debug_lab::{same_window, ReleaseWindow};

#[test]
fn recognizes_equal_release_windows() {
    assert!(same_window(
        ReleaseWindow { start_hour: 9, end_hour: 17 },
        ReleaseWindow { start_hour: 9, end_hour: 17 },
    ));
}

#[test]
fn recognizes_different_release_windows() {
    assert!(!same_window(
        ReleaseWindow { start_hour: 9, end_hour: 17 },
        ReleaseWindow { start_hour: 10, end_hour: 17 },
    ));
}
