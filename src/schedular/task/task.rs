use uuid::Uuid;

use super::{task_options::TaskOptions, types::TaskTime};

#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub should_run_late: bool,
    pub time: TaskTime,
}

impl Task {
    pub fn new(options: TaskOptions) -> Self {
        Task {
            name: options.name.unwrap_or(Uuid::new_v4().to_string()),
            should_run_late: options.should_run_late,
            time: options.time,
        }
    }
}
