pub fn apply_to_count<F: Fn(u32) -> u32>(operation: F) -> u32 {
    operation(3)
}

pub fn incremented_count() -> u32 {
    apply_to_count(|value: u32| value + 1)
}
