use std::fmt::{Display, Formatter, Result as FmtResult};

pub trait Render {
    fn render<T: Display>(&self, value: T) -> String;
}

pub struct Brackets;

pub struct Visible(pub &'static str);

impl Display for Visible {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter.write_str(self.0)
    }
}

impl Render for Brackets {
    fn render<T: Display>(&self, value: T) -> String {
        format!("[{value}]")
    }
}
