use rusqlite::{Connection, Result};
use serde::Serialize;
use std::path::PathBuf;

const DB_SCHEMA: [&str; 2] = [
    "
    create table if not exists repository(
        id integer primary key,
        name text not null unique,
        unique(id, name))
    ",
    "
    create table if not exists fork_stats(
        repository integer,
        timestamp integer,
        commits integer,
        insertions integer,
        deletions integer,
        foreign key(repository) references repository(id)
    )
    ",
];

const INSERT_REPO_NAME_QUERY: &str = "
    insert or ignore into repository(name) values(?1)
";

const GET_REPO_ID_QUERY: &str = "
    select id from repository where name=?1
";

const INSERT_FORK_STATS_QUERY: &str = "
    insert into
        fork_stats(repository,
                   timestamp,
                   commits,
                   insertions,
                   deletions)
        values(?1, ?2, ?3, ?4, ?5)
";

const GET_FORK_STATS_QUERY: &str = "
    select
        timestamp,
        commits,
        insertions,
        deletions
    from fork_stats
    where repository = ?1
    order by timestamp;
";

pub struct Database {
    connection: Connection,
}

fn setup_db(db_file_path: &str) -> Result<Connection> {
    /* connect to the database */
    let conn = Connection::open(db_file_path)?;

    /* maybe create set-up database schema */
    for statement in DB_SCHEMA {
        conn.execute(statement, ())?;
    }

    Ok(conn)
}

#[derive(Serialize, Default)]
pub struct ForkStats {
    timestamps: Vec<u64>,
    commits: Vec<u32>,
    insertions: Vec<u32>,
    deletions: Vec<u32>,
}

fn get_repo_id_by_name(connection: &Connection, name: &str) -> Result<u32> {
    connection.execute(INSERT_REPO_NAME_QUERY, (name,))?;
    let res: Result<u32> = connection.query_row(GET_REPO_ID_QUERY, [name], |row| row.get(0));

    res
}

fn get_repo_id_by_path(connection: &Connection, repository: &PathBuf) -> Result<u32> {
    let name = repository.file_name().unwrap().to_str().unwrap();
    get_repo_id_by_name(connection, name)
}

fn add_fork_stats_entry(
    connection: &Connection,
    repository_id: u32,
    timestamp: u64,
    commits_count: usize,
    insertions: usize,
    deletions: usize,
) -> Result<(), String> {
    let res = connection.execute(
        INSERT_FORK_STATS_QUERY,
        (
            repository_id,
            timestamp,
            commits_count,
            insertions,
            deletions,
        ),
    );

    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

fn get_fork_stats(connection: &Connection, repository_id: u32) -> Result<ForkStats> {
    let mut stmt = connection.prepare(GET_FORK_STATS_QUERY)?;
    let mut rows = stmt.query([repository_id])?;

    let mut fork_stats = ForkStats::default();

    while let Some(row) = rows.next()? {
        fork_stats.timestamps.push(row.get(0)?);
        fork_stats.commits.push(row.get(1)?);
        fork_stats.insertions.push(row.get(2)?);
        fork_stats.deletions.push(row.get(3)?);
    }

    Ok(fork_stats)
}

impl Database {
    pub fn connect(db_file_path: &str) -> Result<Database, String> {
        let conn = match setup_db(db_file_path) {
            Ok(conn) => conn,
            Err(err) => {
                return Err(err.to_string());
            }
        };
        Ok(Database { connection: conn })
    }

    pub fn add_fork_stats_entry(
        &self,
        timestamp: u64,
        repository: &PathBuf,
        commits_count: usize,
        insertions: usize,
        deletions: usize,
    ) -> Result<(), String> {
        let repository_id = get_repo_id_by_path(&self.connection, repository).unwrap();

        println!(
            "DB: {} {} id = {} commits {} insertions {} deletions {}",
            timestamp,
            repository.display(),
            repository_id,
            commits_count,
            insertions,
            deletions,
        );

        add_fork_stats_entry(
            &self.connection,
            repository_id,
            timestamp,
            commits_count,
            insertions,
            deletions,
        )?;

        Ok(())
    }

    pub fn get_fork_stats(&self, repository_name: &str) -> Result<ForkStats, String> {
        let repository_id = get_repo_id_by_name(&self.connection, repository_name).unwrap();
        let res = get_fork_stats(&self.connection, repository_id);

        match res {
            Ok(fork_stats) => Ok(fork_stats),
            Err(err) => Err(err.to_string()),
        }
    }
}
