[![Rich Thumbnail Uploader](https://cod3d.dev/img/readme-rich_thumbnail_uploader.gif)](https://github.com/cod3ddot/rich-thumbnail-uploader)

# Rich Thumbnail Uploader

Thumbnail uploader for [`foo_discord_rich`](https://github.com/RemuSalminen/foo_discord_rich) — a plugin for Discord Rich Presence in foobar2000.

Uploads a thumbnail of album artwork to image hosting services and returns a link.

Inspired by [rust-imgur-upload](https://github.com/s0hv/rust-imgur-upload).

## Supported Services

- **Imgur** — converts to PNG
- **Catbox** — converts to WebP or PNG

## Quick Setup

Clone and build:
```bash
git clone https://github.com/your-username/rich-thumbnail-uploader
cd rich-thumbnail-uploader
cargo build --release
```

## Project Structure

```
├── src/
│   ├── models/             # models for api responses
│   │   ├── imgur.rs
│   │   └── mod.rs
│   ├── uploaders/          # upload logic per service
│   │   ├── imgur.rs
│   │   ├── catbox.rs
│   │   └── mod.rs
│   ├── cli.rs              # Argument parsing with clap
│   ├── config.rs           # Additional argument validation
│   ├── errors.rs           # Contains error types
│   ├── image_processor.rs  # Generates thumbnails
│   └── main.rs             # Entry point
├── Cargo.toml              # Crate metadata and dependencies
├── CHANGES.md              # Changelog
├── LICENSE                 # MIT
└── README.md               # This file
```


## Usage
1. Save executable on disk
2. File -> Preferences -> Discord Rich Presence Integration -> Advanced
3. Set upload command as the path to the executable, with any options you would like

Example: `C:\Users\user\apps\rich-thumbnail-uploader\rich-thumbnail-uploader.exe -s catbox -f webp`


## Options

```bash
Options:
    -d, --dims <DIMS>        Dimensions to resize the image to (maintains aspect ratio) [default: 256]
    -s, --service <SERVICE>  Image hosting service to use [default: imgur] [possible values: imgur, catbox]
    -o, --output <OUTPUT>    Output format for the response (URL only or JSON) [default: url] [possible values: url]
    -u, --uid <UID>          Optional uid (overrides provided client id for imgur / sets user hash for catbox)
    -f, --format <FORMAT>    Preffered image format [default: png] [possible values: png, webp]
    -h, --help               Print help
    -V, --version            Print version
```
## License

[MIT](./LICENSE)
