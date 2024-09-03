use crate::SETTINGS;
use serde::Deserialize;
use std::collections::HashMap;

const API_URL: &str = "https://wallhaven.cc/api/v1";

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Collection {
    id: u32,
    label: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct CollectionData {
    data: Vec<Collection>,
}

impl CollectionData {
    fn find_collection_id(&self, search: &str) -> Option<u32> {
        self.data
            .iter()
            .find(|&collection| collection.label == search)
            .map(|collection| collection.id)
    }
}

pub fn fetch_collection_id() -> u32 {
    let settings = SETTINGS.lock().unwrap();
    let apikey = settings.get("apikey").expect("An API key is required");
    let collection_label = settings
        .get("collection")
        .expect("A collection name is required");
    let colletions = format!("{}/collections?apikey={}", API_URL, apikey);
    let collection_data =
        fetch_collection_data(colletions).expect("Error fetching users collection info");
    let collection_id = collection_data
        .find_collection_id(&collection_label)
        .expect("Collection not available");
    return collection_id;
}

#[tokio::main]
async fn fetch_collection_data(url: String) -> Result<CollectionData, reqwest::Error> {
    let collection_data: CollectionData =
        reqwest::Client::new().get(url).send().await?.json().await?;
    Ok(collection_data)
}

#[tokio::main]
pub async fn fetch_collection_info(id: u32) -> Result<(), reqwest::Error> {
    let settings = SETTINGS.lock().unwrap();

    let username = settings.get("username").expect("A username is required");
    let apikey = settings.get("apikey").expect("An API key is required");

    let url = format!(
        "{}/collections/{}/{}?apikey={}",
        API_URL, username, id, apikey
    );
    let collection_info = reqwest::Client::new().get(url).send().await?.text().await?;
    println!("{:#?}", collection_info);
    Ok(())
}
