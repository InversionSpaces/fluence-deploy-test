use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

module_manifest!();

pub fn main() {}

#[marine]
fn get_n_save(url: String, file_name: String) -> String {
    let downloaded = download(prepend_http(url));
    file_put(file_name, downloaded.into_bytes());
    String::from("Ok")
}

#[marine]
#[link(wasm_import_module = "httpprepend")]
extern "C" {
    pub fn prepend_http(url: String) -> String;
}

#[marine]
#[link(wasm_import_module = "curl")]
extern "C" {
    pub fn download(url: String) -> String;
}

#[marine]
#[link(wasm_import_module = "fs")]
extern "C" {
    // this link_name attribute allows using "file_get" name 
    // for function imported by "get" name
    #[link_name = "get"]
    pub fn file_get(file_name: String) -> Vec<u8>;
    
    #[link_name = "put"]
    pub fn file_put(name: String, file_content: Vec<u8>);
}