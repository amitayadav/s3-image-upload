extern crate s3;

use std::fs::File;
use std::io::Read;

use s3::bucket::Bucket;
use s3::credentials::Credentials;
use s3::serde_types::ListBucketResult;

pub mod constants;

use constants::{BUCKET, REGION, FILE_PATH, STATUS_CODE, BUCKET_FILE_PATH};
use s3::region::Region;
use constants::STATUS_CODE_DEL;
use constants::BUCKET_FILE_PATH_DEL;

fn main() {
    let credentials: Credentials = Credentials::new(None,None,None,None);
    let region: Region = REGION.parse().unwrap();

    /// Instantiate a new `Bucket`.
    let bucket: Bucket = Bucket::new(BUCKET, region, credentials);

    /// List the contents of an S3 bucket.
    let results: Vec<(ListBucketResult,u32)> = bucket.list("", None).unwrap();
    for (list, code) in results {
        assert_eq!(STATUS_CODE, code);
        println!("List of objects in a particular bucket{:?}", list.contents.len());
    }

    /// Delete file from an S3 path.
    let (_, code) = bucket.delete(BUCKET_FILE_PATH_DEL).unwrap();
    assert_eq!(STATUS_CODE_DEL, code);
    println!("Object deleted successfully");

    /// Attempts to open a file in read-only mode.
    let mut file = File::open(FILE_PATH).unwrap();
    let mut content: Vec<u8> = vec![];
    /// Read all bytes until EOF in this source, placing them into `buf`.
    file.read_to_end(&mut content).unwrap();
    println!("Image file read successfully");

    /// Put into an S3 bucket.
    let (_, code) = bucket.put(BUCKET_FILE_PATH, content.as_ref(), "text/plain")
        .unwrap();
    assert_eq!(STATUS_CODE, code);
    println!("Image Uploaded successfully ");

    /// Gets file from an S3 path.
    let (_data,code)= bucket.get(BUCKET_FILE_PATH).unwrap();
    assert_eq!(STATUS_CODE,code);
    println!("Image Retrieved successfully");
}