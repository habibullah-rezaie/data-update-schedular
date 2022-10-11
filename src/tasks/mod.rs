mod domy_task;

use chrono::Weekday;

use crate::schedular::{
    task::task_options::{DailyTaskOptoins, TaskOptions},
    TaskRunner,
};

pub fn get_tasks() -> Vec<(TaskRunner, DailyTaskOptoins)> {
    return vec![(
        Box::new(move || Box::pin(domy_task::domy_task_runner())),
        DailyTaskOptoins {
            // As an example the task should be run on every Tuesday at 00:00:00
            day: Some(Weekday::Tue),
            super_struct: TaskOptions {
                should_run_late: true,
                name: Some("Domy_task".to_string()),
                time: 0,
            },
        },
    )];
}
