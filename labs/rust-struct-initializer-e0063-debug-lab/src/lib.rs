pub struct Task {
    pub title: String,
    pub priority: u8,
}

pub fn daily() -> Task {
    Task {
        title: String::from("daily"),
        priority: 1,
    }
}
