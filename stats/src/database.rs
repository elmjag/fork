use std::path::PathBuf;

use rusqlite::{Connection, Result};

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

fn get_repo_id(connection: &Connection, repository: &PathBuf) -> Result<u32> {
    let name = repository.file_name().unwrap().to_str().unwrap();

    connection.execute(INSERT_REPO_NAME_QUERY, (name,))?;
    let res: Result<u32> = connection.query_row(GET_REPO_ID_QUERY, [name], |row| row.get(0));

    res
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
        let repository_id = get_repo_id(&self.connection, repository).unwrap();

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
}
