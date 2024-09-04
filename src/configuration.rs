use crate::parseargs::Flags;
use crate::{files, SETTINGS};
use config::Config;
use std::collections::HashMap;
use std::path::Path;

pub fn parse_config(flags: &Flags) {
    let mut settings = SETTINGS.lock().unwrap();
    settings.insert("purity".to_string(), "100".to_string());
    settings.insert("categories".to_string(), "111".to_string());
    settings.insert("pages".to_string(), "5".to_string());
    settings.insert("toprange".to_string(), "1M".to_string());
    settings.insert("ratios".to_string(), "landscape".to_string());

    let default_config_path = files::config_file_string();
    let flags_config = flags.config.clone();

    let cfg_path = flags_config.unwrap_or(default_config_path);

    let cfg_file = parse_config_file(cfg_path.to_string());
    let cfg_flags = parse_flags_config(flags);

    settings.extend(cfg_file);
    settings.extend(cfg_flags);
}

fn parse_flags_config(flags: &Flags) -> HashMap<String, String> {
    let mut flags_hash = HashMap::<String, String>::new();

    let fields_to_parse = vec!["apikey", "collection", "random", "purity", "categories"];

    for field in fields_to_parse {
        if let Some(value) = flags.get(field) {
            if value.is_some() {
                flags_hash.insert(field.to_string(), value.as_ref().unwrap().to_string());
            }
        }
    }
    return flags_hash;
}

fn parse_config_file(cfg_path: String) -> HashMap<String, String> {
    if Path::new(&cfg_path).exists() {
        let fileconfig = Config::builder()
            .add_source(config::File::with_name(cfg_path.as_ref()))
            .build()
            .unwrap();

        return fileconfig
            .try_deserialize::<HashMap<String, String>>()
            .unwrap();
    }

    return HashMap::<String, String>::new();
}
