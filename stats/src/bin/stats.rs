use clap::{arg, Command};
use std::fs::canonicalize;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, SystemTimeError, UNIX_EPOCH};

use fork::database::Database;
use fork::repo::get_fork_stats;

///
/// returns path to the repository, as specified with CLI argument
///
fn args() -> (PathBuf, PathBuf) {
    let matches = Command::new("die Gabel")
        .about("how deep does your fork go")
        .arg(arg!([repo_path] "path to Git repository").required(true))
        .arg(arg!([fork_db] "path to fork.db").required(true))
        .get_matches();

    let repo_path = canonicalize(matches.get_one::<String>("repo_path").unwrap()).unwrap();
    let fork_db = canonicalize(matches.get_one::<String>("fork_db").unwrap()).unwrap();

    (repo_path, fork_db)
}

fn get_now_timestamp() -> Result<u64, String> {
    fn get_ts() -> Result<Duration, SystemTimeError> {
        SystemTime::now().duration_since(UNIX_EPOCH)
    }

    match get_ts() {
        Ok(duration) => Ok(duration.as_secs()),
        Err(err) => Err(err.to_string()),
    }
}

fn main() -> Result<(), String> {
    let (repository_path, fork_db) = args();

    let fork_stats = get_fork_stats(&repository_path, "upstream/develop", "maxiv/maxiv-develop")?;
    let db = Database::connect(fork_db.to_str().unwrap())?;

    db.add_fork_stats_entry(
        get_now_timestamp()?,
        &repository_path,
        fork_stats.commits_count,
        fork_stats.insertions,
        fork_stats.deletions,
    )?;

    Ok(())
}
