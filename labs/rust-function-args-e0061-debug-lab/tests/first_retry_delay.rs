use rust_function_args_e0061_debug_lab::first_retry_delay;
#[test]
fn calculates_the_first_retry_delay() {
    assert_eq!(first_retry_delay(), 10);
}
