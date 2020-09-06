use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use s3::S3Error;

#[tokio::main]
async fn main() -> Result<(), S3Error> {
    let bucket_name = "test";
    let region = Region::Custom {
        region: "us-east-1".to_string(),
        endpoint: "http://114.55.145.17:5432".to_string(),
    };
    let credentials = Credentials::new(Some("admin"), Some("admin123"), None, None, None)?;
    let bucket = Bucket::new_with_path_style(bucket_name, region, credentials)?;
    // let mut output_file = File::create("output_file").expect("Unable to create file");
    //
    // let status_code = bucket
    //     .get_object_stream("/test.file", &mut output_file)
    //     .await?;
    // println!("Code: {}", status_code);

    let results = bucket.list("".to_string(), Some("".to_string())).await?;
    for m in results {
        for i in m.contents {
            println!("{}", i.key);
        }
    }
    Ok(())
}
