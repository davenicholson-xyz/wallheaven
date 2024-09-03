mod configuration;
mod files;
mod parseargs;
mod wallhaven;

#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
lazy_static! {
    static ref SETTINGS: Mutex<HashMap<String, String>> = {
        #[allow(unused_mut)]
        let mut m = HashMap::new();
        Mutex::new(m)
    };
}

fn main() {
    let flags = parseargs::cli_args();
    configuration::parse_config(&flags);

    if flags.file {
        let curr = files::cache_to_vec(".current");
        println!("{}", &curr[0].to_string());
        return;
    }

    if flags.url {
        let curr = files::cache_to_vec(".current");
        println!("{}", &curr[1].to_string());
        return;
    }

    if flags.collection.is_some() {
        let chosen = wallhaven::choose_from_collection(flags.collection.unwrap().as_ref());
        let _ = files::set_wallpaper(&chosen);
        return;
    }
}
