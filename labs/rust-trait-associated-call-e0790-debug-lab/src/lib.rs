pub trait Slug {
    fn prefix() -> &'static str;
}

pub struct News;
pub struct Note;

impl Slug for News {
    fn prefix() -> &'static str {
        "news"
    }
}

impl Slug for Note {
    fn prefix() -> &'static str {
        "note"
    }
}

pub fn default_prefix() -> &'static str {
    <News as Slug>::prefix()
}
