pub struct Task { pub title: String, pub priority: u8 }
pub fn priority() -> u8 {
    let task = Task { title: String::from("daily"), priority: 3 };
    let Task { priority, .. } = task;
    priority
}
