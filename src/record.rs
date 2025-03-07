use chrono::prelude::*;

use crate::{config::Config, timesheet_entry::TimeSheetEntry};
#[derive(Debug, serde::Deserialize)]
pub struct Record {
    //#[serde(with = "ts_seconds_option")]
    pub time: DateTime<Local>,
    pub application: String,
    pub title: String,
}

impl Record {
    pub fn as_timesheet_entry(&self, config: &Config) -> TimeSheetEntry {
        TimeSheetEntry {
            start_time: Some(self.time),
            end_time: Some(self.time),
            project: self.guess_project(config),
            description: vec![format!("{} - {}", self.application, self.title)],
        }
    }

    fn guess_project(&self, config: &Config) -> String {
        for project in &config.projects {
            for keyword in &project.application_keywords {
                if self.application.contains(keyword) {
                    return project.name.clone();
                }
            }
            for keyword in &project.title_keywords {
                if self.title.contains(keyword) {
                    return project.name.clone();
                }
            }
            /*
            for keyword in &project.path_keywords {
                if self.path.contains(keyword) {
                    return project.name.clone();
                }
            }
            */
        }

        String::from("UNKNOWN")
    }
}