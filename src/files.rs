use crate::config;
use std::fs::{self, File};
use std::io::{self, copy, BufRead, LineWriter, Write};
use std::path::Path;
use std::process::Command;
use std::{env, path::PathBuf};
use url::Url;

use anyhow::Result;

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

pub fn delete_if_older_than(file: &PathBuf, seconds: u64) -> Result<()> {
    if file.exists() {
        let metadata = fs::metadata(file)?;
        if let Ok(time) = metadata.modified() {
            if time.elapsed().unwrap().as_secs() > seconds {
                std::fs::remove_file(file)?
            }
        }
    }
    Ok(())
}

pub fn config_file_path() -> PathBuf {
    let mut conf_file = config_dir_path().clone();
    conf_file.push("config.toml");
    return conf_file;
}

pub fn config_file_string() -> String {
    return config_file_path().into_os_string().into_string().unwrap();
}

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

pub fn filename_from_url(url_str: &str) -> String {
    let url = Url::parse(url_str).unwrap();
    let path_segments = url.path_segments().unwrap();
    let filename = path_segments.last().unwrap();
    return format!("{}", filename);
}

pub fn download_image(image_url: &str, filename: &PathBuf) -> Result<()> {
    let client = reqwest::blocking::Client::builder()
        .use_rustls_tls()
        .build()?;

    let mut response = client.get(image_url).header("User-Agent", "Rust").send()?;
    let mut file = File::create(filename).expect("Failed to create file");
    copy(&mut response, &mut file).expect("Failed to save the file");
    Ok(())
}

pub fn check_or_create_dir(path: PathBuf) {
    if !(path.exists()) {
        _ = fs::create_dir_all(path);
    }
}

pub fn cache_last_query(query: &str) -> Result<()> {
    check_or_create_dir(cache_dir_path());
    let mut cache = cache_dir_path().clone();
    cache.push(".last_query");
    let file = File::create(cache)?;
    let mut file = LineWriter::new(file);
    file.write_all(query.as_bytes())?;
    Ok(())
}

pub fn fetch_last_query() -> Result<String> {
    let mut cache = cache_dir_path().clone();
    cache.push(".last_query");
    let file = File::open(cache)?;
    let mut reader = io::BufReader::new(file);
    let mut query = String::new();
    reader.read_line(&mut query)?;
    Ok(query.trim().to_string())
}

pub fn vec_to_cache(v: &Vec<String>, filename: &str) -> io::Result<()> {
    check_or_create_dir(cache_dir_path());
    let mut cache = cache_dir_path().clone();
    cache.push(filename);

    let mut file = File::create(cache)?;
    for line in v {
        writeln!(file, "{}", line)?
    }
    Ok(())
}

pub fn cache_to_vec(filename: &str) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    let mut cache = cache_dir_path().clone();
    cache.push(filename);

    for line in std::fs::read_to_string(cache).unwrap().lines() {
        v.push(line.to_string());
    }
    return v;
}

pub fn get_wpid(image_url: &str) -> String {
    let filename = filename_from_url(image_url);
    let path = Path::new(&filename).file_stem().unwrap();
    let wpname = path.to_os_string().into_string().unwrap();
    let wpid = wpname.split("-").collect::<Vec<&str>>();
    return wpid[1].to_string();
}

pub fn set_wallpaper(image_url: &str, output: bool) -> Result<()> {
    let config = config::CONFIG.lock().unwrap();

    let filename = filename_from_url(image_url);
    let mut fname = cache_dir_path().clone();
    fname.push(filename);

    if !(fname.exists()) {
        download_image(image_url, &fname)?;
    }

    let mut current_file = cache_dir_path().clone();
    current_file.push(".current");
    let mut current = File::create(current_file)?;
    writeln!(current, "{}", fname.display().to_string())?;
    let wpid = get_wpid(image_url);
    writeln!(current, "https://wallhaven.cc/w/{}", wpid)?;

    if output {
        println!("{}", fname.display().to_string());
        return Ok(());
    }

    let post_script = config.get_string("post_script");
    if post_script.is_ok() {
        let parsed_command =
            shlex::split(&post_script.unwrap()).expect("Failed to parse external script");
        if let Some((command, args)) = parsed_command.split_first() {
            let _status = Command::new(command)
                .args(args)
                .arg(fname.display().to_string())
                .status()
                .expect("Failed to execute external script");
        };
        return Ok(());
    } else {
        setwallpaper::from_file(&fname.display().to_string())?;
        return Ok(());
    }
}
