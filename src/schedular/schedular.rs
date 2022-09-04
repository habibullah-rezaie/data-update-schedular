use std::{collections::HashMap, env::join_paths, future::Future, pin::Pin};

use chrono::{NaiveTime, Utc, Weekday};
use futures::future::join_all;
use tokio::{join, spawn};

use crate::util::time::diference_in_secs_from_now;

use super::task::{
    task::Task,
    task_options::{DailyTaskOptoins, TaskOptions},
    types::TaskTime,
};

type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send>>;
type TaskRunner = Box<dyn Fn() -> BoxFuture + Send>;
type TodayRestTaskTable = HashMap<u32, String>;

pub struct Schedular {
    pub tasks: HashMap<String, HashMap<String, Vec<Task>>>,
    runners: Vec<(String, TaskRunner)>,
}

impl Schedular {
    pub fn new() -> Self {
        let mut tasks = HashMap::new();

        tasks.insert("day".to_string(), HashMap::new());
        Schedular {
            tasks,
            runners: Vec::new(),
        }
    }

    /// Adds a task that may run every day or in a specific day of week
    pub fn every_day(&mut self, runner: TaskRunner, options: Option<DailyTaskOptoins>) {
        // default options
        let mut task_day = None;
        let mut task_options = TaskOptions {
            time: 0,
            should_run_late: false,
            name: None,
        };

        if let Some(given_options) = options {
            task_options.should_run_late = given_options.super_struct.should_run_late;
            task_options.time = given_options.super_struct.time;
            if let Some(day) = given_options.day {
                task_day = Some(day);
            };
        };

        let new_task = Task::new(task_options);

        match task_day {
            None => {
                let every_day_tasks = self.tasks.get_mut(&String::from("day")).unwrap();

                // push the new_task or create new vector
                match every_day_tasks.get_mut("every") {
                    Some(tasks_vec) => tasks_vec.push(new_task.clone()),
                    None => {
                        every_day_tasks.insert("every".to_string(), vec![new_task.clone()]);
                    }
                };
            }

            Some(day) => {
                let every_day_tasks = self.tasks.get_mut(&String::from("day")).unwrap();

                // push the new_task or create new vector
                match every_day_tasks.get_mut(&day.to_string()) {
                    Some(tasks_vec) => tasks_vec.push(new_task.clone()),
                    None => {
                        every_day_tasks.insert(day.to_string(), vec![new_task.clone()]);
                    }
                };
            }
        };

        self.runners.push((
            String::from(new_task.name),
            Box::new(move || Box::pin(runner())),
        ));
    }

    fn get_every_day_tasks(&self) -> &Vec<Task> {
        self.tasks.get("day").unwrap().get("every").unwrap()
    }

    fn get_every_day_overdue_tasks(&self) -> Vec<&Task> {
        let now = Utc::now().time();

        println!("{}", now);
        self.get_every_day_tasks()
            .iter()
            .filter(|task| {
                NaiveTime::signed_duration_since(
                    now,
                    NaiveTime::from_num_seconds_from_midnight(task.time, 0),
                )
                .num_seconds()
                .is_positive()
            })
            .collect::<Vec<_>>()
    }

    async fn run_overdue_tasks(&self) {
        let mut task_names: HashMap<String, bool> = HashMap::new();
        self.get_every_day_overdue_tasks().iter().for_each(|task| {
            println!("{:#?}", task);
            task_names.insert(String::from(&task.name), true);
        });

        let runners = self
            .runners
            .iter()
            .filter(|runner_tuple| task_names.contains_key(&runner_tuple.0))
            .map(|runner| (runner.1)())
            .collect::<Vec<_>>();

        self.run_tasks(runners).await;
    }

    async fn run_tasks(&self, arg_futures: Vec<BoxFuture>) {
        join_all(arg_futures).await;
    }

    fn prepareTodayRestTaskTable(&self) -> TodayRestTaskTable {
        let mut restTaskTable: TodayRestTaskTable = HashMap::new();

        let every_day_tasks = self.get_every_day_tasks();
        let mut minRest: TaskTime = diference_in_secs_from_now(every_day_tasks[0].time as usize);
        for task in every_day_tasks {}

        return restTaskTable;
    }

    pub async fn start(&self) {
        self.run_overdue_tasks().await;

        loop {
            // Map of (timeOfDay, taskId)
            let mut todayRestTaskTable = self.prepareTodayRestTaskTable();
        }

        // while have tasks
        // get next tasks out of today tasks
        // get rest duration
        // rest
        // run next tasks

        // rust until startof nextday
        // as if its a human ;)
        // }
    }
}
