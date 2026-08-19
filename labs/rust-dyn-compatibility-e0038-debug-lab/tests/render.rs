use rust_dyn_compatibility_e0038_debug_lab::{render, Note};
#[test] fn renders_id_through_a_trait_object() { assert_eq!(render(&Note { id: String::from("n-1") }), "n-1"); }
