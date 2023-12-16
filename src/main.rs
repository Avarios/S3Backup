mod cloud;
use chrono;
use chrono::{DateTime, Utc};
use cloud::s3::{get_all_files_bucket, put_file, S3_File};
use std::{env, path::Display, path::Path, time::Instant};

use walkdir::{DirEntry, WalkDir};

// Add Logger
// Add Database for failed files and retry
// Scheduler
#[::tokio::main]
async fn main() {
    let path = "E:\\Anleitungen".to_string(); //env::args().nth(1).expect("No Path provided");
    let bucket_name = "nasbak34243243245".to_string(); //env::args().nth(2).expect("Please provide a bucket name");
    match get_all_files_bucket(&bucket_name).await {
        Ok(files) => {
            let local_files = crawl_path(path);
            for file in get_files_for_backup(local_files, files) {
                //TODO: Check if entry is file or not
                let local_file = &file;
                if upload_file(local_file, bucket_name.to_owned().as_str()).await {
                    //TODO: Exchange with logger or logging DB
                    println!("Uploaded file: {}", &local_file)
                } else {
                    //TODO: Exchange with logger or logging DB
                    println!("Error Upload File: {}", local_file)
                }
            }
        }
        Err(e) => {
            println!("{}", e);
            panic!("Cannot access remot files");
        }
    }
}

fn get_files_for_backup(local_files: Vec<DirEntry>, bucket_files: Vec<S3_File>) -> Vec<String> {
    //Write me a function that checks if local file is in bucket file and the modify date from local file is lager than from bucket file
    let mut s3_iter_files = bucket_files.iter();
    let files_to_backup : Vec<String> = local_files
        .iter()
        .filter(|file| {
            return !s3_iter_files.any(|s3| {
                let local_file_time: DateTime<Utc> =
                    chrono::DateTime::from(file.metadata().unwrap().modified().unwrap());
                let bucket_time = s3.last_modified;
                return String::from(file.path().to_str().unwrap()).contains(&s3.file_key)
                    && local_file_time > bucket_time;
            });
        })
        .map(|f| f.path().display().to_string())
        .collect();

    return files_to_backup.to_owned();
}

fn crawl_path(path: String) -> Vec<DirEntry> {
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

async fn upload_file(path: &String, bucket_name: &str) -> bool {
    return match put_file(path, bucket_name).await {
        Ok(_entry) => true,
        Err(_e) => false,
    };
}
