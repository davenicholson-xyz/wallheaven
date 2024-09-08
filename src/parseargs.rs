use clap::Parser;

/// Wallpaper fetcher for wallhaven.cc
#[derive(Parser, Debug)]
pub struct Flags {
    /// Config file to use
    #[arg(long)]
    pub config: Option<String>,

    /// API key
    #[arg(long)]
    pub apikey: Option<String>,

    /// Specify the collection name to fetch wallpaper from (requires username and apikey)
    #[arg(short, long, conflicts_with = "random")]
    pub collection: Option<String>,

    /// Query to search for random wallpaper
    #[arg(short, long)]
    pub random: Option<String>,

    /// Query to search for random wallpaper
    #[arg(short, long)]
    pub toplist: bool,

    /// Query to search for random wallpaper
    #[arg(long)]
    pub hot: bool,

    /// Purity (sfw|sketchy|nsfw) or binary flags
    #[arg(short, long)]
    pub purity: Option<String>,

    /// Categories to search from (general|anime|people) or binary flags
    #[arg(long, alias = "cat")]
    pub categories: Option<String>,

    /// Time in seconds to use cache file before requery of wallhaven.cc
    #[arg(short, long)]
    pub expiry: Option<i64>,

    /// Update collection from last search to minimise api calls
    #[arg(long = "from-cache")]
    pub from_cache: bool,

    /// Returns the path to the current set wallpaper
    #[arg(short, long = "file")]
    pub file: bool,

    /// Returns the link to the current set wallpaper on wallhaven.cc
    #[arg(short, long = "url")]
    pub url: bool,

    /// Deletes wallpaper cache files
    #[arg(long)]
    pub clear: bool,
}

//impl Flags {
//    pub fn get(&self, field_name: &str) -> Option<&Option<String>> {
//        match field_name {
//            "apikey" => Some(&self.apikey),
//            "collection" => Some(&self.collection),
//            "random" => Some(&self.random),
//            "purity" => Some(&self.purity),
//            "categories" => Some(&self.categories),
//            _ => None,
//        }
//    }
//}

pub fn cli_args() -> Flags {
    let flags = Flags::parse();
    return flags;
}
