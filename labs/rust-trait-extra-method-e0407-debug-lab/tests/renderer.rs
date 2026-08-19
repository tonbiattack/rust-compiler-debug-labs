use rust_trait_extra_method_e0407_debug_lab::{Renderer, Task};
#[test] fn renders_and_exposes_a_display_name() { let task=Task{title:String::from("daily")}; assert_eq!(task.render(),"task:daily"); assert_eq!(task.display_name(),"DAILY"); }
