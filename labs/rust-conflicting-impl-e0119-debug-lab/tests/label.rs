use rust_conflicting_impl_e0119_debug_lab::{Label, Task};
#[test] fn gives_a_task_its_specific_label() { assert_eq!(Task { id: 7 }.label(), "task-7"); }
