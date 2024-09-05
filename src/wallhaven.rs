use std::fs;

use crate::enums::Sorting;
use crate::files::cache_to_vec;
use crate::structs::{CollectionData, CollectionsData, PageData, Wallpaper};
use crate::{files, utils, SETTINGS};
use url::Url;
use url_builder::URLBuilder;
use urlencoding::encode;

const API_URL: &str = "https://wallhaven.cc/api/v1";

pub fn query(sorting: Sorting) -> Result<Vec<String>, reqwest::Error> {
    print!("{}", sorting.file().unwrap());

    if sorting.file().is_none() {
        return Ok(fetch_query(sorting)?);
    }

    let settings = SETTINGS.lock().unwrap();
    let maxage = settings.get("cache_age").unwrap().parse::<u64>().unwrap();
    std::mem::drop(settings);

    let mut query_cache = files::cache_dir_path().clone();
    query_cache.push(sorting.file().unwrap());

    dbg!(&query_cache);

    if query_cache.exists() {
        println!("{} exists", sorting.file().unwrap());
        let metadata = fs::metadata(query_cache).unwrap();
        if let Ok(time) = metadata.modified() {
            if time.elapsed().unwrap().as_secs() > maxage {
                println!("using query");
                return Ok(fetch_query(sorting)?);
            } else {
                println!("using cache");
                return Ok(cache_to_vec(&sorting.file().unwrap()));
            }
        } else {
            println!("using cache");
            return Ok(cache_to_vec(&sorting.file().unwrap()));
        }
    } else {
        println!("using query");
        return Ok(fetch_query(sorting)?);
    }
}

pub fn fetch_query(sorting: Sorting) -> Result<Vec<String>, reqwest::Error> {
    let settings = SETTINGS.lock().unwrap();

    let mut url = Url::parse(API_URL).unwrap();
    url.path_segments_mut().unwrap().push("search");
    url.query_pairs_mut()
        .append_pair("purity", settings.get("purity").unwrap())
        .append_pair("categories", settings.get("categories").unwrap())
        .append_pair("seed", &utils::generate_seed())
        .append_pair("ratios", "landscape")
        .append_pair("sorting", sorting.param().as_ref());

    if let Some(apikey) = settings.get("apikey") {
        url.query_pairs_mut().append_pair("apikey", apikey);
    }

    if sorting == Sorting::Random {
        let q = encode(settings.get("random").unwrap());
        url.query_pairs_mut().append_pair("q", &q.into_owned());
    }

    let mut wallpapers: Vec<String> = Vec::new();
    for p in 1..=settings.get("pages").unwrap().parse::<u32>().unwrap() {
        let u = url.clone();
        let wps = fetch_query_page(u, p)?;
        wallpapers.extend(wps);
    }
    let cache_file = sorting.file().unwrap_or(".random".to_string());
    let _ = files::vec_to_cache(&wallpapers, &cache_file);
    Ok(wallpapers)
}

fn fetch_json_string(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).send()?.text()?;
    Ok(response)
}

fn fetch_query_page(mut url: url::Url, page: u32) -> Result<Vec<String>, reqwest::Error> {
    url.query_pairs_mut().append_pair("page", &page.to_string());

    let response = fetch_json_string(&url.as_str())?;
    let page_data = serde_json::from_str::<PageData>(&response).unwrap();

    let mut wallpapers: Vec<String> = Vec::new();
    for wallpaper in page_data.data {
        wallpapers.push(wallpaper.path);
    }

    Ok(wallpapers)
}
