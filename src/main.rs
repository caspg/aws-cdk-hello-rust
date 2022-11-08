use aws_sdk_s3::{Client, Error};

async fn print_buckets() -> Result<(), Error> {
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    print_buckets().await
}
