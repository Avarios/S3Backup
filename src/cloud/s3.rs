use aws_sdk_s3 as s3;
use s3::primitives::{ByteStream, SdkBody};
use std::{fs};

pub async fn process_file(entry: walkdir::DirEntry) -> Result<(),()> {    
    let file_stream = ByteStream::from_path(entry.path()).await;
    let stream_data = match file_stream {
        Ok(stream_data) => stream_data,
        Err(_) => return Err(())
    };

    let config = aws_config::load_from_env().await;
    let s3client = s3::Client::new(&config);
    let put_object_output = s3client.put_object().bucket("input").key("SETZE PFAD HERE").body(stream_data).send().await;
    match put_object_output {
        Ok(_) => return Ok(()),
        Err(_) => return Err(())
    };
    
} 
