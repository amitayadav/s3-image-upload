# Load credentials directly
let access_key = String::from("AKIAIOSFODNN7EXAMPLE");
let secret_key = String::from("wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY");
let credentials = Credentials::new(Some(access_key), Some(secret_key), None, None);

# Load credentials from the environment
use std::env;
env::set_var("AWS_ACCESS_KEY_ID", "AKIAIOSFODNN7EXAMPLE");
env::set_var("AWS_SECRET_ACCESS_KEY", "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY");
let credentials = Credentials::new(None, None, None, None);

# Loads from the standard AWS credentials file with the given profile name,
defaults to "default".

use s3::credentials::Credentials;

* Load credentials from `[default]` profile
let credentials = Credentials::default();

* Also loads credentials from `[default]` profile
let credentials = Credentials::new(None, None, None, None);

*Load credentials from `[my-profile]` profile
let credentials = Credentials::new(None, None, None, Some("my-profile".into()));