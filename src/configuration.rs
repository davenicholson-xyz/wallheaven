use crate::files;
use config::Config;
use std::collections::HashMap;
use std::path::Path;

pub fn parse_config(args_config: &Option<String>) -> HashMap<String, String> {
    let mut cfg: HashMap<String, String> = HashMap::new();

    // Sensible defaults
    cfg.insert("purity".to_string(), "100".to_string());
    cfg.insert("categories".to_string(), "111".to_string());
    cfg.insert("pages".to_string(), "5".to_string());
    cfg.insert("toprange".to_string(), "1M".to_string());
    cfg.insert("ratios".to_string(), "landscape".to_string());

    let default_config_path = files::config_file_string();
    let cfg_path = args_config
        .as_deref()
        .unwrap_or(default_config_path.as_ref());
    println!("{}", &cfg_path);
    let cfg_file = parse_config_file(cfg_path.to_string());

    let mut settings = cfg.clone();
    settings.extend(cfg_file);
    return settings;
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
