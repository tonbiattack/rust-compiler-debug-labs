mod labels { pub fn format(label: &str) -> String { format!("status:{label}") } }
use self::labels::format;
pub fn status() -> String { format("ready") }
