use std::path::MAIN_SEPARATOR;

pub const CONFIG_PATH_PREFIX: &str = "configs";
pub const DEFAULT_CONFIG: &str = "default";

pub const ENV_DEVELOPMENT: &str = "Development";

pub fn get_config_path(prefix: &str, suffix: &str) -> String {
    format!("{}{}{}", prefix, MAIN_SEPARATOR, suffix)
}

pub fn get_default_config_path(suffix: &str) -> String {
    get_config_path(CONFIG_PATH_PREFIX, suffix)
}
