use rust_associated_type_dyn_e0191_debug_lab::{read, Number};
#[test] fn reads_a_number_through_the_trait_object() { assert_eq!(read(&Number), 7); }
