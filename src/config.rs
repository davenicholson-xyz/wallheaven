use crate::files;
use crate::flags;
use config::Config;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::sync::Mutex;

pub static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    let flags = flags::cli_args();

    let mut config = Config::builder()
        .set_default("pages", 3)
        .unwrap()
        .set_default("expiry", 600)
        .unwrap()
        .set_default("purity", "110".to_string())
        .unwrap()
        .set_default("categories".to_string(), "111".to_string())
        .unwrap()
        .set_default("pages".to_string(), 5)
        .unwrap()
        .set_default("range".to_string(), "1M".to_string())
        .unwrap()
        .set_default("ratios".to_string(), "landscape".to_string())
        .unwrap();

    if let Some(flags_config) = flags.config {
        let cfg_path = PathBuf::from(&flags_config);
        if cfg_path.exists() {
            config = config.add_source(config::File::with_name(&flags_config));
        } else {
            panic!("File Error: Cannot find configuration file");
        }
    } else {
        let default_config_path = files::config_file_string();
        config = config.add_source(config::File::with_name(&default_config_path));
    }

    config = config.add_source(config::Environment::with_prefix("WALLHEAVEN"));

    if let Some(pages) = flags.pages {
        config = config.set_override("pages", pages).unwrap();
    }

    if let Some(expiry) = flags.expiry {
        config = config.set_override("expiry", expiry).unwrap();
    }

    if let Some(username) = flags.username {
        config = config.set_override("username", username).unwrap();
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

    if let Some(script) = flags.script {
        config = config.set_override("post_script", script).unwrap();
    }

    if let Some(range) = flags.range {
        config = config.set_override("range", range).unwrap();
    }

    Mutex::new(config.build().unwrap())
});
