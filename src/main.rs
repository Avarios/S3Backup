mod cloud;
use chrono;
use chrono::{DateTime, Utc};
use cloud::s3::{get_all_files_bucket, put_file, S3File};
use tokio::sync::Semaphore;
use std::sync::Arc;
use std::time::Instant;
use walkdir::{DirEntry, WalkDir};


static BUCKET_NAME:&str = "nasbak34243243245";

pub struct BackupData {
    pub(crate) path: String,
    pub(crate) files: Vec<String>
}


// Add Logger
// Add Database for failed files and retry
// Scheduler
#[::tokio::main]
async fn main() {
    println!("[TRACE]::MAIN -> Starting backup");
    let timer = Instant::now();
    let path = "C:\\Users\\pawel\\Pictures".to_string(); //env::args().nth(1).expect("No Path provided"); //env::args().nth(2).expect("Please provide a bucket name");
    println!("[INFO]::MAIN::main -> Crawling path: {}", path);
    match get_all_files_bucket(&BUCKET_NAME.to_owned()).await {
        Ok(files) => {
            let local_files = crawl_path(path);
            let files_to_backup = check_backup_state(local_files, files);
            
            let tasks: Vec<_> = files_to_backup.into_iter()
                .map(|item| tokio::spawn(async {
                    let local_file = item;
                    if upload_file(&local_file).await {
                        println!("[INFO]::MAIN::main -> Uploaded file: {}", &local_file)
                    } else {
                        println!("[ERROR]::MAIN::main -> Error Upload File: {}", local_file)
                    }
                }))
                .collect();
            let permits = Arc::new(Semaphore::new(10));
            for task in tasks {
                let _permit = permits.acquire().await.unwrap();
                task.await.unwrap();
            }
        }
        Err(e) => {
            println!("{}", e);
            panic!("[ERROR]::MAIN::main -> Cannot access remot files");
        }
    }
    println!("[TRACE]::MAIN -> Backup took {:?}", timer.elapsed());
}

fn check_backup_state(local_files: Vec<DirEntry>, bucket_files: Vec<S3File>) -> Vec<String> {
    let files_to_backup: Vec<String> = local_files
        .into_iter()
        .filter(|file| {
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

//TODO: Crawl trough each dir, set the new vec and then spawn task per directory not file
fn get_local_files(path: String) -> Vec<BackupData> {
    let local_path = path;
    println!("[TRACE]::MAIN::CRAWL_PATH -> Starting Crawler for Path : {}", &local_path);
    let timer = Instant::now();
    let mut local_data: Vec<String> = Vec::new();
    let mut result: Vec<BackupData> = Vec::new();
    for entry in WalkDir::new(&local_path) {
        match entry {
            Ok(entry) => {
                if entry.metadata().unwrap().is_file() {
                    local_data.push(entry.path().display().to_string());
                } else {
                    result.append(&mut get_local_files(entry.path().display().to_string()));
                }
            }
            Err(err) => {
                println!(
                    "[ERROR]::MAIN::CRAWL_PATH -> failed to access entry {}, with error {}",
                    &local_path,
                    err.to_string()
                );
            }
        }
    }
    let duration = timer.elapsed();
    println!("[TRACE]::MAIN::CRAWL_PATH -> Crawler took {:?} for path {}", duration, &local_path);
    result.push(BackupData {
        files: local_data,
        path: local_path.to_owned()
    });
    return result;
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
            }
        }
    }
    let duration = timer.elapsed();
    println!("[TRACE]::MAIN::CRAWL_PATH -> Crawler took {:?} for path {}", duration, &local_path);
    return files;
}

async fn upload_file(path: &str) -> bool {
    return match put_file(path, &BUCKET_NAME).await {
        Ok(_entry) => true,
        Err(_e) => false,
    };
}
