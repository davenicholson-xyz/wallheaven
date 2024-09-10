## Wallheaven - Random wallpaper fetcher from wallhaven.cc

Wallheaven is a Rust powered CLI tool for fetching random wallpapers from [wallhaven.cc](https://wallhaven.cc/). Wallheaven will randomly choose a wallpaper from various lists including your own collections using your api key.

### Installation

_install instructions here_

### Configuration 

Configuration is stored in `$HOME/.config/wallheaven/config.toml` on linux/mac or `%appdata%/Roaming/wallheaven/config.tml` on Windows. Wallheaven's defaults will be overwritten by these options, which in turn will be overwritten by any environment variables set preceeded with WALLHEAVEN_. These can then be overridden with any command line flags.

An example `config.toml` is in this [here](http://github.com)

### Usage

| Flag | Description | Example |
| --- | --- | --- |
| `--config` | Location of the config file to override default location ||
| `--username` | wallhaven username used for selecting from users collection ||
| `--apikey` | wallhaven api key used for accessing nsfw images ||
| `-c --collection` | Name of your collection to choose wallpaper from |  |
| `-r --random` | Search term for random wallpaper | |
| `-t --toplist` | Random image from popular wallpapers | |
| `--range` | Period to select toplist images from ||
| `--hot` | Random image from popular images right now | |
| `-p --purity` | Turn purity[sfw/sketchy/nsfw] on(1) or off(0) with bits. API key required for SNFW images | `-p 110` _sfw & sketchy_ |


