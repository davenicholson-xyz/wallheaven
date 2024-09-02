mod configuration;
mod files;
mod parseargs;
mod wallhaven;

fn main() {
    let args = parseargs::cli_args();
    dbg!(&args);
    let conf = configuration::parse_config(&args.config);
    println!("{:#?}", conf);
    let wid = wallhaven::fetch_collection_id(&args);
    println!("{:?}", wid);
}

//let url = "https://w.wallhaven.cc/full/vq/wallhaven-vq6ze3.jpg";
//let filename = files::filename_from_url(url);
//println!("filename: {}", filename);
//files::download_image(url);
//
