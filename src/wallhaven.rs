use std::fs::read_to_string;

use crate::parseargs;
use crate::{files, SETTINGS};
use rand::seq::SliceRandom;
use reqwest::Error;
use serde::Deserialize;
use serde_json;

const API_URL: &str = "https://wallhaven.cc/api/v1";

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Collection {
    id: u32,
    label: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct CollectionsData {
    data: Vec<Collection>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Wallpaper {
    path: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct CollectonMeta {
    last_page: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct CollectionData {
    meta: CollectonMeta,
    data: Vec<Wallpaper>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct PageData {
    data: Vec<Wallpaper>,
}

impl CollectionsData {
    fn find_collection_id(&self, search: &str) -> Option<u32> {
        self.data
            .iter()
            .find(|&collection| collection.label == search)
            .map(|collection| collection.id)
    }
}

pub fn fetch_json_string(url: &str) -> Result<String, Error> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).send()?.text()?;
    Ok(response)
}

pub fn fetch_collection_id(label: &str) -> u32 {
    let settings = SETTINGS.lock().unwrap();
    let apikey = settings.get("apikey").expect("An API key is required");
    //let collection_label = settings
    //    .get("collection")
    //    .expect("A collection name is required");
    let colletions = format!("{}/collections?apikey={}", API_URL, apikey);
    let collections_data =
        fetch_collection_data(&colletions).expect("Error fetching users collection info");
    let collection_id = collections_data
        .find_collection_id(&label)
        .expect("Collection not available");
    return collection_id;
}

fn fetch_collection_data(url: &str) -> Result<CollectionsData, reqwest::Error> {
    let response = fetch_json_string(url);
    let collections_data = serde_json::from_str::<CollectionsData>(response.unwrap().as_ref());
    if collections_data.is_err() {
        panic!("{}", "Error with API. Check username and apikey");
    }
    Ok(collections_data.unwrap())
}

pub fn choose_from_collection(label: &str) -> String {
    let mut wallpapers: Vec<String> = Vec::new();
    let flags = parseargs::cli_args();

    let mut collection_cache = files::cache_dir_path().clone();
    collection_cache.push(label);

    if flags.from_cache && collection_cache.exists() {
        let v = files::cache_to_vec(label);
        wallpapers.extend(v);
    } else {
        let collection_id = fetch_collection_id(&label);
        let wp_list = fetch_collection(collection_id);
        wallpapers.extend(wp_list);
        if wallpapers.len() > 0 {
            _ = files::vec_to_cache(&wallpapers, label);
        }
    }

    let random_wallpaper = wallpapers.choose(&mut rand::thread_rng());
    random_wallpaper.unwrap().to_string()
}

pub fn fetch_collection(id: u32) -> Vec<String> {
    let settings = SETTINGS.lock().unwrap();

    let username = settings.get("username").expect("A username is required");
    let apikey = settings.get("apikey").expect("An API key is required");

    let url = format!(
        "{}/collections/{}/{}?apikey={}",
        API_URL, username, id, apikey
    );

    let response = fetch_json_string(&url);
    let collection_data =
        serde_json::from_str::<CollectionData>(response.unwrap().as_ref()).unwrap();

    let mut wallpapers: Vec<String> = Vec::new();
    for wallpaper in collection_data.data {
        wallpapers.push(wallpaper.path);
    }

    if collection_data.meta.last_page > 1 {
        for p in 2..=collection_data.meta.last_page {
            let wps = fetch_collection_page(id, p, username, apikey).unwrap();
            wallpapers.extend(wps);
        }
    }
    return wallpapers;
}

fn fetch_collection_page(
    id: u32,
    page: u32,
    username: &str,
    apikey: &str,
) -> Result<Vec<String>, reqwest::Error> {
    let url = format!(
        "{}/collections/{}/{}?page={}&apikey={}",
        API_URL, username, id, page, apikey
    );

    let mut wallpapers: Vec<String> = Vec::new();
    let response = fetch_json_string(&url);
    let page_data = serde_json::from_str::<PageData>(response.unwrap().as_ref()).unwrap();

    for wallpaper in page_data.data {
        wallpapers.push(wallpaper.path);
    }

    Ok(wallpapers)
}
