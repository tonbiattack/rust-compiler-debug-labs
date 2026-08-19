use rust_thread_spawn_e0373_debug_lab::complete_in_worker;

#[test]
fn returns_a_completion_message_from_the_worker() {
    let actual = complete_in_worker("月次集計".to_owned());

    assert_eq!(actual, "月次集計: 完了");
}

#[test]
fn accepts_an_empty_label_without_changing_the_message_format() {
    let actual = complete_in_worker(String::new());

    assert_eq!(actual, ": 完了");
}
