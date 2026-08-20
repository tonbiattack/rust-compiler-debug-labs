pub struct Defaults;

impl Defaults {
    pub fn value<T: Default>(&self) -> T {
        T::default()
    }
}

pub fn default_count() -> u32 {
    Defaults.value()
}
