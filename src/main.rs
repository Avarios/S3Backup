mod cloud;
use chrono;
use chrono::{DateTime, Utc};
use cloud::s3::{get_all_files_bucket, put_file, S3File};
use std::{env, time::Instant};
use walkdir::{DirEntry, WalkDir};


// Add Logger
// Add Database for failed files and retry
// Scheduler
#[::tokio::main]
async fn main() {
    let path = "E:\\Anleitungen".to_string(); //env::args().nth(1).expect("No Path provided");
    let bucket_name = "nasbak34243243245".to_string(); //env::args().nth(2).expect("Please provide a bucket name");
    println!("[INFO]::MAIN::main -> Crawling path: {}", path);
    match get_all_files_bucket(&bucket_name).await {
        Ok(files) => {
            let local_files = crawl_path(path);
            for file in get_files_for_backup(local_files, files) {
                let local_file = &file;
                if upload_file(local_file, bucket_name.to_owned().as_str()).await {
                    println!("[INFO]::MAIN::main -> Uploaded file: {}", &local_file)
                } else {
                    println!("[ERROR]::MAIN::main -> Error Upload File: {}", local_file)
                }
            }
        }
        Err(e) => {
            println!("{}", e);
            panic!("[ERROR]::MAIN::main -> Cannot access remot files");
        }
    }
}

fn get_files_for_backup(local_files: Vec<DirEntry>, bucket_files: Vec<S3File>) -> Vec<String> {
    //Write me a function that checks if local file is in bucket file and the modify date from local file is lager than from bucket file

    let files_to_backup: Vec<String> = local_files
        .into_iter()
        .filter(|file| {
            //TODO: Replace any
            let mut files_bucket = bucket_files.to_vec().into_iter();
            let file_path = String::from(file.path().display().to_string());

            match files_bucket.find(|f| f.file_key.eq(&file_path)) {
                Some(f) => {
                    let local_file_time: DateTime<Utc> =
                        chrono::DateTime::from(file.metadata().unwrap().modified().unwrap());
                    let bucket_time = f.last_modified;
                    let is_newer = local_file_time > bucket_time;
                    return is_newer;
                }
                None => {
                    return true;
                }
            };
        })
        .map(|f| f.path().display().to_string())
        .collect();

    return files_to_backup;
}

fn crawl_path(path: String) -> Vec<DirEntry> {
    let local_path = path;
    println!("[TRACE]::MAIN::CRAWL_PATH -> Starting Crawler for Path : {}", &local_path);
    let timer = Instant::now();
    let mut files = Vec::new();
    for entry in WalkDir::new(&local_path) {
        match entry {
            Ok(entry) => {
                if entry.metadata().unwrap().is_file() {
                    files.push(entry);
                }
            }
            Err(err) => {
                println!(
                    "[ERROR]::MAIN::CRAWL_PATH -> failed to access entry {}, with error {}",
                    &local_path,
                    err.to_string()
                );
                //Add to logger
            }
        }
    }
    let duration = timer.elapsed();
    println!("[TRACE]::MAIN::CRAWL_PATH -> Crawler took {:?} for path {}", duration, &local_path);
    return files;
}

async fn upload_file(path: &String, bucket_name: &str) -> bool {
    return match put_file(path, bucket_name).await {
        Ok(_entry) => true,
        Err(_e) => false,
    };
}
