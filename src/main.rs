mod configuration;
mod files;
mod parseargs;
mod wallhaven;

fn main() {
    let flags = parseargs::cli_args();
    dbg!(&flags);
    let conf = configuration::parse_config(&flags);
    println!("{:#?}", &conf);
    let wid = wallhaven::fetch_collection_id(&conf);
    println!("{:?}", wid);
}

//let url = "https://w.wallhaven.cc/full/vq/wallhaven-vq6ze3.jpg";
//let filename = files::filename_from_url(url);
//println!("filename: {}", filename);
//files::download_image(url);
//
