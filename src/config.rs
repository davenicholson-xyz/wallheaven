use crate::files;
use crate::flags;
use config::Config;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    let flags = flags::cli_args();

    let default_config_path = files::config_file_string();
    let flags_config = flags.config.clone();
    let cfg_path = flags_config.unwrap_or(default_config_path);

    let mut config = Config::builder()
        .set_default("expiry", 600)
        .unwrap()
        .set_default("purity", "100".to_string())
        .unwrap()
        .set_default("categories".to_string(), "111".to_string())
        .unwrap()
        .set_default("pages".to_string(), 5)
        .unwrap()
        .set_default("toprange".to_string(), "1M".to_string())
        .unwrap()
        .set_default("ratios".to_string(), "landscape".to_string())
        .unwrap()
        .add_source(config::File::with_name(&cfg_path))
        .add_source(config::Environment::with_prefix("WALLHEAVEN"));

    if let Some(expiry) = flags.expiry {
        config = config.set_override("expiry", expiry).unwrap();
    }

    if let Some(apikey) = flags.apikey {
        config = config.set_override("apikey", apikey).unwrap();
    }

    if let Some(purity) = flags.purity {
        config = config.set_override("purity", purity).unwrap();
    }

    if let Some(categories) = flags.categories {
        config = config.set_override("categories", categories).unwrap();
    }

    if let Some(collection) = flags.collection {
        config = config.set_override("collection", collection).unwrap();
    }

    if let Some(random) = flags.random {
        config = config.set_override("random", random).unwrap();
    }

    Mutex::new(config.build().unwrap())
});
