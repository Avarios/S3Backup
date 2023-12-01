use aws_sdk_s3 as s3;
use s3::primitives::ByteStream;
use std::error::Error;
use std::path::Path;
use aws_sdk_s3::primitives::DateTime;
use walkdir::DirEntry;

pub struct S3_File {
    filepath:String,
    last_modified:DateTime
}

pub async fn get_all_files_bucket(bucket_name:String) -> Result<Vec<S3_File>, Box<dyn Error>> {
    let s3_client = s3::Client::new(&aws_config::load_from_env().await);
    let file_list = s3_client.list_objects_v2().bucket(bucket_name).send().await;
    match file_list {
        Ok(file_list) => {
            let mut file_vec: Vec<S3_File> = Vec::new();
            for file in file_list.contents() {
                let file_path = file.key().unwrap();
                let file_last_modified = file.last_modified().unwrap().to_owned();
                file_vec.push(S3_File {
                    filepath: file_path.to_owned(),
                    last_modified: file_last_modified,
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

pub async fn process_file(entry: DirEntry) -> Result<(), Box<dyn Error>> {
    // make checks
    // is file already on S3 Bucket ?
    // is it archived already ?
    return put_file(entry.path()).await;
}

async fn put_file(path: &Path) -> Result<(), Box<dyn Error>> {
    let file_stream = ByteStream::from_path(path).await;
    let stream_data = match file_stream {
        Ok(stream_data) => stream_data,
        Err(e) => {
            return Err(format!(
                "Bot able to open filestream for: {} with ERR -> {} ",
                path.display(),
                e.to_string()
            )
            .to_owned())?
        }
    };

    let config = aws_config::load_from_env().await;
    let s3client = s3::Client::new(&config);
    let put_object_output = s3client
        .put_object()
        .bucket("input")
        .key("SETZE PFAD HERE")
        .body(stream_data)
        .send()
        .await;
    match put_object_output {
        Ok(_) => return Ok(()),
        Err(e) => {
            return Err(format!(
                "Not able to upload file to S3 FILE:{} -> ERR: {}",
                path.display(),
                e.to_string()
            )
            .to_owned())?
        }
    };
}
