use crate::enums::Sorting;
use crate::structs::{CollectionData, CollectionsData, PageData};
use crate::{files, utils, SETTINGS};
use url_builder::URLBuilder;
use urlencoding::encode;

const API_URL: &str = "wallhaven.cc/api/v1";

pub fn fetch_query(sorting: Sorting) -> Result<Vec<String>, reqwest::Error> {
    let settings = SETTINGS.lock().unwrap();

    let mut url = URLBuilder::new();
    url.set_protocol("https")
        .set_host(API_URL)
        .add_route("search")
        .add_param("purity", settings.get("purity").unwrap())
        .add_param("categories", settings.get("categories").unwrap())
        .add_param("seed", &utils::generate_seed())
        .add_param("ratios", "landscape");

    if let Some(apikey) = settings.get("apikey") {
        url.add_param("apikey", apikey);
    }

    url.add_param("sorting", sorting.param().as_ref());

    if sorting == Sorting::Random {
        let q = encode(settings.get("random").unwrap());
        url.add_param("q", &q.into_owned());
    }

    let wallpapers = fetch_query_page(url, 1)?;
    let _ = files::vec_to_cache(&wallpapers, ".query");
    Ok(wallpapers)
}

fn fetch_json_string(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).send()?.text()?;
    Ok(response)
}

fn fetch_query_page(mut url: URLBuilder, page: u32) -> Result<Vec<String>, reqwest::Error> {
    url.add_param("page", &page.to_string());

    let response = fetch_json_string(&url.build())?;
    let page_data = serde_json::from_str::<PageData>(&response).unwrap();

    let mut wallpapers: Vec<String> = Vec::new();
    for wallpaper in page_data.data {
        wallpapers.push(wallpaper.path);
    }

    Ok(wallpapers)
}
