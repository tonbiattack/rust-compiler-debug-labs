pub trait Storage {
    type Item;
    fn value(&self) -> Self::Item;
}

pub struct Number;

impl Storage for Number {
    type Item = u32;

    fn value(&self) -> u32 {
        7
    }
}

pub fn read(storage: &dyn Storage<Item = u32>) -> u32 {
    storage.value()
}
