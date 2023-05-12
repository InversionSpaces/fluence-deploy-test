use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

module_manifest!();

pub fn main() {}

#[marine]
pub fn prepend_http(url: String) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        url
    } else {
        format!("http://{}", url)
    }
}

#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;

    #[marine_test(config_path = "../../../Config.toml")]
    fn test_without_http(prepend: marine_test_env::httpprepend::ModuleInterface) {
        let url = "example.com".to_string();
        let result = prepend.prepend_http(url);
        assert_eq!(result, "http://example.com");
    }

    #[marine_test(config_path = "../../../Config.toml")]
    fn test_with_http(prepend: marine_test_env::httpprepend::ModuleInterface) {
        let url = "https://example.com".to_string();
        let result = prepend.prepend_http(url);
        assert_eq!(result, "https://example.com");
    }
}
