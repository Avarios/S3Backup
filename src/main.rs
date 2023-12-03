mod cloud;
use std::{
    env,
    path::Display,
    path::Path,
    time::Instant
};
use cloud::s3::{
    S3_File,
    get_all_files_bucket,
    process_file
};
use chrono;
use chrono::{DateTime, Utc};

use walkdir::{DirEntry, WalkDir};

// Add Logger
// Add Database for failed files and retry
// Scheduler
#[::tokio::main]
async fn main() {
    let path = env::args().nth(1).expect("No Path provided");
    let bucket_name = env::args().nth(2).expect("Please provide a bucket name");
    let bucket_files = get_all_files_bucket(bucket_name).await;
    match bucket_files {
        Ok(files) => {
            let local_files = crawl_path(path);
            let files = get_files_for_backup(local_files, files);
            for file in files {
                upload_file(file).await;
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn get_files_for_backup(local_files: Vec<DirEntry>, bucket_files: Vec<S3_File>) -> Vec<DirEntry> {
   //Write me a function that checks if local file is in bucket file and the modify date from local file is lager than from bucket file
   let mut files = Vec::new();
   for local_file in local_files {
       for bucket_file in bucket_files.iter() {
           let local_file_time:DateTime<Utc> = chrono::DateTime::from(local_file.metadata().unwrap().modified().unwrap());
           let bucket_time = bucket_file.last_modified;
           if local_file.path().display().to_string() == bucket_file.filepath && local_file_time > bucket_time {
               files.push(local_file.to_owned());
           }
       }
   }
   return files;
}

fn crawl_path(path: String) -> Vec<DirEntry>  {
    let timer = Instant::now();
    let mut files = Vec::new();
    for entry in WalkDir::new(path) {
        match entry {
            Ok(entry) => {
                files.push(entry);
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
   return files;
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
        Ok(_entry) => {
            true
        }
        Err(_e) => {
            false
        }
    }
}
