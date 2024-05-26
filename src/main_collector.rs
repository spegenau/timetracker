use chrono::prelude::*;
use std::{fs::OpenOptions, io::Write, path::Path};
use std::{fs, thread, time};

use active_win_pos_rs::get_active_window;

fn append_to_file(filename: &str, line: &str) {
    let path = Path::new(filename);

    if !path.exists() {
        match fs::write(path, line) {
            Ok(_) => {},
            Err(e) => println!("ERROR: Unable to write file: {}", e.to_string()),
        }
    } else {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)
            .unwrap();

        let result = file.write(line.as_bytes());

        match result {
            Ok(_) => {}
            Err(e) => println!("ERROR: Unable to write file: {}", e.to_string()),
        }
    }
}

fn main() {
    let local: DateTime<Local> = Local::now();
    let date = local.format("%Y-%m-%d__%H-%M-%S");
    let filename = format!("{date}.csv");

    append_to_file(&filename, "time,application,title,path\n");

    let interval = 5;

    loop {
        match get_active_window() {
            Ok(active_window) => {
                let time: DateTime<Local> = Local::now();
                time.to_string();

                let line = format!(
                    "{},{},{},{:?}",
                    time.to_string(),
                    active_window.app_name,
                    active_window.title,
                    active_window.process_path
                );
                println!("{line}");

                append_to_file(&filename, format!("{line}\n").as_str());

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
