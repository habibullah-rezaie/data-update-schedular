use std::time::Duration;

use crate::schedular::task::task_options::{DailyTaskOptoins, TaskOptions};
use dotenv::dotenv;
use schedular::schedular::Schedular;
use tokio::time::{sleep_until, Instant};

mod schedular;
mod tasks;
mod util;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut my_schedular = Schedular::new();

    async fn my_runner2() {
        sleep_until(Instant::now() + Duration::new(3, 0)).await;
        println!("HI2");
    }

    async fn my_runner1() {
        sleep_until(Instant::now() + Duration::new(20, 0)).await;
        println!("MY");
    }
    async fn my_runner() {
        sleep_until(Instant::now() + Duration::new(45, 0)).await;
        println!("HI");
    }

    my_schedular.every_day(
        Box::new(move || Box::pin(my_runner())),
        Some(DailyTaskOptoins {
            day: None,
            super_struct: TaskOptions {
                should_run_late: false,
                name: None,
                time: 0,
            },
        }),
    );

    my_schedular.every_day(
        Box::new(move || Box::pin(my_runner1())),
        Some(DailyTaskOptoins {
            day: None,
            super_struct: TaskOptions {
                should_run_late: false,
                name: None,
                time: 37800,
            },
        }),
    );

    my_schedular.every_day(
        Box::new(move || Box::pin(my_runner2())),
        Some(DailyTaskOptoins {
            day: None,
            super_struct: TaskOptions {
                should_run_late: false,
                name: None,
                time: 10,
            },
        }),
    );
    my_schedular.start().await;
}
