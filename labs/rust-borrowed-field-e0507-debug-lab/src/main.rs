use rust_borrowed_field_e0507_debug_lab::{duplicate_label, Job};

fn main() {
    let job = Job {
        label: String::from("daily-report"),
    };
    println!("{}", duplicate_label(&job));
}
