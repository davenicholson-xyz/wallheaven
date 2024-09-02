use std::collections::HashMap;

const API_URL: &str = "https://wallhaven.cc/api/v1";

pub fn fetch_collection_id(conf: &HashMap<String, String>) -> String {
    let apikey = conf.get("apikey").expect("An API key is required");
    let url = format!("{}/collections?apikey={}", API_URL, apikey);
    return url;
}
