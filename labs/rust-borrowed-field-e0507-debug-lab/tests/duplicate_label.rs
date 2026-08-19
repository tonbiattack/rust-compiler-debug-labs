use rust_borrowed_field_e0507_debug_lab::{duplicate_label, Job};

#[test]
fn returns_an_owned_copy_of_the_label() {
    let job = Job {
        label: String::from("daily-report"),
    };

    assert_eq!(duplicate_label(&job), "daily-report");
    assert_eq!(job.label, "daily-report");
}
