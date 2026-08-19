use rust_mutable_reference_e0596_debug_lab::mark_complete;

#[test]
fn appends_the_completion_suffix() {
    let mut label = String::from("daily-report");
    mark_complete(&mut label);

    assert_eq!(label, "daily-report-complete");
}
