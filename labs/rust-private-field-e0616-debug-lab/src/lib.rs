mod model {
    pub struct Task {
        title: String,
    }

    impl Task {
        pub fn new(title: &str) -> Self {
            Self { title: title.to_owned() }
        }

        pub fn title(&self) -> &str {
            &self.title
        }
    }
}

pub fn read_title() -> String {
    let task = model::Task::new("daily-report");
    task.title().to_owned()
}
