use rust_trait_item_e0046_debug_lab::{LabelFormatter, TaskLabel};
#[test]
fn formats_a_task_label_and_exposes_its_category() {
    let label = TaskLabel { name: String::from("daily") };
    assert_eq!(label.format(), "task:daily");
    assert_eq!(label.category(), "task");
}
