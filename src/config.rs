use std::{fs, path::Path};
use serde_json;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub interval: u8,
    pub projects: Vec<Project>,
}

impl Default for Config {
    fn default() -> Self {
        let projects: Vec<Project> = vec![Project::break_project()];

        Self {
            interval: 5,
            projects,
        }
    }
}

impl Config {
    pub fn load_or_create(path: &str) -> Config {
        let path = Path::new(path);
        
        if path.exists() {
            let config = fs::read_to_string(path).expect("Unable to read config file");
            serde_json::from_str(&config).expect("Unable to parse configuration")
        } else {
            println!("Unable to find config. Creating it.");
            let config = Config::default();

            let config_str = serde_json::to_string_pretty(&config).expect("Unable to serialize config.");
            let config_str = config_str.as_str();
            fs::write(path, config_str).expect("Unable to write config");

            config
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Project {
    pub name: String,
    pub application_keywords: Vec<String>,
    pub title_keywords: Vec<String>,
    pub path_keywords: Vec<String>,
}

impl Project {
    pub fn break_project() -> Project {
        Project {
            name: String::from("Break"),
            application_keywords: vec![String::from("LockApp.exe")],
            path_keywords: vec![],
            title_keywords: vec![],
        }
    }
}
