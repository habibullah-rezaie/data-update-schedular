use tokio::time::Duration as TokioDuration;

pub async fn domy_task_runner() {
    tokio::time::sleep(TokioDuration::from_secs(5)).await;
    println!("Domy_task_ran!")
}
