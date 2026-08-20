pub trait Label {
    fn label(&self) -> String;
}

pub struct Task {
    pub id: u32,
}

impl Label for Task {
    fn label(&self) -> String {
        format!("task-{}", self.id)
    }
}
