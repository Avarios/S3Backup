# S3 Backuper
This is a Rust written S3 Backup process.
You can run this as a normal program or as a Docker container.

## Setup

You have to specify the directory you want to backup , the S3 bucket name.

````
S3backup.exe "D:\myfilestobackp" "Bucketname"
````

Furthermore, you have to provide aws cli access
by setting ENV Variable AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY or
As entries in the credentials file in the .aws directory in your home directory (~/.aws/credentials on Linux, macOS, and Unix; %userprofile%\.aws\credentials on Microsoft Windows):

````

[default]
aws_access_key_id=YOUR-ACCESS-KEY
aws_secret_access_key=YOUR-SECRET-KEY

````

## The How.

1. Crawl trough all the folder/subfolder and collect all files
2. Get all Files in S3 Bucket
3. Compare if file is already in bucket and if so, the local file is newer
4. Upload all needed files to S3

