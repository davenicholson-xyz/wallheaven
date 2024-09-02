use std::collections::HashMap;
use std::path::Path;

use crate::userpaths;
use config::Config;

pub fn parse_config() -> HashMap<String, String> {
    let mut cfg: HashMap<String, String> = HashMap::new();

    // Sensible defaults
    cfg.insert("purity".to_string(), "100".to_string());
    cfg.insert("categories".to_string(), "111".to_string());
    cfg.insert("pages".to_string(), "5".to_string());
    cfg.insert("toprange".to_string(), "1M".to_string());
    cfg.insert("ratios".to_string(), "landscape".to_string());

    let cfg_file = parse_config_file();

    let mut settings = cfg.clone();
    settings.extend(cfg_file);
    return settings;
}

fn parse_config_file() -> HashMap<String, String> {
    if Path::new(&userpaths::config_file_string()).exists() {
        let fileconfig = Config::builder()
            .add_source(config::File::with_name(&userpaths::config_file_string()))
            .build()
            .unwrap();

        return fileconfig
            .try_deserialize::<HashMap<String, String>>()
            .unwrap();
    }

    return HashMap::<String, String>::new();
}
