use chrono::Weekday;

use super::types::TaskTime;

pub struct TaskOptions {
    pub should_run_late: bool,
    pub name: Option<String>,
    pub time: TaskTime,
}

pub struct DailyTaskOptoins {
    pub day: Option<Weekday>,
    pub super_struct: TaskOptions,
}
