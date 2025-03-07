mod record;
use chrono::{DateTime, Local};
use config::Config;
use record::Record;
use timesheet_entry::{TimeSheetEntries, TimeSheetEntry};
#[macro_use]
extern crate prettytable;

mod config;
mod timesheet_entry;

fn parse_csv(path: &str) -> Vec<Record> {
    let mut rdr = csv::Reader::from_path(path).unwrap();

    let mut results: Vec<Record> = Vec::new();

    for result in rdr.deserialize() {
        match result {
            Ok(result) => results.push(result),
            Err(err) => {
                println!("Error parsing: {err}")
            }
        }
    }
    results
}

fn main() {
    // Get the home directory
    let home_dir = dirs::home_dir().expect("Unable to get home directory");
    let folder_path = home_dir.join("timetracker_entries");

    let file = match std::env::args().nth(1) {
        Some(path) => path,
        None => {
            let time: DateTime<Local> = Local::now();
            let date = time.format("%Y-%m-%d");
            let file_path = folder_path.join(format!("{date}.csv"));
            file_path.to_str().unwrap().to_string()
        }
    };

    let config_file = folder_path.join("config.json");
    let config = Config::load_or_create(config_file.to_str().unwrap());
    println!("Using config: {:#?}", config_file);

    let records = parse_csv(file.as_str());

    let mut entries: Vec<TimeSheetEntry> = Vec::new();

    let mut current_entry: Option<TimeSheetEntry> = None;

    let mut i = 0;
    let len = records.len() as i32;
    for record in records {
        let entry = record.as_timesheet_entry(&config);

        i += 1;
        match &current_entry {
            Some(existing_entry) => {
                if i != len && entry.project == existing_entry.project {
                    let mut new_current_entry = existing_entry.clone();
                    new_current_entry.end_time = entry.end_time;
                    new_current_entry
                        .description
                        .push(entry.description.first().unwrap().clone());

                    current_entry = Some(new_current_entry);
                } else {
                    entries.push(existing_entry.clone());
                    current_entry = Some(entry.clone());
                }
            }
            None => {
                current_entry = Some(entry);
            }
        }
    }

    println!("Found {} entries", entries.len());

    entries.as_table().printstd();

    let unknown_entries: Vec<TimeSheetEntry> = entries
        .clone()
        .into_iter()
        .filter(|e| e.project == "UNKNOWN")
        .collect();
    unknown_entries.as_table().printstd();

    entries.time_per_projects().printstd();

    let start_time = entries
        .first()
        .unwrap_or(&TimeSheetEntry::default())
        .start_time
        .unwrap_or(Local::now());
    let end_time = entries
        .last()
        .unwrap_or(&TimeSheetEntry::default())
        .end_time
        .unwrap_or(Local::now());
    let duration = end_time - start_time;

    let simple_format = "%H:%M:%S";

    let minutes = duration.num_minutes() % 60;
    let hours = duration.num_hours();

    println!(
        "Start: {}\tEnd: {},\tDuration: {hours}:{minutes}h",
        start_time.format(simple_format),
        end_time.format(simple_format)
    );
}
