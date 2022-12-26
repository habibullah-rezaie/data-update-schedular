# data-update-schedular
Async Task Runner 

This project is built to be used to run async tasks given to it, overtime, at exact time assigned.
All project is coded in Rust and I myself use it to retrieve updates from arround the web and store them in BookShelf's database.
So, It can be used to scrap websites regularly.

Example use case:

Every task has a runner, an async func, and other details

```Rust
       let task_a = (
            Box::new(move || Box::pin(dummy_runner())),
            DailyTaskOptoins {
                // SHould run on wed 17 utc
                day: Some(Weekday::Wed),
                super_struct: TaskOptions {
                    should_run_late: true,
                    name: Some("Domy_task1".to_string()),
                    time: 61200,
                },
            },
        )
        
        let mut my_schedular = Schedular::new();

        // Add a task to schedular like this
        my_schedular.every_day(runner, Some(options));
    
        my_schedular.start().await;
```

