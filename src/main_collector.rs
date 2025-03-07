use chrono::prelude::*;
use std::fs::create_dir_all;
use std::{fs, thread, time};
use std::{fs::OpenOptions, io::Write, path::Path};

use active_win_pos_rs::get_active_window;

fn append_to_file(filename: &str, line: &str) {
    let path = Path::new(filename);

    if !path.exists() {
        match fs::write(path, line) {
            Ok(_) => {}
            Err(e) => println!("ERROR: Unable to write file: {e}"),
        }
    } else {
        let mut file = OpenOptions::new().append(true).open(path).unwrap();

        let result = file.write(line.as_bytes());

        match result {
            Ok(_) => {}
            Err(e) => println!("ERROR: Unable to write file: {e}"),
        }
    }
}

fn main() {
    let interval = 5;

    // Get the home directory
    let home_dir = dirs::home_dir().expect("Unable to get home directory");
    let folder_path = home_dir.join("timetracker_entries");

    // Create the folder if it does not exist
    if !folder_path.exists() {
        if let Err(e) = create_dir_all(&folder_path) {
            println!("ERROR: Unable to create directory: {e}");
            return;
        }
    }

    loop {
        match get_active_window() {
            Ok(active_window) => {
                let time: DateTime<Local> = Local::now();
                let date = time.format("%Y-%m-%d");
                let file_path = folder_path.join(format!("{date}.csv"));
                let filename = file_path.to_str().unwrap();

                if !Path::new(filename).exists() {
                    append_to_file(filename, "time,application,title\n");
                }

                let line = format!(
                    "\"{}\",\"{}\",\"{}\"",
                    time, active_window.app_name, active_window.title
                );
                println!("{line}");

                append_to_file(filename, format!("{line}\n").as_str());

                //println!("active window: {:#?}", active_window);

                let sleep_duration = time::Duration::from_secs(interval);
                thread::sleep(sleep_duration);
            }
            Err(()) => {
                println!("error occurred while getting the active window");
            }
        }
    }
}
