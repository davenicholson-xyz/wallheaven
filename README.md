## 🌆 Wallheaven - Random wallpaper fetcher for wallhaven.cc

Wallheaven is a Rust powered CLI tool for fetching random wallpapers from [wallhaven.cc](https://wallhaven.cc/). Wallheaven will randomly choose a wallpaper from various lists including your own collections using your API key. The selected wallpaper will then be downloaded and will output the filepath. Alternatively if a script is provided then the script will be called with the filepath.

All queries and selected wallpapers are cached to cut down on API calls. 

--- 

### Installation

_coming soon_

---

### Configuration 

Configuration is stored in `$HOME/.config/wallheaven/config.toml` on linux/mac or `%appdata%/Roaming/wallheaven/config.toml` on Windows. Wallheaven's defaults will be overwritten by these options, which in turn will be overwritten by any environment variables set proceeded with `WALLHEAVEN_`. These can then be overridden with any command line flags.

An example `config.toml` is in this [here](https://github.com/davenicholson-xyz/wallheaven/blob/main/examples/config.toml)

---

### Usage

Call the `wallheaven` command and select an option to retrieve a random wallpaper.

`wallheaven -c collection_name` will select a wallpaper from your collection with that name. Include your username and API key in the congig/flags.

`wallheaven -r search_term` will return wallpapers from that query. 

`wallheaven -t` to get a random wallpaper from the toplist. Pass a range option to search time range `--range 1w` will search the toplist for the past week.

`wallheaven --hot` will get wallpapers from the hottest wallpapers list.

---

### Command line flags

#### `--config` [path]
Path to config file to use if not using default config path

#### `--username` [username]
wallhaven.cc username. Required to fetch from your collections

#### `--apikey` [apikey]
wallhaven.cc apikey to fetch NSFW images

#### `-c --collection` [name]
Name of collection to fetch random wallpaper from. Requires API key and username to be set

#### `-r --random`
Search query to select random wallpaper from

#### `-t --toplist`
Select a random image wallpaper from the most popular wallpapers in a time range

#### `--range [1d, 3d, 1w, 1M, 3M, 6M, 1y]` - Default `1M`
The time range to select toplist wallpaper from. Default to `1M`

#### `--hot`
Choose a random wallpaper from the most popular wallpapers right now!

#### `-p --purity [100,001,101 etc]` - Default `110`
Bits representing (sfw/sketchy/nsfw). Turn purities on(1) or off(0). NSFW requires a valid API key. Example: `-p 100` will only return SFW images. 

#### `--categories [100,001,101 etc]` - Default `111`
Bits representing categories to search from (general/anime/people). Turn categories on(1) or off (0). Example: `--categories 101` will not show anime images

#### `--pages[1 - 10]` - Default `3`
Amount of pages to search for random toplist/hot image. Higher number = more API calls so slower. 

#### `-e --expiry [number]` - Default `600`
Time in seconds to use cached search results. After this time the API will be searched for new wallpapers.

#### `-s --script [path]`
Path to external script to call with path to downloaded image. Use to set desktop wallpaper.

#### `-f --file`
Returns the current file path of the last wallpaper set with wallhwaven

#### `-u --url`
Return the wallhaven.cc link to the last wallpaper set with wallheaven

#### `-d --delete`
Deletes cache files

#### `-h --help`
Help file

#### `-V --version`
Version info

---

## Setting wallpaper with external script

_coming soon_

