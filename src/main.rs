mod configuration;
mod enums;
mod errors;
mod files;
mod parseargs;
mod structs;
mod utils;
mod wallhaven;

use crate::enums::Sorting;

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

fn main() -> Result<(), reqwest::Error> {
    let flags = parseargs::cli_args();
    configuration::parse_config(&flags);

    if flags.random.is_some() {
        let wallpapers = wallhaven::fetch_query(Sorting::Random)?;
        let chosen = utils::random_vec(&wallpapers);
        let set = files::set_wallpaper(&chosen).unwrap();
        println!("{set}");
        return Ok(());
    }

    if flags.toplist {
        let wallpapers = wallhaven::fetch_query(Sorting::Toplist)?;
        let chosen = utils::random_vec(&wallpapers);
        let set = files::set_wallpaper(&chosen).unwrap();
        println!("{set}");
        return Ok(());
    }

    if flags.hot {
        let wallpapers = wallhaven::fetch_query(Sorting::Hot)?;
        let chosen = utils::random_vec(&wallpapers);
        let set = files::set_wallpaper(&chosen).unwrap();
        println!("{set}");
        return Ok(());
    }

    Ok(())

    //if flags.clear {
    //    return;
    //}
    //
    //if flags.file {
    //    let curr = files::cache_to_vec(".current");
    //    println!("{}", &curr[0].to_string());
    //    return;
    //}
    //
    //if flags.url {
    //    let curr = files::cache_to_vec(".current");
    //    println!("{}", &curr[1].to_string());
    //    return;
    //}
    //
    //if flags.collection.is_some() {
    //    let chosen = wallhaven::choose_from_collection(flags.collection.unwrap().as_ref());
    //    let _ = files::set_wallpaper(&chosen);
    //    return;
    //}
    //
    //if flags.toplist {
    //    let wps = wallhaven::fetch_query(Sorting::Toplist);
    //    dbg!(wps);
    //    return;
    //}
    //
    //if flags.random.is_some() {
    //    let query = flags.random.unwrap();
    //    let chosen = wallhaven::fetch_random(&query);
    //    let _ = files::set_wallpaper(&chosen);
    //    return;
    //}
}
