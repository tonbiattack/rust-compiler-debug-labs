use rust_iterator_branch_e0308_debug_lab::statuses;

fn main() {
    println!("{:?}", statuses(true).collect::<Vec<_>>());
}
