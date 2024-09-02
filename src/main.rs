mod configuration;
mod userpaths;

fn main() {
    let conf = configuration::parse_config();
    println!("{:#?}", conf);
}

//let url = "https://w.wallhaven.cc/full/vq/wallhaven-vq6ze3.jpg";
//let filename = files::filename_from_url(url);
//println!("filename: {}", filename);
//files::download_image(url);
//
