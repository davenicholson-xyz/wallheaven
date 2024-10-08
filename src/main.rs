mod config;
mod enums;
mod errors;
mod files;
mod flags;
mod structs;
mod utils;
mod wallhaven;

use anyhow::Result;
use enums::Sorting;
use std::fs;

fn main() -> Result<()> {
    let flags = flags::cli_args();

    let output = flags.output;

    if flags.collection.is_some() {
        let wallpapers = wallhaven::collection(&flags.collection.unwrap())?;
        let chosen = utils::random_vec(&wallpapers);
        files::set_wallpaper(&chosen, output)?;
        return Ok(());
    }

    if flags.random.is_some() {
        let wallpapers = wallhaven::query(Sorting::Random)?;
        let chosen = utils::random_vec(&wallpapers);
        files::set_wallpaper(&chosen, output)?;
        return Ok(());
    }

    if flags.toplist {
        let wallpapers = wallhaven::query(Sorting::Toplist)?;
        let chosen = utils::random_vec(&wallpapers);
        files::set_wallpaper(&chosen, output)?;
        return Ok(());
    }

    if flags.hot {
        let wallpapers = wallhaven::query(Sorting::Hot)?;
        let chosen = utils::random_vec(&wallpapers);
        files::set_wallpaper(&chosen, output)?;
        return Ok(());
    }

    if flags.id.is_some() {
        let wallpaper = wallhaven::by_id(&flags.id.unwrap())?;
        files::set_wallpaper(&wallpaper, output)?;
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

    if flags.last {
        let last = files::fetch_last_query()?;
        println!("{}", &last);
        return Ok(());
    }

    if flags.delete {
        let cache_path = files::cache_dir_path();
        fs::remove_dir_all(cache_path)?;
        return Ok(());
    }

    Ok(())
}
