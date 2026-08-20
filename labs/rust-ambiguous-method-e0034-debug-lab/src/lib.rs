pub trait ShortTitle {
    fn title(&self) -> String;
}

pub trait LongTitle {
    fn title(&self) -> String;
}

pub struct Task;

impl ShortTitle for Task {
    fn title(&self) -> String {
        "task".to_owned()
    }
}

impl LongTitle for Task {
    fn title(&self) -> String {
        "task-detail".to_owned()
    }
}

pub fn short_title() -> String {
    <Task as ShortTitle>::title(&Task)
}
