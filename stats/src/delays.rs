use std::time::Duration;

use chrono::{DateTime, NaiveTime, Utc};

fn get_execution_date_time(now: &DateTime<Utc>, execution_time: &NaiveTime) -> DateTime<Utc> {
    println!(
        "Current time {}, execution time {}",
        now.time(),
        execution_time
    );

    if *execution_time < now.time() {
        // execute tomorrow
        let later = now
            .date_naive()
            .succ_opt()
            .unwrap()
            .and_time(*execution_time)
            .and_utc();

        println!("execute tomorrow at: {}", later);
        later
    } else {
        // execute later today
        let later = now.date_naive().and_time(*execution_time).and_utc();
        println!("execute today at: {}", later);
        later
    }
}

pub fn get_delay(execution_time: &NaiveTime) -> Duration {
    let now = Utc::now();
    let exec_time = get_execution_date_time(&now, execution_time);

    (exec_time - now).to_std().unwrap()
}
