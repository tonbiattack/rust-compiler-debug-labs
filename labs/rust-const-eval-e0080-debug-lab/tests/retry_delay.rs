use rust_const_eval_e0080_debug_lab::retry_delay;
#[test] fn uses_a_three_hundred_millisecond_delay() { assert_eq!(retry_delay(), 300); }
