use aws_sdk_s3 as s3;
use s3::primitives::ByteStream;
use std::error::Error;
use std::path::Path;
use walkdir::DirEntry;

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
                path,
                e.to_string()
            )
            .to_owned())?
        }
    };
}
