use reqwest::blocking::get;
use std::fs::File;
use std::io::copy;
use url::Url;

#[allow(dead_code)]
pub fn filename_from_url(url_str: &str) -> String {
    let url = Url::parse(url_str).unwrap();
    let path_segments = url.path_segments().unwrap();
    let filename = path_segments.last().unwrap();
    return format!("{}", filename);
}

#[allow(dead_code)]
pub fn download_image(image_url: &str) {
    let filename = filename_from_url(image_url);
    let mut response = get(image_url).expect("Failed to download image");
    let mut file = File::create(filename).expect("Failed to create file");
    copy(&mut response, &mut file).expect("Failed to save the file");
    println!("Image downloaded successfully...");
}
