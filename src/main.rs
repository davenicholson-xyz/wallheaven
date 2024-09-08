mod configuration;
mod enums;
mod errors;
mod files;
mod parseargs;
mod structs;
mod utils;
mod wallhaven;

use crate::enums::Sorting;
use anyhow::Result;

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

fn main() -> Result<()> {
    let flags = parseargs::cli_args();
    configuration::parse_config(&flags)?;

    if flags.collection.is_some() {
        let wallpapers = wallhaven::collection(&flags.collection.unwrap())?;
        let chosen = utils::random_vec(&wallpapers);
        let set = files::set_wallpaper(&chosen).unwrap();
        println!("{set}");
        return Ok(());
    }

    if flags.random.is_some() {
        let wallpapers = wallhaven::query(Sorting::Random)?;
        let chosen = utils::random_vec(&wallpapers);
        let set = files::set_wallpaper(&chosen).unwrap();
        println!("{set}");
        return Ok(());
    }

    if flags.toplist {
        let wallpapers = wallhaven::query(Sorting::Toplist)?;
        let chosen = utils::random_vec(&wallpapers);
        let set = files::set_wallpaper(&chosen).unwrap();
        println!("{set}");
        return Ok(());
    }

    if flags.hot {
        let wallpapers = wallhaven::query(Sorting::Hot)?;
        let chosen = utils::random_vec(&wallpapers);
        let set = files::set_wallpaper(&chosen).unwrap();
        println!("{set}");
        return Ok(());
    }

    if flags.file {
        let curr = files::cache_to_vec(".current");
        println!("{}", &curr[0].to_string());
        return Ok(());
    }

    if flags.url {
        let curr = files::cache_to_vec(".current");
        println!("{}", &curr[1].to_string());
        return Ok(());
    }

    Ok(())
}
