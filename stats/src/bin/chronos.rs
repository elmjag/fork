use clap::arg;
use std::fs::canonicalize;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;

use fork::config::Config;
use fork::delays::get_delay;

///
/// returns path to the config file, as specified with CLI argument
///
fn args() -> PathBuf {
    let matches = clap::Command::new("die Zeit")
        .about("how deep does your fork go")
        .arg(arg!([config_file] "path to config.toml").required(true))
        .get_matches();

    let config_file = canonicalize(matches.get_one::<String>("config_file").unwrap()).unwrap();
    config_file
}

fn start_ueb(config: &Config) {
    println!("launching ueb");

    let res = Command::new(config.get_ueb_bin())
        .arg(config.get_ueb_ui())
        .arg(config.get_database_file())
        .output();

    match res {
        Err(err) => println!("Failed to start ueb: {}", err.to_string()),
        Ok(output) => {
            println!(
                "Ueb process terminated, {}\n{}{}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}

fn collect_fork_stats(config: &Config, repo: &str) {
    println!("collecting fork statistics for {repo}\n");

    let res = Command::new(config.get_stats_bin())
        .arg(repo)
        .arg(config.get_database_file())
        .output();

    match res {
        Err(err) => println!("Failed to collect statistics: {}", err.to_string()),
        Ok(output) => {
            println!(
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}

fn periodically_collect_stats(config: &Config) {
    let execution_time = config.get_stats_exec_time();

    loop {
        // add 1 second to the delay,
        // to avoid running multiple times same day
        let delay = get_delay(&execution_time) + Duration::from_secs(1);
        thread::sleep(delay);

        for repo in config.get_repos() {
            collect_fork_stats(config, repo);
        }
    }
}

fn main() {
    let config: &'static Config = Box::leak(Box::new(Config::new(args())));
    let mut handles = vec![];

    handles.push(thread::spawn(move || start_ueb(&config)));
    handles.push(thread::spawn(move || periodically_collect_stats(&config)));

    for handle in handles {
        handle.join().unwrap();
    }
}
