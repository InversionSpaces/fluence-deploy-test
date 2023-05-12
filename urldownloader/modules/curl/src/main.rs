use marine_rs_sdk::marine;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Debug)
        .build()
        .unwrap();
}

#[marine]
pub fn download(url: String) -> String {
    log::info!("download called with {}", url);

    let result = curl(vec![url]);
    String::from_utf8(result.stdout).unwrap()
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    fn curl(cmd: Vec<String>) -> MountedBinaryResult;
}