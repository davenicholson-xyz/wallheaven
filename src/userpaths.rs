use dirs;
use std::{env, path::PathBuf};

pub fn config_dir_path() -> PathBuf {
    if env::consts::OS == "windows" {
        let mut conf_dir = dirs::config_dir().unwrap();
        conf_dir.push("wallheaven");
        return conf_dir;
    } else {
        let mut conf_dir = dirs::home_dir().unwrap();
        conf_dir.push(".config/wallheaven");
        return conf_dir;
    }
}

pub fn config_file_path() -> PathBuf {
    let mut conf_file = config_dir_path().clone();
    conf_file.push("config.toml");
    return conf_file;
}

pub fn config_file_string() -> String {
    return config_file_path().into_os_string().into_string().unwrap();
}

#[allow(dead_code)]
pub fn cache_dir_path() -> PathBuf {
    if env::consts::OS == "windows" {
        let mut cache_dir = dirs::cache_dir().unwrap();
        cache_dir.push("wallheaven");
        return cache_dir;
    } else {
        let mut cache_dir = dirs::home_dir().unwrap();
        cache_dir.push(".cache/wallheaven");
        return cache_dir;
    }
}
