use reqwest::blocking::get;
use std::fs::{self, File};
use std::io::{self, copy, Write};
use std::{env, path::PathBuf};
use url::Url;

pub fn config_dir_path() -> PathBuf {
    if env::consts::OS == "windows" {
        let mut conf_dir = dirs::config_dir().unwrap();
        conf_dir.push("wallheaven");
        return conf_dir;
    } else {
        let mut conf_dir = dirs::home_dir().unwrap();
        conf_dir.push(".config/wallheaven");
        return conf_dir;
    }
}

pub fn config_file_path() -> PathBuf {
    let mut conf_file = config_dir_path().clone();
    conf_file.push("config.toml");
    return conf_file;
}

pub fn config_file_string() -> String {
    return config_file_path().into_os_string().into_string().unwrap();
}

#[allow(dead_code)]
pub fn cache_dir_path() -> PathBuf {
    if env::consts::OS == "windows" {
        let mut cache_dir = dirs::cache_dir().unwrap();
        cache_dir.push("wallheaven");
        return cache_dir;
    } else {
        let mut cache_dir = dirs::home_dir().unwrap();
        cache_dir.push(".cache/wallheaven");
        return cache_dir;
    }
}

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

pub fn check_or_create_dir(path: PathBuf) {
    if !(path.exists()) {
        _ = fs::create_dir_all(path);
    }
}

pub fn vec_to_cache(v: &Vec<String>, filename: &str) -> io::Result<()> {
    check_or_create_dir(cache_dir_path());
    let mut fname = cache_dir_path().clone();
    fname.push(filename);

    let mut file = File::create(fname)?;
    for line in v {
        writeln!(file, "{}", line)?
    }
    Ok(())
    //_ = std::fs::write(fname, "arses");
}
