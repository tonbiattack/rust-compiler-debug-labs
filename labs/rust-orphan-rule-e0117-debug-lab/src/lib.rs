use std::fmt;

pub struct Labels(pub Vec<String>);

impl fmt::Display for Labels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join(","))
    }
}

pub fn labels() -> String {
    Labels(vec![String::from("rust"), String::from("cargo")]).to_string()
}
