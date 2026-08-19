use rust_slice_split_at_mut_e0499_debug_lab::overwrite_first_two;

#[test]
fn overwrites_the_first_two_values_without_changing_the_rest() {
    let mut values = vec![1, 2, 3];

    overwrite_first_two(&mut values);

    assert_eq!(values, vec![10, 20, 3]);
}

#[test]
fn accepts_a_slice_with_exactly_two_values() {
    let mut values = vec![0, 0];

    overwrite_first_two(&mut values);

    assert_eq!(values, vec![10, 20]);
}
