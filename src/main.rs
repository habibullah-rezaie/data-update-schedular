use dotenv::dotenv;
use schedular::schedular::Schedular;
use tasks::get_tasks;

mod schedular;
mod tasks;
mod util;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut my_schedular = Schedular::new();

    for (runner, options) in get_tasks() {
        my_schedular.every_day(runner, Some(options));
    }
    my_schedular.start().await;
}
