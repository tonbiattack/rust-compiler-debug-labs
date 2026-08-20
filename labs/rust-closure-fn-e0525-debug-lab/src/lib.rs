pub fn twice<F: Fn() -> String>(build: F) -> (String, String) {
    (build(), build())
}

pub fn labels() -> (String, String) {
    let label = String::from("task");
    let build = || label.clone();
    twice(build)
}
