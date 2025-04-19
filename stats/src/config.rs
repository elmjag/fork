use chrono::NaiveTime;
use std::fs::read_to_string;
use std::path::PathBuf;
use toml::{map::Map, Table, Value};

pub struct Config {
    root_table: Map<String, Value>,
}

impl Config {
    pub fn new(config_file: PathBuf) -> Config {
        let toml = read_to_string(config_file).unwrap();
        let root_table = toml.parse::<Table>().unwrap();

        Config { root_table }
    }

    pub fn get_database_file(&self) -> String {
        self.root_table["database_file"]
            .as_str()
            .unwrap()
            .to_string()
    }

    pub fn get_ueb_bin(&self) -> String {
        self.root_table["ueb"]["bin"].as_str().unwrap().to_string()
    }

    pub fn get_ueb_ui(&self) -> String {
        self.root_table["ueb"]["ui"].as_str().unwrap().to_string()
    }

    pub fn get_stats_bin(&self) -> String {
        self.root_table["stats"]["bin"]
            .as_str()
            .unwrap()
            .to_string()
    }

    pub fn get_stats_exec_time(&self) -> NaiveTime {
        let exec_time = self.root_table["stats"]["exec_time"].as_str().unwrap();

        NaiveTime::parse_from_str(exec_time, "%H:%M:%S").unwrap()
    }

    pub fn get_repos(&self) -> impl Iterator<Item = &str> + '_ {
        let repos = self.root_table["repos"].as_array().unwrap();
        repos.into_iter().map(|v| v.as_str().unwrap())
    }
}
