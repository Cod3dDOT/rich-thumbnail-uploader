[![Rich Thumbnail Uploader](https://cod3d.dev/img/readme-rich_thumbnail_uploader.gif)](https://github.com/cod3ddot/rich-thumbnail-uploader)

[![License: AGPL v3](https://www.gnu.org/graphics/agplv3-155x51.png)](https://www.gnu.org/licenses/agpl-3.0)
[![Version](https://img.shields.io/github/v/tag/cod3ddot/rich-thumbnail-uploader?label=version&style=for-the-badge&logo=git&logoColor=white)](https://github.com/cod3ddot/rich-thumbnail-uploader/releases)
[![Language: Rust](https://img.shields.io/badge/Rust-orange.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)

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

Example: `C:\Users\user\rich-thumbnail-uploader.exe -s catbox -f webp`


## Options

```bash
Options:
    -d <DIMS>           Dimensions to resize the image to (maintains aspect ratio) [default: 256]
    -s <SERVICE>        Image hosting service to use [default: imgur] [possible values: imgur, catbox]
    -f <FORMAT>         Preffered image format [default: png] [possible values: png, webp]
    --uid <UID>         Optional uid (overrides provided client id for imgur / sets user hash for catbox)
    -h, --help          Print help
    -V, --version       Print version
```
## License

This project strives to be [REUSE](https://reuse.software/) compliant.

Generally:
- Documentation is licensed under CC-BY-NC-SA-4.0
- Code is licensed under AGPL-3.0-or-later
- Config files are under CC0-1.0

```
    rich-thumbnail-uploader: thumbnail uploader for foo_discord_rich
    Copyright (C) 2025  cod3ddot@proton.me

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
```
