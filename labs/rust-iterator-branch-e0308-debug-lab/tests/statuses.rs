use rust_iterator_branch_e0308_debug_lab::statuses;

#[test]
fn includes_closed_status_when_requested() {
    let actual = statuses(true).collect::<Vec<_>>();

    assert_eq!(actual, vec!["open", "closed"]);
}

#[test]
fn returns_only_open_status_when_closed_is_not_requested() {
    let actual = statuses(false).collect::<Vec<_>>();

    assert_eq!(actual, vec!["open"]);
}
