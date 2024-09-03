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
    collection: Option<String>,

    /// Query to search for random wallpaper
    #[arg(short, long)]
    random: Option<String>,

    /// Purity (sfw|sketchy|nsfw) or bindary flags
    #[arg(short, long)]
    purity: Option<String>,
}

impl Flags {
    pub fn get(&self, field_name: &str) -> Option<&Option<String>> {
        match field_name {
            "apikey" => Some(&self.apikey),
            "collection" => Some(&self.collection),
            "random" => Some(&self.random),
            _ => None,
        }
    }
}

pub fn cli_args() -> Flags {
    let flags = Flags::parse();
    return flags;
}
