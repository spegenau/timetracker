use std::collections::HashMap;

use chrono::prelude::*;
use prettytable::Table;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct TimeSheetEntry {
    //#[serde(with = "ts_seconds_option")]
    pub start_time: Option<DateTime<Local>>,
    //#[serde(with = "ts_seconds_option")]
    pub end_time: Option<DateTime<Local>>,
    pub project: String,
    pub description: Vec<String>,
}

impl Default for TimeSheetEntry {
    fn default() -> Self {
        Self {
            start_time: None,
            end_time: None,
            project: String::from("UNKNOWN"),
            description: Vec::new(),
        }
    }
}

impl TimeSheetEntry {
    pub fn get_duration_as_minutes(&self) -> i64 {
        let start = self.start_time.expect("No start time");
        let end = self.end_time.expect("No end time");

        let duration = end - start;

        duration.num_minutes()

    }
}

pub trait TimeSheetEntries {
    fn as_table(&self) -> Table;
    fn time_per_projects(&self) -> Table;
}

fn format_datetime(datetime: Option<DateTime<Local>>) -> String {
    match datetime {
        Some(datetime) => {
            datetime.format("%H:%M:%S").to_string()
        },
        None => String::from(""),
    }
}

impl TimeSheetEntries for Vec<TimeSheetEntry> {
    fn as_table(&self) -> Table {
        // Create the table
        let mut table = Table::new();

        table.add_row(row!["Start", "End", "Duration (minutes)", "Project", "Description"]);


        
        for entry in self {
            let mut descriptions = entry.description.clone();
            descriptions.sort();
            descriptions.dedup();

            table.add_row(row![
                format_datetime(entry.start_time),
                format_datetime(entry.end_time),
                entry.get_duration_as_minutes(),
                entry.project,
                descriptions.join("\n"),
            ]);
        }

        table
    }
    
    fn time_per_projects(&self) -> Table {
        // Create the table
        let mut table = Table::new();

        table.add_row(row!["Project", "Duration (minutes)"]);
        
        let mut durations: HashMap<String, i64> = HashMap::new();

        for entry in self {
            let project = entry.project.clone();
            if !durations.contains_key(&project) {
                durations.insert(project.clone(), 0);   
            }

            let duration = durations.get(&project).unwrap() + entry.get_duration_as_minutes();

            durations.insert(project, duration);
        }

        for (project, duration) in &durations {
            table.add_row(row![project, duration]);
        }

        table
        
    }
}
