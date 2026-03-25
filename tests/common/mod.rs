#![allow(dead_code)]

use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use letit::LetItClient;

pub const PRODUCTION_BASE_URL: &str = "https://api.letit.com";

pub fn live_client() -> LetItClient {
    LetItClient::new(PRODUCTION_BASE_URL, required_api_token())
}

pub fn required_api_token() -> String {
    env::var("LETIT_API_TOKEN")
        .expect("LETIT_API_TOKEN must be set to run live API integration tests")
}

pub fn unique_name(prefix: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before UNIX_EPOCH")
        .as_millis();

    format!("{prefix}-{timestamp}")
}

pub fn repository_logo() -> (String, Vec<u8>, String) {
    let logo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(".github")
        .join("logo.png");
    let bytes = fs::read(&logo_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", logo_path.display()));

    ("logo.png".to_string(), bytes, "image/png".to_string())
}
