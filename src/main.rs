use aws_sdk_s3::{
    model::{BucketLocationConstraint, CreateBucketConfiguration},
    Client, Error, Region,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    Ok(())
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
