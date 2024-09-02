use clap::Parser;

/// Wallpaper fetcher for wallhaven.cc
#[derive(Parser, Debug)]
pub struct Args {
    /// Config file to use
    #[arg(long)]
    pub config: Option<String>,

    /// Specify the collection name to fetch wallpaper from (requires username and apikey)
    #[arg(short, long)]
    collection: Option<String>,

    /// Query to search for random wallpaper
    #[arg(short, long)]
    random: Option<String>,
}

pub fn cli_args() -> Args {
    let cli = Args::parse();
    return cli;
}
