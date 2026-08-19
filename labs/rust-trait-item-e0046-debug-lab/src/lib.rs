pub trait LabelFormatter {
    fn format(&self) -> String;
    fn category(&self) -> &'static str;
}

pub struct TaskLabel {
    pub name: String,
}

impl LabelFormatter for TaskLabel {
    fn format(&self) -> String {
        format!("task:{}", self.name)
    }

    fn category(&self) -> &'static str {
        "task"
    }
}
