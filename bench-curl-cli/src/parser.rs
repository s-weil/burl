use core::BenchConfig;
use std::fs;

// TODO: convert to result
pub fn parse_toml(file_name: &str) -> Option<BenchConfig> {
    let file_content = fs::read_to_string(file_name).ok()?;
    let specs: BenchConfig = toml::from_str(&file_content).ok()?;
    Some(specs)
}

pub fn from_get_url(url: String) -> BenchConfig {
    BenchConfig::new(url)
}