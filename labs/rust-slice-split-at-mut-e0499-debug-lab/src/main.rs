use rust_slice_split_at_mut_e0499_debug_lab::overwrite_first_two;

fn main() {
    let mut values = vec![0, 0, 0];
    overwrite_first_two(&mut values);
    println!("{values:?}");
}
