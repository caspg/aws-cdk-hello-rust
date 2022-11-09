use aws_sdk_s3::{
    model::{BucketLocationConstraint, CreateBucketConfiguration},
    types::ByteStream,
    Client, Error, Region,
};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let bucket_region = "us-east-2";
    let bucket_name = "hello-rust-bucket-1";

    let region_provider = Region::new(bucket_region);
    let config = aws_config::from_env().region(region_provider).load().await;
    let s3 = Client::new(&config);

    // listt the first 10 keys in the bucket
    let result = s3
        .list_objects_v2()
        .bucket(bucket_name)
        .max_keys(10)
        .send()
        .await?;

    for object in result.contents().unwrap() {
        let key = object.key().unwrap();

        let object = s3.get_object().bucket(bucket_name).key(key).send().await?;

        // convert the body into a string
        let data = object.body.collect().await.unwrap().into_bytes();

        // Note that this code assumes that the files are utf8 encoded plain text format.
        let content = std::str::from_utf8(&data).unwrap();

        println!("key: #{key}, content: {content}");
    }

    Ok(())
}

async fn upload_file() {
    let bucket_region = "us-east-2";
    let bucket_name = "hello-rust-bucket-1";

    let region_provider = Region::new(bucket_region);
    let config = aws_config::from_env().region(region_provider).load().await;
    let s3 = Client::new(&config);

    let file_name = "testfile.txt";
    let file_path = Path::new(file_name);
    // unwrap will panic if there's an error reading the file
    let file_body = ByteStream::from_path(file_path).await.unwrap();

    let result = s3
        .put_object()
        .bucket(bucket_name)
        .key(file_name)
        .body(file_body)
        .content_type("text/plain")
        .send()
        .await;

    match result {
        Ok(_) => {
            println!("Succesfully uploaded {file_name} to {bucket_name}");
        }

        Err(err) => {
            eprintln!("Error uploading {file_name} {err}");
        }
    }
}

async fn print_buckets() -> Result<(), Error> {
    // get default credentials from ~/.aws/credentials
    let config = aws_config::load_from_env().await;

    // create an s3 client
    let s3 = Client::new(&config);

    // list the first page of buckets in the account
    let response = s3.list_buckets().send().await?;

    if let Some(buckets) = response.buckets() {
        for bucket in buckets {
            println!("bucket name: {}", bucket.name().unwrap());
        }
    } else {
        println!("You don't have any buckets!");
    }

    Ok(())
}

async fn create_bucket() -> Result<(), Error> {
    let bucket_region = "us-east-2";
    let bucket_name = "hello-rust-bucket-1";

    let region_provider = Region::new(bucket_region);
    let config = aws_config::from_env().region(region_provider).load().await;

    let s3 = Client::new(&config);

    println!("Creating {bucket_name} in {bucket_region}");

    let constraint = BucketLocationConstraint::from(bucket_region);

    let bucket_configuration = CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();

    let result = s3
        .create_bucket()
        .create_bucket_configuration(bucket_configuration)
        .bucket(bucket_name)
        .send()
        .await;

    match result {
        Ok(_) => {
            println!("Successfull created {bucket_name}");
        }

        Err(err) => {
            eprintln!("Failed to create bucket: {err}");
        }
    }

    Ok(())
}
