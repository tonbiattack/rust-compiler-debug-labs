mod model {
    pub struct Task { title: String }
    impl Task {
        pub fn new(title: &str) -> Self { Self { title: title.to_owned() } }
        pub fn display_title(&self) -> String { self.title.to_uppercase() }
    }
}
pub fn public_title() -> String {
    let task = model::Task::new("daily-report");
    task.display_title()
}
