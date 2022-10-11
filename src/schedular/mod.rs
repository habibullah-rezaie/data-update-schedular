pub mod task;

use std::{collections::HashMap, future::Future, pin::Pin};

use chrono::Utc;
use futures::future::join_all;

use crate::util::time::get_diff_from_now_in_secs;

use self::task::{
    task_options::{DailyTaskOptoins, TaskOptions},
    types::TaskId,
    Task,
};

type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send>>;
pub type TaskRunner = Box<dyn Fn() -> BoxFuture + Send>;

type TaskRepeatTable = HashMap<String, HashMap<String, Vec<TaskId>>>;

pub struct Schedular {
    tasks: HashMap<TaskId, Task>,
    task_repeat_table: TaskRepeatTable,
    runners: HashMap<TaskId, TaskRunner>,
    first_day_running: bool,
}

impl Schedular {
    pub fn new() -> Self {
        let tasks: HashMap<TaskId, Task> = HashMap::new();

        let mut task_repeat_table: TaskRepeatTable = HashMap::new();

        task_repeat_table.insert("day".to_string(), HashMap::new());
        Schedular {
            tasks,
            task_repeat_table,
            runners: HashMap::new(),
            first_day_running: true,
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
                let every_day_tasks = self
                    .task_repeat_table
                    .get_mut(&String::from("day"))
                    .unwrap();

                // push the new_task or create new vector
                match every_day_tasks.get_mut("every") {
                    Some(tasks_vec) => tasks_vec.push(new_task.name.clone()),
                    None => {
                        every_day_tasks.insert("every".to_string(), vec![new_task.name.clone()]);
                    }
                };
            }

            Some(day) => {
                let every_day_tasks = self
                    .task_repeat_table
                    .get_mut(&String::from("day"))
                    .unwrap();

                // push the new_task or create new vector
                match every_day_tasks.get_mut(&day.to_string()) {
                    Some(tasks_vec) => tasks_vec.push(new_task.name.clone()),
                    None => {
                        every_day_tasks.insert(day.to_string(), vec![new_task.name.clone()]);
                    }
                };
            }
        };

        self.tasks.insert(new_task.name.clone(), new_task.clone());
        self.runners.insert(
            String::from(&new_task.name),
            Box::new(move || Box::pin(runner())),
        );
    }

    fn get_every_day_tasks(&self) -> Vec<TaskId> {
        self.task_repeat_table
            .get("day")
            .unwrap()
            .get("every")
            .unwrap_or(&Vec::new())
            .clone()
    }

    // fn get_tasks_of_ids(&self, task_ids: Vec<TaskId>) -> Vec<Task> {
    //     task_ids
    //         .iter()
    //         .map(|task_id| self.tasks.get(task_id).unwrap().clone())
    //         .collect::<Vec<_>>()
    // }

    fn get_a_particular_weekday_tasks(&self, day: &str) -> Vec<TaskId> {
        if !["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"].contains(&day) {
            panic!("Invalid weekday provided");
        }

        self.task_repeat_table
            .get("day")
            .unwrap()
            .get(&day.to_string())
            .unwrap_or(&Vec::new())
            .clone()
    }

    pub fn get_task(&self, task_id: &String) -> &Task {
        self.tasks
            .get(task_id)
            .expect(&(String::from("Task not found: ") + task_id))
    }

    fn get_all_overdue_for_start(&self) -> Vec<TaskId> {
        let mut overdue_tasks: Vec<TaskId> = Vec::new();

        // Look for due daily tasks
        self.task_repeat_table
            .get("day")
            .unwrap()
            .keys()
            .for_each(|day_period| {
                if day_period == &String::from("every") {
                    let mut everyday_overdue = self
                        .get_every_day_tasks()
                        .iter()
                        .filter_map(|task_id| {
                            // everyday tasks are overdue only if their run time is past
                            let task = self.get_task(task_id);

                            if get_diff_from_now_in_secs(task.time).unwrap().is_negative() {
                                Some(task_id.clone())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();

                    if !everyday_overdue.is_empty() {
                        overdue_tasks.append(&mut everyday_overdue);
                    }
                } else {
                    let weekdays = ["Not a day", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

                    // Get the Weekday in number from 1 - 7  and  is Mon is 1
                    let today_index = format!("{}", Utc::today().format("%u"));

                    // get the index of day_period
                    if let Some(day_period_index) =
                        weekdays.iter().position(|day| day == day_period)
                    {
                        // tasks in a specific day are overdue in two cases:
                        // It is in the same day as today

                        println!("{day_period_index} day_period_index");
                        if day_period_index.to_string() == today_index {
                            let mut day_period_overdues = self
                                .task_repeat_table
                                .get("day")
                                .unwrap()
                                .get(day_period)
                                .unwrap()
                                .iter()
                                .filter_map(|task_id| {
                                    // everyday tasks are overdue only if their run time is past
                                    let task = self.get_task(task_id);

                                    if get_diff_from_now_in_secs(task.time).unwrap().is_negative() {
                                        Some(task_id.clone())
                                    } else {
                                        None
                                    }
                                })
                                .collect();

                            overdue_tasks.append(&mut day_period_overdues);
                            // It is past in this week

                            // TODO: Check if this works properly
                        } else if day_period_index.to_string() < today_index {
                            let mut tasks_of_day_period = self
                                .task_repeat_table
                                .get("day")
                                .unwrap()
                                .get(day_period)
                                .unwrap()
                                .clone();

                            overdue_tasks.append(&mut tasks_of_day_period);
                        }
                    };
                }
            });

        overdue_tasks
    }

    async fn run_overdue_tasks(&self) -> Vec<TaskId> {
        let overdue_tasks = self.get_all_overdue_for_start();

        let task_runners = overdue_tasks
            .iter()
            .filter_map(|task_id| {
                let task = self.get_task(task_id);

                return if task.should_run_late {
                    let runner = self.runners.get(task_id).unwrap();
                    Some((runner)())
                } else {
                    None
                };
            })
            .collect::<Vec<_>>();

        self.run_tasks(task_runners).await;

        overdue_tasks
    }

    async fn run_tasks(&self, arg_futures: Vec<BoxFuture>) {
        join_all(arg_futures).await;
    }

    fn get_today_tasks(&self) -> Vec<String> {
        // get everyday tasks
        let mut today_tasks = self.get_every_day_tasks();

        // get today's weekday and tasks in it
        let now = Utc::now().to_rfc2822();
        let today = &now.split_once(',').unwrap().0;
        let mut tasks_of_this_weekday = self.get_a_particular_weekday_tasks(today);

        today_tasks.append(&mut tasks_of_this_weekday);

        // Sort based on their time
        today_tasks.sort_by(|a, b| {
            let a_task = self.tasks.get(a).unwrap();
            let b_task = self.tasks.get(b).unwrap();

            a_task.time.partial_cmp(&b_task.time).unwrap()
        });

        today_tasks
    }

    pub async fn start(&mut self) {
        let overdue_tasks = self.run_overdue_tasks().await;

        println!("this is overdue tasks {:?}", overdue_tasks);
        loop {
            println!("Started day at {}", Utc::now().timestamp());
            let mut today_tasks = self.get_today_tasks();

            // Do not include overdue tasks if it's the first day that program has started running
            if self.first_day_running {
                today_tasks = today_tasks
                    .iter()
                    .filter_map(|task_id| {
                        if overdue_tasks.contains(task_id) {
                            None
                        } else {
                            Some(task_id.clone())
                        }
                    })
                    .collect::<Vec<String>>();

                // Set the first_day_running to false,
                // meaning that there should not be any overdue tasks.
                self.first_day_running = false;
            }

            println!("this is tasks {:?}", today_tasks);
            while !today_tasks.is_empty() {
                let (time_to_run, runners) = self.get_tasks_for_next_run(&mut today_tasks);

                let rest_duration = get_diff_from_now_in_secs(time_to_run).unwrap();
                assert!(rest_duration.is_positive());

                tokio::time::sleep(std::time::Duration::from_secs(rest_duration as u64)).await;

                self.run_tasks(runners).await;
                println!("{:?}", today_tasks);
            }

            let seconds_passed_today = Utc::now().timestamp() % 86400;
            let seconds_till_tomorrow = (86400 - seconds_passed_today) as u64;

            tokio::time::sleep_until(
                tokio::time::Instant::now()
                    + tokio::time::Duration::from_millis(seconds_till_tomorrow * 1000),
            )
            .await;
        }
    }

    fn get_tasks_for_next_run(&self, today_tasks: &mut Vec<String>) -> (u32, Vec<BoxFuture>) {
        // In case no tasks for today is empty

        // get the time of the first task
        // get the difference from now
        let first_task_time = self.tasks.get(&today_tasks[0]).unwrap().time;

        let mut runners: Vec<BoxFuture> = Vec::new();
        let mut task_id = today_tasks.remove(0);
        while self.tasks.get(&task_id).unwrap().time == first_task_time {
            let runner_fn = self.runners.get(&task_id).unwrap();
            runners.push((runner_fn)());

            if today_tasks.is_empty() {
                break;
            } else {
                task_id = today_tasks.remove(0);
            }
        }

        (first_task_time, runners)
    }
}
