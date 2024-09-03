mod configuration;
mod files;
mod parseargs;
mod wallhaven;

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

fn main() {
    let flags = parseargs::cli_args();
    configuration::parse_config(&flags);
    let col_id = wallhaven::fetch_collection_id();
    let _ = wallhaven::fetch_collection_info(col_id);
}

//let url = "https://w.wallhaven.cc/full/vq/wallhaven-vq6ze3.jpg";
//let filename = files::filename_from_url(url);
//println!("filename: {}", filename);
//files::download_image(url);
//
