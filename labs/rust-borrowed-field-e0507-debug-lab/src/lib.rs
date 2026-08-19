pub struct Job {
    pub label: String,
}

/// ジョブのラベルを所有値として返します。
pub fn duplicate_label(job: &Job) -> String {
    job.label.clone()
}
