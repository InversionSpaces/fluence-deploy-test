use marine_rs_sdk::marine;
use std::path::PathBuf;
use std::fs;

const SITES_DIR: &str = "/sites/";

pub fn main() {}

#[marine]
pub fn put(name: String, file_content: Vec<u8>) {
    let rpc_tmp_filepath = format!("{}{}", SITES_DIR, name);
    let _ = std::fs::write(PathBuf::from(rpc_tmp_filepath.clone()), file_content);
}

#[marine]
pub fn get(file_name: String) -> Vec<u8> {
    let tmp_filepath = format!("{}{}", SITES_DIR, file_name);
    fs::read(tmp_filepath).unwrap()
}