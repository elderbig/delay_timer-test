use delay_timer;
use anyhow::Result;
use delay_timer::prelude::*;
use delay_timer::utils::convenience::functions as dt_functions;
use std::time::Duration;
use std::thread;
use log::info;


// #[tokio::main]
#[async_std::main]
async fn main() -> Result<()>{
    log4rs::init_file("conf/log4rs.yaml", Default::default()).unwrap();
    info!("begin");
    let delay_timer = DelayTimerBuilder::default().build();
    for i in 0..1000{
        let r = build_task_async_execute_process(i);
        match r {
            Ok(task) => {
                let r = delay_timer.add_task(task);
                match r {
                    Ok(_) => info!("init task id = [{}]", i),
                    Err(e) => info!("{}", e)
                } 
            },
            Err(e)=> info!("{}", e)
        }
    }
    info!("==== All job is be init! ====");
    thread::sleep(Duration::from_secs(3600*6));
    Ok(delay_timer.stop_delay_timer()?)
}

fn build_task_async_execute_process(task_id:u64) -> Result<Task, TaskError> {
    let mut task_builder = TaskBuilder::default();
    let cmd_string = String::from("sh tester/test_script.sh");
    info!("cmd_string = [{}]", &cmd_string);
    let body = dt_functions::unblock_process_task_fn(cmd_string);
    task_builder
        .set_frequency_repeated_by_cron_str("* * * * * *".into())
        .set_task_id(task_id)
        .set_maximum_running_time(10)
        .set_maximum_parallel_runnable_num(1)
        .spawn(body)
}
