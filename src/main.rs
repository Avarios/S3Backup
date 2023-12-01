mod cloud;
use std::{
    env,
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
    let bucket_name = env::args().nth(2).expect("Please put a bucketname as second argument");
    let bucket_files = cloud::s3::get_all_files_bucket(bucket_name).await;
    crawl_path(path);
}

fn crawl_path(path: String) {
    let timer = Instant::now();
    for entry in WalkDir::new(path) {
        match entry {
            Ok(entry) => {
                let _ = process_entry(entry);
            }
            Err(err) => {
                let path: Display<'_> = err.path().unwrap_or(Path::new("")).display();
                println!("failed to access entry {}", path);
                //Add to logger
            }
        }
    }
   let duration = timer.elapsed();
   println!("Took {:?}", duration);
}

async fn process_entry(entry: DirEntry) {
    if entry.file_type().is_file() {
        if check_archive_status(entry).await {

        }
    }
   
}

async fn check_archive_status(entry: DirEntry) -> bool {
    return  true;
}

async fn upload_file(entry:DirEntry) -> bool {
    return match cloud::s3::process_file(entry).await {
        Ok(entry) => {
            true
        }
        Err(e) => {
            false
        }
    }
}
