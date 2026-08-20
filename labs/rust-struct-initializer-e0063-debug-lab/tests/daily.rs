use rust_struct_initializer_e0063_debug_lab::daily;
#[test] fn creates_a_daily_task_with_normal_priority() { let task = daily(); assert_eq!(task.title, "daily"); assert_eq!(task.priority, 1); }
