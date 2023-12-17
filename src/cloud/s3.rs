use std::{error::Error, path::Path};

use aws_sdk_s3 as s3;
use aws_smithy_types_convert::date_time::DateTimeExt;
use chrono::{DateTime, Utc};
use s3::primitives::ByteStream;

#[derive(Clone)]
pub struct S3_File {
    pub(crate) file_key: String,
    pub(crate) last_modified: DateTime<Utc>,
}

pub async fn get_all_files_bucket(bucket_name: &String) -> Result<Vec<S3_File>, Box<dyn Error>> {
    let s3_client = s3::Client::new(&aws_config::load_from_env().await);
    let file_list = s3_client.list_objects_v2().bucket(bucket_name).send().await;
    match file_list {
        Ok(file_list) => {
            let mut file_vec: Vec<S3_File> = Vec::new();
            for file in file_list.contents() {
                let file_last_modified = file
                    .last_modified()
                    .map(|t| t.to_chrono_utc())
                    .unwrap()
                    .expect("date must be set");
                file_vec.push(S3_File {
                    file_key: file.key.to_owned().unwrap().to_owned(),
                    last_modified: file_last_modified.to_owned(),
                });
            }
            return Ok(file_vec);
        }
        Err(e) => {
            return Err(format!(
                "Not able to get file list from S3 Bucket: with ERR: {}",
                e.to_string()
            )
            .to_owned())?
        }
    }
}

pub async fn put_file(path: &String, bucket: &str) -> Result<(), Box<dyn Error>> {
    let file_stream = ByteStream::from_path(path.to_owned()).await;
    let local_path = path;
    let stream_data = match file_stream {
        Ok(stream_data) => stream_data,
        Err(e) => {
            return Err(format!(
                "Bot able to open filestream for: {} with ERR -> {} ",
                local_path,
                e.to_string()
            )
            .to_owned())?
        }
    };
    let config = aws_config::load_from_env().await;
    let s3client = s3::Client::new(&config);
    let put_object_output = s3client
        .put_object()
        .bucket(bucket)
        //Extract path without windows drive names if windows
        .key(local_path)
        .body(stream_data)
        .send()
        .await;
    match put_object_output {
        Ok(_) => return Ok(()),
        Err(e) => {
            return Err(format!(
                "Not able to upload file to S3 FILE:{} -> ERR: {}",
                local_path,
                e.to_string()
            )
            .to_owned())?
        }
    };
}
