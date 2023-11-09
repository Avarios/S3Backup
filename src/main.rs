mod cloud;
use std::{
    env,
    fs::{self, Metadata},
    path::Display,
    path::Path,
    time::Instant
};
use walkdir::{DirEntry, WalkDir};


// Add Logger
// Add Database for failed files and retry
// Scheduler
#[::tokio::main]
async fn main() {
    let path = env::args().nth(1).expect("No Path provided");
    crawl_path(path);
}

fn crawl_path(path: String) {
    let timer = Instant::now();
    for entry in WalkDir::new(path) {
        match entry {
            Ok(entry) => {
                process_entry(entry);
            }
            Err(err) => {
                let path: Display<'_> = err.path().unwrap_or(Path::new("")).display();
                println!("failed to access entry {}", path);
                //Add to logger
                //Mark as not backuped
            }
        }
    }
   let duration = timer.elapsed();
   println!("Took {:?}", duration);
}

fn process_entry(entry: DirEntry) {
    if entry.file_type().is_file() {
        check_archive_status(entry);
    }
   
}

fn check_archive_status(entry: DirEntry) {
    cloud::s3::process_file(entry);
}
