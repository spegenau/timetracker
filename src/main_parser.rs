mod record;
use config::Config;
use record::Record;
use timesheet_entry::{TimeSheetEntries, TimeSheetEntry};
#[macro_use] extern crate prettytable;

mod config;
mod timesheet_entry;

fn parse_csv(path: &str) -> Vec<Record> {
    let mut rdr = csv::Reader::from_path(path).unwrap();

    let mut results: Vec<Record> = Vec::new();

    for result in rdr.deserialize() {
        match result {
            Ok(result) => results.push(result),
            Err(err) => {
                println!("Error parsing: {}", err.to_string())
            }
        }
    }
    results
}

fn main() {
    let file = std::env::args()
        .nth(1)
        .expect("Please provide the path to the CSV file as first argument");

    let config = Config::load_or_create("config.json");

    let records = parse_csv(file.as_str());

    let mut entries: Vec<TimeSheetEntry> = Vec::new();

    let mut current_entry: Option<TimeSheetEntry> = None;

    for record in records {
        let entry = record.as_timesheet_entry(&config);

        match &current_entry {
            Some(existing_entry) => {
                if entry.project == existing_entry.project {
                    let mut new_current_entry = existing_entry.clone();
                    new_current_entry.end_time = entry.end_time;
                    new_current_entry.description.push(entry.description.get(0).unwrap().clone());

                    current_entry = Some(new_current_entry);
                } else {
                    entries.push(existing_entry.clone());
                    current_entry = Some(entry.clone());
                }
            },
            None => {
                current_entry = Some(entry);
            },
        }
    }

    entries.as_table().printstd();

    entries.time_per_projects().printstd();
}
