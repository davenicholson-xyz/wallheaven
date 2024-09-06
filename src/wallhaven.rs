use std::fs;

use crate::enums::Sorting;
use crate::errors::CustomError;
use crate::files::{cache_to_vec, delete_if_older_than, vec_to_cache};
use crate::structs::{CollectionData, CollectionsData, PageData, Wallpaper};
use crate::{files, utils, SETTINGS};
use anyhow::{anyhow, Result};
use url::Url;

const API_URL: &str = "https://wallhaven.cc/api/v1";

pub fn query(sorting: Sorting) -> Result<Vec<String>> {
    if sorting.file().is_none() {
        return Ok(fetch_query(sorting)?);
    }

    let settings = SETTINGS.lock().unwrap();
    let maxage = settings.get("cache_age").unwrap().parse::<u64>().unwrap();
    std::mem::drop(settings);

    let mut query_cache = files::cache_dir_path().clone();
    query_cache.push(sorting.file().unwrap());

    delete_if_older_than(&query_cache, maxage)?;

    if query_cache.exists() {
        return Ok(cache_to_vec(&sorting.file().unwrap()));
    } else {
        return Ok(fetch_query(sorting)?);
    }
}

fn fetch_query(sorting: Sorting) -> Result<Vec<String>> {
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
        let q = settings.get("random").unwrap();
        url.query_pairs_mut().append_pair("q", q);
    }

    let mut wallpapers: Vec<String> = Vec::new();
    for p in 1..=settings.get("pages").unwrap().parse::<u32>().unwrap() {
        let u = url.clone();
        let wps = fetch_query_page(u, p)?;
        wallpapers.extend(wps);
    }

    if wallpapers.len() > 0 {
        let cache_file = sorting.file().unwrap_or(".random".to_string());
        let _ = files::vec_to_cache(&wallpapers, &cache_file);
        return Ok(wallpapers);
    } else {
        return Err(anyhow!(CustomError::new("No wallpapers found")));
    }
}

pub fn get_collection_id(label: &str) -> Result<u32> {
    let settings = SETTINGS.lock().unwrap();
    let maxage = settings.get("cache_age").unwrap().parse::<u64>().unwrap();
    std::mem::drop(settings);

    let mut collection_id_cache = files::cache_dir_path().clone();
    collection_id_cache.push(".collections");

    delete_if_older_than(&collection_id_cache, maxage)?;

    let collections: Vec<(String, u32)>;

    if collection_id_cache.exists() {
        let collections_list = cache_to_vec(".collections");
        collections = collection_to_tuple_pairs(collections_list)?;
    } else {
        let collections_list = fetch_collections_data()?;
        vec_to_cache(&collections_list, ".collections")?;
        collections = collection_to_tuple_pairs(collections_list)?;
    }

    let id = find_id_by_label(&collections, label)
        .ok_or(anyhow!(CustomError::new("Could not find collection id")))?;

    Ok(id)
}

fn collection_to_tuple_pairs(collections_list: Vec<String>) -> Result<Vec<(String, u32)>> {
    let mut collections: Vec<(String, u32)> = Vec::new();
    for collection in collections_list {
        let mut parts = collection.split("|");
        let label = parts.next().unwrap().to_string();
        let id = parts.next().unwrap().parse::<u32>()?;
        collections.push((label, id));
    }
    Ok(collections)
}

fn find_id_by_label(tuples: &Vec<(String, u32)>, label: &str) -> Option<u32> {
    for (tuple_label, id) in tuples {
        if tuple_label == label {
            return Some(id.clone());
        }
    }
    None
}

fn fetch_collections_data() -> Result<Vec<String>> {
    let settings = SETTINGS.lock().unwrap();

    let mut url = Url::parse(API_URL).unwrap();
    url.path_segments_mut().unwrap().push("collections");

    if let Some(apikey) = settings.get("apikey") {
        url.query_pairs_mut().append_pair("apikey", apikey);
    } else {
        return Err(anyhow!(CustomError::new(
            "API key required to find collection"
        )));
    }

    let response = fetch_json_string(&url.as_str())?;
    let collections_data = serde_json::from_str::<CollectionsData>(&response)?;

    let mut collections: Vec<String> = Vec::new();
    for collection in collections_data.data {
        collections.push(format!("{}|{}", collection.label, collection.id));
    }

    return Ok(collections);
}

fn fetch_json_string(url: &str) -> Result<String> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).send()?.text()?;
    Ok(response)
}

fn fetch_query_page(mut url: url::Url, page: u32) -> Result<Vec<String>> {
    url.query_pairs_mut().append_pair("page", &page.to_string());

    let response = fetch_json_string(&url.as_str())?;
    let page_data = serde_json::from_str::<PageData>(&response)?;

    let mut wallpapers: Vec<String> = Vec::new();
    for wallpaper in page_data.data {
        wallpapers.push(wallpaper.path);
    }

    Ok(wallpapers)
}
