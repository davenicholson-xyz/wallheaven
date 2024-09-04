use crate::enums::Sorting;
use crate::errors::CustomError;
use crate::{utils, SETTINGS};

use url_builder::URLBuilder;
use urlencoding::encode;

const API_URL: &str = "wallhaven.cc/api/v1";

pub fn fetch_query(sorting: Sorting) -> Result<Vec<String>, CustomError> {
    let settings = SETTINGS.lock().unwrap();

    dbg!(&settings);

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

    dbg!(url.build());

    let wallpapers: Vec<String> = Vec::new();
    Ok(wallpapers)
}
