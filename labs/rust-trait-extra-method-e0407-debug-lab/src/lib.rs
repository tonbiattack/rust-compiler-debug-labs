pub trait Renderer { fn render(&self) -> String; }
pub struct Task { pub title: String }
impl Renderer for Task {
    fn render(&self) -> String { format!("task:{}", self.title) }
}
impl Task {
    pub fn display_name(&self) -> String { self.title.to_uppercase() }
}
