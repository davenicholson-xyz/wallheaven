use clap::Parser;

/// Wallpaper fetcher for wallhaven.cc
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Flags {
    /// Config file to use
    #[arg(long)]
    pub config: Option<String>,

    /// API key from wallhaven.cc
    #[arg(long)]
    pub apikey: Option<String>,

    /// Specify the collection name to fetch wallpaper from (requires username and apikey)
    #[arg(short, long, conflicts_with_all = ["random", "toplist", "hot"])]
    pub collection: Option<String>,

    /// Query to search for random wallpaper
    #[arg(short, long, conflicts_with_all = ["collection", "toplist", "hot"])]
    pub random: Option<String>,

    /// Select random image from toplist
    #[arg(short, long)]
    pub toplist: bool,

    /// Select random image from hot
    #[arg(long)]
    pub hot: bool,

    /// Set purity filter as bits (sfw|sketchy|nsfw) eg. 110 = sfw & sketchy
    #[arg(short, long, value_parser = check3bit)]
    pub purity: Option<String>,

    /// Set category filter as bits (general|anime|people) eg. 101 = general & people
    #[arg(long, alias = "cat", value_parser = check3bit)]
    pub categories: Option<String>,

    /// Pages to search for random image (higher = slower) max: 10
    #[arg(long, value_parser = clap::value_parser!(i64).range(1..11))]
    pub pages: Option<i64>,

    /// Time in seconds to use cache file before requery of wallhaven.cc
    #[arg(short, long)]
    pub expiry: Option<i64>,

    /// External script to pass selected file to (set wallpaper etc.)
    #[arg(short, long)]
    pub script: Option<String>,

    /// Returns the path to the current set wallpaper
    #[arg(short, long = "file")]
    pub file: bool,

    /// Returns the link to the current set wallpaper on wallhaven.cc
    #[arg(short, long = "url")]
    pub url: bool,

    /// Deletes wallpaper cache folder
    #[arg(short, long)]
    pub delete: bool,
}

pub fn cli_args() -> Flags {
    let flags = Flags::parse();
    return flags;
}

pub fn check3bit(bits: &str) -> Result<String, String> {
    if bits.len() == 3 && bits.chars().all(|c| c == '0' || c == '1') {
        return Ok(bits.to_string());
    } else {
        return Err(format!("should be bits e.g. 110"));
    }
}

pub fn max10(n: i32) -> Result<i32, String> {
    if n > 10 {
        return Err(format!("maximum of 10"));
    } else {
        return Ok(n);
    }
}
